use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_core::bounded::BoundedVec;
use sp_std::{cmp::*, collections::btree_map::BTreeMap, fmt::Debug, prelude::*};

/// Defines the actions that can be applied to an Itemized storage
#[derive(Clone, Encode, Decode, PartialEq, Debug, TypeInfo, Eq, PartialOrd, Ord)]
pub enum ItemAction {
	Add { data: Vec<u8> },
	Remove { index: u16 },
}

/// This header is used to specify how long an item is inside the buffer and inserted into buffer
/// before every item
#[derive(Encode, Decode, PartialEq, MaxEncodedLen, Debug)]
pub struct ItemHeader {
	/// The length of this item, not including the size of this header.
	pub payload_len: u16,
}

#[derive(Debug, PartialEq)]
pub enum PageError {
	ErrorParsing(&'static str),
	InvalidAction(&'static str),
	ArithmeticOverflow,
	PageSizeOverflow,
}

/// A page of data
#[derive(Encode, Decode, TypeInfo, MaxEncodedLen, PartialEq, Debug, Default)]
#[scale_info(skip_type_params(PageDataSize))]
#[codec(mel_bound(PageDataSize: MaxEncodedLen))]
pub struct Page<PageDataSize: Get<u32>> {
	pub data: BoundedVec<u8, PageDataSize>,
}

/// an internal struct which contains the parsed items in a page
#[derive(Debug, PartialEq)]
pub struct ParsedItemPage<'a> {
	/// page current size
	pub page_size: usize,
	/// a map of item index to a slice of blob (header included)
	pub items: BTreeMap<u16, &'a [u8]>,
}

impl<PageDataSize: Get<u32>> Page<PageDataSize> {
	pub fn is_empty(&self) -> bool {
		self.data.is_empty()
	}
}

impl<PageDataSize: Get<u32>> From<BoundedVec<u8, PageDataSize>> for Page<PageDataSize> {
	fn from(bounded: BoundedVec<u8, PageDataSize>) -> Self {
		Self { data: bounded }
	}
}

impl<PageDataSize: Get<u32>> TryFrom<Vec<u8>> for Page<PageDataSize> {
	type Error = ();

	fn try_from(data: Vec<u8>) -> Result<Self, Self::Error> {
		let bounded: BoundedVec<u8, PageDataSize> = BoundedVec::try_from(data).map_err(|_| ())?;
		Ok(Page::from(bounded))
	}
}

impl<PageDataSize: Get<u32>> Page<PageDataSize> {
	/// applies all actions to specified page and returns the updated page
	pub fn apply_item_actions(&self, actions: &[ItemAction]) -> Result<Self, PageError> {
		let mut parsed = self.parse_as_itemized()?;

		let mut updated_page_buffer = Vec::with_capacity(parsed.page_size);
		let mut add_buffer = Vec::new();
		log::info!("parsed page len: {:?}", parsed.page_size);

		for action in actions {
			match action {
				ItemAction::Remove { index } => {
					ensure!(
						parsed.items.contains_key(&index),
						PageError::InvalidAction("item index is invalid")
					);
					parsed.items.remove(&index);
				},
				ItemAction::Add { data } => {
					let header = ItemHeader {
						payload_len: data
							.len()
							.try_into()
							.map_err(|_| PageError::InvalidAction("invalid payload size"))?,
					};
					add_buffer.extend_from_slice(&header.encode()[..]);
					add_buffer.extend_from_slice(&data[..]);
				},
			}
		}
		log::info!("add_buffer len: {:?}", add_buffer.len());

		// since BTreemap is sorted by key, all items will be kept in their old order
		for (_, slice) in parsed.items.iter() {
			updated_page_buffer.extend_from_slice(slice);
		}
		log::info!("rebuilt orig page buffer len: {:?}", updated_page_buffer.len());
		updated_page_buffer.append(&mut add_buffer);
		log::info!("new appended page buffer len: {:?}", updated_page_buffer.len());
		log::info!("page size limit: {:?}", PageDataSize::get());

		Page::<PageDataSize>::try_from(updated_page_buffer)
			.map_err(|_| PageError::InvalidAction("page size exceeded"))
	}

	/// Parses all the items inside an ItemPage
	pub fn parse_as_itemized(&self) -> Result<ParsedItemPage, PageError> {
		let mut count = 0u16;
		let mut items = BTreeMap::new();
		let mut offset = 0;
		while offset < self.data.len() {
			ensure!(
				offset + ItemHeader::max_encoded_len() <= self.data.len(),
				PageError::ErrorParsing("wrong header size")
			);
			let header = <ItemHeader>::decode(&mut &self.data[offset..])
				.map_err(|_| PageError::ErrorParsing("decoding header"))?;
			let item_total_length = ItemHeader::max_encoded_len() + header.payload_len as usize;
			ensure!(
				offset + item_total_length <= self.data.len(),
				PageError::ErrorParsing("wrong payload size")
			);

			items.insert(count, &self.data[offset..(offset + item_total_length)]);
			offset += item_total_length;
			count = count.checked_add(1).ok_or(PageError::ArithmeticOverflow)?;
		}

		Ok(ParsedItemPage { page_size: self.data.len(), items })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use frame_support::assert_ok;
	use pretty_assertions::assert_eq;

	type TestPageSize = ConstU32<2048>;
	type TestPage = Page<TestPageSize>;

	fn generate_payload_bytes(id: u8) -> Vec<u8> {
		format!("{{'type':{id}, 'description':'another test description {id}'}}")
			.as_bytes()
			.to_vec()
	}

	fn create_page_from(payloads: &[Vec<u8>]) -> TestPage {
		let mut buffer: Vec<u8> = vec![];
		for p in payloads {
			buffer.extend_from_slice(&ItemHeader { payload_len: p.len() as u16 }.encode()[..]);
			buffer.extend_from_slice(p);
		}
		TestPage::try_from(buffer).unwrap()
	}

	#[test]
	fn parsing_a_well_formed_item_page_should_work() {
		// arrange
		let payloads = vec![generate_payload_bytes(1), generate_payload_bytes(2)];
		let page = create_page_from(payloads.as_slice());

		// act
		let parsed = page.parse_as_itemized();

		// assert
		assert_ok!(&parsed);
		assert_eq!(
			parsed.as_ref().unwrap().page_size,
			payloads.len() * ItemHeader::max_encoded_len() +
				payloads.iter().map(|p| p.len()).sum::<usize>()
		);

		let items = parsed.unwrap().items;
		for index in 0..payloads.len() {
			assert_eq!(
				items.get(&(index as u16)).unwrap()[ItemHeader::max_encoded_len()..],
				payloads[index][..]
			);
		}
	}

	#[test]
	fn parsing_item_with_wrong_payload_size_should_return_parsing_error() {
		// arrange
		let payload = generate_payload_bytes(1);
		let mut buffer: Vec<u8> = vec![];
		buffer.extend_from_slice(
			&ItemHeader { payload_len: (payload.len() + 1) as u16 }.encode()[..],
		);
		buffer.extend_from_slice(&payload);
		let page: TestPage = Page::try_from(buffer).unwrap();

		// act
		let parsed = page.parse_as_itemized();

		// assert
		assert_eq!(parsed, Err(PageError::ErrorParsing("wrong payload size")));
	}

	#[test]
	fn parsing_wrong_item_header_size_page_should_return_parsing_error() {
		// arrange
		let payload = generate_payload_bytes(2);
		let mut buffer: Vec<u8> = vec![];
		buffer.extend_from_slice(
			&ItemHeader { payload_len: (payload.len() - 1) as u16 }.encode()[..],
		);
		buffer.extend_from_slice(&payload);
		let page = TestPage::try_from(buffer).unwrap();

		// act
		let parsed = page.parse_as_itemized();

		// assert
		assert_eq!(parsed, Err(PageError::ErrorParsing("wrong header size")));
	}

	#[test]
	fn applying_remove_action_with_existing_index_should_remove_item() {
		// arrange
		let payloads = vec![generate_payload_bytes(2), generate_payload_bytes(4)];
		let page = create_page_from(payloads.as_slice());
		let expecting_page = create_page_from(&payloads[1..]);
		let actions = vec![ItemAction::Remove { index: 0 }];

		// act
		let result = page.apply_item_actions(&actions);

		// assert
		assert_ok!(&result);
		let updated = result.unwrap();
		assert_eq!(expecting_page.data, updated.data);
	}

	#[test]
	fn applying_add_action_should_add_item_to_the_end_of_the_page() {
		// arrange
		let payload1 = vec![generate_payload_bytes(2)];
		let page = create_page_from(payload1.as_slice());
		let payload2 = vec![generate_payload_bytes(4)];
		let expecting_page = create_page_from(&vec![payload1[0].clone(), payload2[0].clone()][..]);
		let actions = vec![ItemAction::Add { data: payload2[0].clone() }];

		// act
		let result = page.apply_item_actions(&actions[..]);

		// assert
		assert_ok!(&result);
		let updated = result.unwrap();
		assert_eq!(expecting_page.data, updated.data);
	}

	#[test]
	fn applying_remove_action_with_non_existing_index_should_fail() {
		// arrange
		let payloads = vec![generate_payload_bytes(2), generate_payload_bytes(4)];
		let page = create_page_from(payloads.as_slice());
		let actions = vec![ItemAction::Remove { index: 2 }];

		// act
		let result = page.apply_item_actions(&actions[..]);

		// assert
		assert_eq!(result.is_err(), true);
	}

	#[test]
	fn applying_add_action_with_full_page_should_fail() {
		// arrange
		let mut arr: Vec<Vec<u8>> = vec![];
		let payload = generate_payload_bytes(2);
		while (arr.len() + 1) * (&payload.len() + ItemHeader::max_encoded_len()) <
			<TestPageSize as sp_core::Get<u32>>::get() as usize
		{
			arr.push(payload.clone());
		}
		let page = create_page_from(arr.as_slice());
		let actions = vec![ItemAction::Add { data: payload.clone() }];

		// act
		let result = page.apply_item_actions(&actions[..]);

		// assert
		assert_eq!(result.is_err(), true);
	}

	#[test]
	fn is_empty_false_for_non_empty_page() {
		let page: TestPage = vec![1].try_into().unwrap();

		assert_eq!(page.is_empty(), false);
	}

	#[test]
	fn is_empty_true_for_empty_page() {
		let page: TestPage = Vec::<u8>::new().try_into().unwrap();

		assert_eq!(page.is_empty(), true);
	}
}

use crate as pallet_messages;
use common_primitives::{
	msa::{
		MsaLookup,
		MsaValidator,
		ProviderLookup,
		DelegationValidator,
		SchemaGrantValidator,
		Delegator, MessageSourceId, OrderedSetExt, Provider, ProviderInfo},
	schema::*,
};
use frame_support::{
	dispatch::DispatchResult,
	parameter_types,
	traits::{ConstU16, ConstU64, OnFinalize, OnInitialize},
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	DispatchError,
};
use std::fmt::Formatter;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub const INVALID_SCHEMA_ID: SchemaId = 65534;
pub const IPFS_SCHEMA_ID: SchemaId = 65535;

pub const IPFS_PAYLOAD_LENGTH: u32 = 1200;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		MessagesPallet: pallet_messages::{Pallet, Call, Storage, Event<T>},
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MaxMessagesPerBlock: u32 = 500;
	pub const MaxMessagePayloadSizeBytes: u32 = 100;
	pub const MaxSchemaGrants: u32 = 30;
}

impl Clone for MaxSchemaGrants {
	fn clone(&self) -> Self {
		MaxSchemaGrants {}
	}
}

impl Eq for MaxSchemaGrants {
	fn assert_receiver_is_total_eq(&self) -> () {}
}

impl PartialEq for MaxSchemaGrants {
	fn eq(&self, _other: &Self) -> bool {
		true
	}
}

impl sp_std::fmt::Debug for MaxSchemaGrants {
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}

impl std::fmt::Debug for MaxMessagePayloadSizeBytes {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("MaxMessagePayloadSizeBytes")
			.field("v", &MaxMessagePayloadSizeBytes::get())
			.finish()
	}
}

impl PartialEq for MaxMessagePayloadSizeBytes {
	fn eq(&self, _other: &Self) -> bool {
		true
	}
}

impl Clone for MaxMessagePayloadSizeBytes {
	fn clone(&self) -> Self {
		MaxMessagePayloadSizeBytes {}
	}
}

pub struct MsaInfoHandler;
pub struct DelegationInfoHandler;
pub struct SchemaGrantValidationHandler;
impl MsaLookup for MsaInfoHandler {
	type AccountId = u64;

	fn get_msa_id(key: &Self::AccountId) -> Option<MessageSourceId> {
		if *key == 1000 {
			return None
		}
		if *key == 2000 {
			return Some(2000 as MessageSourceId)
		}
		Some(get_msa_from_account(*key) as MessageSourceId)
	}
}

impl MsaValidator for MsaInfoHandler {
	type AccountId = u64;

	fn ensure_valid_msa_key(key: &Self::AccountId) -> Result<MessageSourceId, DispatchError> {
		if *key == 1000 {
			return Err(DispatchError::Other("some error"))
		}
		if *key == 2000 {
			return Ok(2000)
		}

		Ok(get_msa_from_account(*key))
	}
}
impl ProviderLookup for DelegationInfoHandler {
	type BlockNumber = u64;
	type MaxSchemaGrants = MaxSchemaGrants;

	fn get_provider_info_of(
		_delegator: Delegator,
		provider: Provider,
	) -> Option<ProviderInfo<Self::BlockNumber, MaxSchemaGrants>> {
		if provider == Provider(2000) {
			return None
		};
		Some(ProviderInfo { expired: 100, schemas: OrderedSetExt::new() })
	}
}
impl DelegationValidator for DelegationInfoHandler {
	type BlockNumber = u64;
	type MaxSchemaGrants = MaxSchemaGrants;

	fn ensure_valid_delegation(
		provider: Provider,
		_delegator: Delegator,
		_block_number: Option<Self::BlockNumber>,
	) -> Result<ProviderInfo<Self::BlockNumber, Self::MaxSchemaGrants>, DispatchError> {
		if provider == Provider(2000) {
			return Err(DispatchError::Other("some delegation error"))
		};

		Ok(ProviderInfo { schemas: OrderedSetExt::new(), expired: Default::default() })
	}
}
impl SchemaGrantValidator for SchemaGrantValidationHandler {
	fn ensure_valid_schema_grant(
		provider: Provider,
		delegator: Delegator,
		_schema_id: SchemaId,
	) -> DispatchResult {
		match DelegationInfoHandler::get_provider_info_of(delegator, provider) {
			Some(_) => Ok(()),
			None => Err(DispatchError::Other("no schema grant or delegation")),
		}
	}
}

pub struct SchemaHandler;
impl SchemaProvider<u16> for SchemaHandler {
	fn get_schema_by_id(schema_id: SchemaId) -> Option<SchemaResponse> {
		if schema_id == INVALID_SCHEMA_ID {
			return None
		}
		if schema_id == IPFS_SCHEMA_ID {
			return Some(SchemaResponse {
				schema_id,
				model: r#"schema"#.to_string().as_bytes().to_vec(),
				model_type: ModelType::Parquet,
				payload_location: PayloadLocation::IPFS,
			})
		}

		Some(SchemaResponse {
			schema_id,
			model: r#"schema"#.to_string().as_bytes().to_vec(),
			model_type: ModelType::AvroBinary,
			payload_location: PayloadLocation::default(),
		})
	}
}

impl pallet_messages::Config for Test {
	type Event = Event;
	type MsaInfoProvider = MsaInfoHandler;
	type DelegationInfoProvider = DelegationInfoHandler;
	type SchemaGrantValidator = SchemaGrantValidationHandler;
	type SchemaProvider = SchemaHandler;
	type WeightInfo = ();
	type MaxMessagesPerBlock = MaxMessagesPerBlock;
	type MaxMessagePayloadSizeBytes = MaxMessagePayloadSizeBytes;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			MessagesPallet::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		MessagesPallet::on_initialize(System::block_number());
	}
}

pub fn get_msa_from_account(account_id: u64) -> u64 {
	account_id + 100
}

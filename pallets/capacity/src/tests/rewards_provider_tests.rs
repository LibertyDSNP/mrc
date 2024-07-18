use super::mock::*;
use crate::{
	Config, CurrentEraInfo, Error, ProviderBoostHistories, ProviderBoostHistory,
	ProviderBoostRewardClaim, ProviderBoostRewardsProvider, RewardEraInfo, StakingDetails,
	StakingType::*, UnclaimedRewardInfo,
};
use frame_support::{assert_err, assert_ok, traits::Len};

use crate::tests::testing_utils::{
	run_to_block, set_era_and_reward_pool, setup_provider, system_run_to_block,
};
use common_primitives::msa::MessageSourceId;
use sp_core::{Get, H256};

#[test]
fn staking_reward_total_happy_path() {
	new_test_ext().execute_with(|| {
		CurrentEraInfo::<Test>::set(RewardEraInfo { era_index: 11, started_at: 100 });
		// this calls the implementation in the pallet
		assert_eq!(Ok(5u64), Capacity::staking_reward_total(1, 5u32, 10u32));
		let proof = H256::random();
		let payload: ProviderBoostRewardClaim<Test> = ProviderBoostRewardClaim {
			claimed_reward: 1,
			staking_account_end_state: StakingDetails { active: 1, staking_type: MaximumCapacity },
			from_era: 1,
			to_era: 5,
		};
		assert!(Capacity::validate_staking_reward_claim(4, proof, payload));
	})
}

#[test]
fn payout_eligible_happy_path() {
	new_test_ext().execute_with(|| {
		let is_eligible = Capacity::payout_eligible(1);
		assert!(!is_eligible);
	})
}

#[test]
fn test_staking_reward_total_era_out_of_range() {
	new_test_ext().execute_with(|| {
		CurrentEraInfo::<Test>::set(RewardEraInfo { era_index: 10, started_at: 100 });
		assert_err!(Capacity::staking_reward_total(1, 5u32, 4u32), Error::<Test>::EraOutOfRange);
		assert_err!(Capacity::staking_reward_total(1, 15u32, 5u32), Error::<Test>::EraOutOfRange);
		assert_err!(Capacity::staking_reward_total(1, 5u32, 13u32), Error::<Test>::EraOutOfRange);
	});
}

// This tests Capacity implementation of the trait, but uses the mock's constants,
// to ensure that it's correctly specified in the pallet.
#[test]
fn era_staking_reward_implementation() {
	struct TestCase {
		total_staked: u64,    // by all boosters
		amount_staked: u64,   // by a single booster
		expected_reward: u64, // for a single era
		reward_pool: u64,     // reward pool size
	}
	let test_cases: Vec<TestCase> = vec![
		TestCase {
			total_staked: 1_000_000,
			amount_staked: 0,
			expected_reward: 0,
			reward_pool: 10_000,
		}, // shouldn't happen, but JIC
		TestCase {
			total_staked: 1_000_000,
			amount_staked: 30,
			expected_reward: 0,
			reward_pool: 10_000,
		}, // truncated result
		TestCase {
			total_staked: 1_000_000,
			amount_staked: 150,
			expected_reward: 1,
			reward_pool: 10_000,
		}, // truncated result
		TestCase {
			total_staked: 1_000_000,
			amount_staked: 1000,
			expected_reward: 4,
			reward_pool: 10_000,
		}, // hits the cap starting with this example
		TestCase {
			total_staked: 1_000_000,
			amount_staked: 11000,
			expected_reward: 42,
			reward_pool: 10_000,
		}, // > 0.0038% of total, reward = 11000*.0038
		TestCase {
			total_staked: 20_000_000,
			amount_staked: 888_889,
			expected_reward: 3378,
			reward_pool: 11_000_000,
		}, //  testnet/mainnet values
	];
	for tc in test_cases {
		assert_eq!(
			Capacity::era_staking_reward(tc.amount_staked, tc.total_staked, tc.reward_pool),
			tc.expected_reward
		);
	}
}

#[test]
fn check_for_unclaimed_rewards_returns_empty_set_when_no_staking() {
	new_test_ext().execute_with(|| {
		let account = 500u64;
		let history: ProviderBoostHistory<Test> = ProviderBoostHistory::new();
		ProviderBoostHistories::<Test>::set(account, Some(history));
		let rewards = Capacity::list_unclaimed_rewards(&account).unwrap();
		assert!(rewards.is_empty())
	})
}
#[test]
fn check_for_unclaimed_rewards_returns_empty_set_when_only_staked_this_era() {
	new_test_ext().execute_with(|| {
		system_run_to_block(5);
		set_era_and_reward_pool(5u32, 1u32, 1000u64);
		let account = 500u64;
		let mut history: ProviderBoostHistory<Test> = ProviderBoostHistory::new();
		history.add_era_balance(&5u32, &100u64);
		ProviderBoostHistories::<Test>::set(account, Some(history));
		let rewards = Capacity::list_unclaimed_rewards(&account).unwrap();
		assert!(rewards.is_empty())
	})
}

// Check that eligible amounts are only for what's staked an entire era.
#[test]
fn check_for_unclaimed_rewards_has_eligible_rewards() {
	new_test_ext().execute_with(|| {
		let account = 10_000u64;
		let target: MessageSourceId = 10;
		let amount = 1_000u64;

		// staking 1k as of block 1, era 1 (1-10)
		setup_provider(&account, &target, &amount, ProviderBoost);

		// staking 2k as of block 11, era 2  (11-20)
		run_to_block(11);
		assert_ok!(Capacity::provider_boost(RuntimeOrigin::signed(account), target, amount));

		//  staking 3k as of era 4, block 31, first block of era (31-40)
		run_to_block(31);
		assert_ok!(Capacity::provider_boost(RuntimeOrigin::signed(account), target, amount));

		run_to_block(51);
		assert_eq!(Capacity::get_current_era().era_index, 6u32);
		assert_eq!(Capacity::get_current_era().started_at, 51u32);

		// rewards for era 6 should not be returned; era 6 is current era and therefore ineligible.
		// eligible amounts for rewards for eras should be:  1=0, 2=1k, 3=2k, 4=2k, 5=3k
		let rewards = Capacity::list_unclaimed_rewards(&account).unwrap();
		assert_eq!(rewards.len(), 5usize);
		let expected_info: [UnclaimedRewardInfo<Test>; 5] = [
			UnclaimedRewardInfo {
				reward_era: 1u32,
				expires_at_block: 130,
				eligible_amount: 0,
				earned_amount: 0,
			},
			UnclaimedRewardInfo {
				reward_era: 2u32,
				expires_at_block: 140,
				eligible_amount: 1000,
				earned_amount: 4,
			},
			UnclaimedRewardInfo {
				reward_era: 3u32,
				expires_at_block: 150,
				eligible_amount: 2_000,
				earned_amount: 8,
			},
			UnclaimedRewardInfo {
				reward_era: 4u32,
				expires_at_block: 160,
				eligible_amount: 2000,
				earned_amount: 8,
			},
			UnclaimedRewardInfo {
				reward_era: 5u32,
				expires_at_block: 170,
				eligible_amount: 3_000,
				earned_amount: 11,
			},
		];
		for i in 0..5 {
			assert_eq!(rewards.get(i).unwrap(), &expected_info[i]);
		}

		run_to_block(131);
		let rewards = Capacity::list_unclaimed_rewards(&account).unwrap();
		let max_history: u32 = <Test as Config>::ProviderBoostHistoryLimit::get();
		// the earliest era should no longer be stored.
		assert_eq!(rewards.len(), max_history as usize);
		assert_eq!(rewards.get(0).unwrap().reward_era, 2u32);

		// there was no change in stake, so the eligible and earned amounts should be the same as in
		// reward era 5.
		assert_eq!(
			rewards.get((max_history - 1) as usize).unwrap(),
			&UnclaimedRewardInfo {
				reward_era: 13u32,
				expires_at_block: 250,
				eligible_amount: 3_000,
				earned_amount: 11,
			}
		)
	})
}

// check that if an account boosted and then let it run for more than the number
// of  history retention eras, eligible rewards are correct.
#[test]
fn check_for_unclaimed_rewards_returns_correctly_for_old_single_boost() {
	new_test_ext().execute_with(|| {
		let account = 10_000u64;
		let target: MessageSourceId = 10;
		let amount = 1_000u64;

		assert!(!Capacity::has_unclaimed_rewards(&account));

		// boost 1k as of block 1, era 1
		setup_provider(&account, &target, &amount, ProviderBoost);
		assert!(!Capacity::has_unclaimed_rewards(&account));

		run_to_block(131);
		assert_eq!(Capacity::get_current_era().era_index, 14u32);
		assert_eq!(Capacity::get_current_era().started_at, 131u32);

		let rewards = Capacity::list_unclaimed_rewards(&account).unwrap();

		let max_history: u32 = <Test as Config>::ProviderBoostHistoryLimit::get();
		assert_eq!(rewards.len(), max_history as usize);

		// the earliest era should no longer be stored.
		for i in 0u32..max_history {
			let era = i + 2u32;
			let expected_info: UnclaimedRewardInfo<Test> = UnclaimedRewardInfo {
				reward_era: era.into(),
				expires_at_block: (era * 10u32 + 120u32).into(),
				eligible_amount: 1000,
				earned_amount: 4,
			};
			assert_eq!(rewards.get(i as usize).unwrap(), &expected_info);
		}
	})
}

#[test]
fn has_unclaimed_rewards_works() {
	new_test_ext().execute_with(|| {
		let account = 10_000u64;
		let target: MessageSourceId = 10;
		let amount = 1_000u64;

		assert!(!Capacity::has_unclaimed_rewards(&account));

		// staking 1k as of block 1, era 1
		setup_provider(&account, &target, &amount, ProviderBoost);
		assert!(!Capacity::has_unclaimed_rewards(&account));

		// staking 2k as of block 11, era 2
		run_to_block(11);
		assert_ok!(Capacity::provider_boost(RuntimeOrigin::signed(account), target, amount));
		assert!(Capacity::has_unclaimed_rewards(&account));

		//  staking 3k as of era 4, block 31
		run_to_block(31);
		assert!(Capacity::has_unclaimed_rewards(&account));

		run_to_block(61);
		assert!(Capacity::has_unclaimed_rewards(&account));
	})
}

#[test]
fn has_unclaimed_rewards_returns_true_with_old_single_boost() {
	new_test_ext().execute_with(|| {
		let account = 10_000u64;
		let target: MessageSourceId = 10;
		let amount = 1_000u64;

		assert!(!Capacity::has_unclaimed_rewards(&account));

		// boost 1k as of block 1, era 1
		setup_provider(&account, &target, &amount, ProviderBoost);
		assert!(!Capacity::has_unclaimed_rewards(&account));

		run_to_block(71);
		assert!(Capacity::has_unclaimed_rewards(&account));
	});
}

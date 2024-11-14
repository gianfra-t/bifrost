// This file is part of Bifrost.

// Copyright (C) Liebi Technologies PTE. LTD.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// Ensure we're `no_std` when compiling for Wasm.

#![cfg(test)]

use crate::{
	mock::*, types::RedeemOrder, DispatchError::Module, Error as VtokenMintingError,
	Event as VtokenMintingEvent, *,
};
use bifrost_primitives::{
	currency::{BNC, FIL, KSM, MOVR, VBNC, VFIL, VKSM, VMOVR},
	VtokenMintingInterface, VtokenMintingOperator, DOT, GLMR, VDOT, VGLMR,
};
use frame_support::{
	assert_noop, assert_ok, sp_runtime::Permill, traits::fungibles::Unbalanced, BoundedVec,
};
use orml_traits::parameter_type_with_key;
use parity_scale_codec::Encode;
use sp_core::parameter_types;
use sp_runtime::ModuleError;

const V_DOT_TOTAL_ISSUANCE: u128 = 58_336_547_059_130_296;
const V_GLMR_TOTAL_ISSUANCE: u128 = 1_202_811_821_263_067_819_049_370;

fn init_test() {
	HookIterationLimit::<Runtime>::put(10);

	// Dot
	ConfigurationByCurrency::<Runtime>::insert(
		DOT,
		CurrencyConfiguration {
			mint_fee_rate: Permill::from_perthousand(0),
			redeem_fee_rate: Permill::from_perthousand(1),
			unlock_duration: TimeUnit::Era(28),
			min_mint_amount: 5_000_000_000u128,
			min_redeem_amount: 4_000_000_000u128,
			is_supported_restake: true,
			is_supported_fast_redeem: true,
		},
	);
	ActiveStateByCurrency::<Runtime>::insert(
		DOT,
		ActiveState {
			total_stake_amount: 82_780_231_171_068_554,
			total_unstake_amount: 4_630_200_353_096_381,
			total_restake_amount: 8_680_203_873_848_064,
			ongoing_time_unit: TimeUnit::Era(1622),
			next_redeem_id: 7943,
		},
	);
	Tokens::set_total_issuance(VDOT, V_DOT_TOTAL_ISSUANCE);
	// GLMR
	ConfigurationByCurrency::<Runtime>::insert(
		GLMR,
		CurrencyConfiguration {
			mint_fee_rate: Permill::from_perthousand(0),
			redeem_fee_rate: Permill::from_perthousand(1),
			unlock_duration: TimeUnit::Round(28),
			min_mint_amount: 2_500_000_000_000_000_000u128,
			min_redeem_amount: 2_000_000_000_000_000_000u128,
			is_supported_restake: true,
			is_supported_fast_redeem: true,
		},
	);
	ActiveStateByCurrency::<Runtime>::insert(
		GLMR,
		ActiveState {
			total_stake_amount: 1_399_452_284_017_371_216_780_475,
			total_unstake_amount: 197_873_382_472_729_666_844_723,
			total_restake_amount: 608_055_772_299_769_811_881_421,
			ongoing_time_unit: TimeUnit::Round(4129),
			next_redeem_id: 1816,
		},
	);
	Tokens::set_total_issuance(VGLMR, V_GLMR_TOTAL_ISSUANCE)
}

#[test]
fn mint_should_work() {
	ExtBuilder::default().setup_funds().build().execute_with(|| {
		init_test();
		let amount = 10_0_000_000_000;
		let v_amount = 7_0_471_592_352;

		let mut expect_active_state = ActiveStateByCurrency::<Runtime>::get(DOT).unwrap();
		expect_active_state.add_stake_amount(amount);

		assert_ok!(VtokenMinting::mint(
			RuntimeOrigin::signed(BOB),
			DOT,
			amount,
			BoundedVec::default(),
			None
		));

		expect_event(VtokenMintingEvent::Minted {
			minter: BOB,
			currency_id: DOT,
			currency_amount: amount,
			v_currency_amount: v_amount,
			mint_fee: 0,
			remark: Default::default(),
			channel_id: None,
		});

		let (entrance_account, _) = VtokenMinting::get_entrance_and_exit_accounts();
		assert_eq!(Tokens::free_balance(DOT, &entrance_account), amount);
		assert_eq!(Tokens::free_balance(VDOT, &BOB), 1000_0_000_000_000 + v_amount);
		assert_eq!(ActiveStateByCurrency::<Runtime>::get(DOT).unwrap(), expect_active_state);
	});
}

#[test]
fn redeem_should_work() {
	ExtBuilder::default().setup_funds().build().execute_with(|| {
		init_test();
		let amount = 9_9_900_000_000;
		let v_amount = 70_471_592_352;

		let mut expect_active_state = ActiveStateByCurrency::<Runtime>::get(DOT).unwrap();
		expect_active_state.subtract_stake_amount(amount);
		expect_active_state.add_unstake_amount(amount);
		expect_active_state.add_next_redeem_id();

		assert_ok!(VtokenMinting::redeem(RuntimeOrigin::signed(BOB), VDOT, v_amount));

		expect_event(VtokenMintingEvent::Redeemed { id: 7943 });

		let redeem_fee = ConfigurationByCurrency::<Runtime>::get(DOT)
			.unwrap()
			.redeem_fee_rate
			.mul_floor(v_amount);

		assert_eq!(Tokens::free_balance(VDOT, &BOB), 1000_0_000_000_000 - v_amount);
		assert_eq!(
			Tokens::free_balance(VDOT, &<Runtime as crate::Config>::FeeAccount::get()),
			redeem_fee
		);
		assert_eq!(ActiveStateByCurrency::<Runtime>::get(DOT).unwrap(), expect_active_state);
		assert_eq!(
			RedeemQueueByCurrency::<Runtime>::get(DOT).to_vec(),
			vec![RedeemOrder {
				id: 7943,
				creator: RedeemCreator::Substrate(BOB),
				currency: DOT,
				currency_amount: amount,
				remaining_currency_amount: amount,
				v_currency: VDOT,
				v_currency_amount: v_amount - redeem_fee,
				final_time_unit: TimeUnit::Era(1622 + 28),
				redeem_to: RedeemTo::Native(BOB),
			}]
		);
	});
}

#[test]
fn redeem_queue_should_work() {
	ExtBuilder::default().setup_funds().build().execute_with(|| {
		init_test();
		let amount = 14_1_759_248_890;
		let v_amount = 10_0_000_000_000;
		assert_ok!(VtokenMinting::redeem(RuntimeOrigin::signed(BOB), VDOT, v_amount));
		assert_ok!(VtokenMinting::redeem(RuntimeOrigin::signed(BOB), VDOT, v_amount));
		assert_ok!(VtokenMinting::redeem(RuntimeOrigin::signed(BOB), VDOT, v_amount));

		let redeem_fee = ConfigurationByCurrency::<Runtime>::get(DOT)
			.unwrap()
			.redeem_fee_rate
			.mul_floor(v_amount);
		let mut expect_redeem_queue = vec![
			RedeemOrder {
				id: 7943,
				creator: RedeemCreator::Substrate(BOB),
				currency: DOT,
				currency_amount: amount,
				remaining_currency_amount: amount,
				v_currency: VDOT,
				v_currency_amount: v_amount - redeem_fee,
				final_time_unit: TimeUnit::Era(1622 + 28),
				redeem_to: RedeemTo::Native(BOB),
			},
			RedeemOrder {
				id: 7944,
				creator: RedeemCreator::Substrate(BOB),
				currency: DOT,
				currency_amount: amount,
				remaining_currency_amount: amount,
				v_currency: VDOT,
				v_currency_amount: v_amount - redeem_fee,
				final_time_unit: TimeUnit::Era(1622 + 28),
				redeem_to: RedeemTo::Native(BOB),
			},
			RedeemOrder {
				id: 7945,
				creator: RedeemCreator::Substrate(BOB),
				currency: DOT,
				currency_amount: amount,
				remaining_currency_amount: amount,
				v_currency: VDOT,
				v_currency_amount: v_amount - redeem_fee,
				final_time_unit: TimeUnit::Era(1622 + 28),
				redeem_to: RedeemTo::Native(BOB),
			},
		];
		assert_eq!(RedeemQueueByCurrency::<Runtime>::get(DOT).to_vec(), expect_redeem_queue);

		run_to_block(2);
		assert_eq!(RedeemQueueByCurrency::<Runtime>::get(DOT).to_vec(), expect_redeem_queue);

		let (entrance_account, _) = VtokenMinting::get_entrance_and_exit_accounts();
		let entrance_balance = 10_000_000_000;
		assert_ok!(Tokens::set_balance(
			RuntimeOrigin::root(),
			entrance_account.clone(),
			DOT,
			entrance_balance,
			0
		));
		run_to_block(3);
		assert_eq!(Tokens::free_balance(DOT, &entrance_account), 0);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 1000_0_000_000_000 + 10_000_000_000);
		expect_redeem_queue[0].subtract_remaining_currency_amount(entrance_balance);
		assert_eq!(RedeemQueueByCurrency::<Runtime>::get(DOT).to_vec(), expect_redeem_queue);
		println!("expect_redeem_queue: {:?}", expect_redeem_queue);

		let entrance_balance = 20_0_000_000_000;
		assert_ok!(Tokens::set_balance(
			RuntimeOrigin::root(),
			entrance_account.clone(),
			DOT,
			entrance_balance,
			0
		));
		run_to_block(4);
		assert_eq!(Tokens::free_balance(DOT, &entrance_account), 0);
		assert_eq!(
			Tokens::free_balance(DOT, &BOB),
			1000_0_000_000_000 + 10_000_000_000 + 20_0_000_000_000
		);
		let remaining_currency_amount = expect_redeem_queue.remove(0).remaining_currency_amount();
		expect_redeem_queue[0]
			.subtract_remaining_currency_amount(entrance_balance - remaining_currency_amount);
		assert_eq!(RedeemQueueByCurrency::<Runtime>::get(DOT).to_vec(), expect_redeem_queue);
		println!("expect_redeem_queue: {:?}", expect_redeem_queue);

		let entrance_balance = 30_0_000_000_000;
		assert_ok!(Tokens::set_balance(
			RuntimeOrigin::root(),
			entrance_account.clone(),
			DOT,
			entrance_balance,
			0
		));
		run_to_block(5);
		assert_eq!(
			Tokens::free_balance(DOT, &entrance_account),
			30_0_000_000_000 - 73518497780 - 141759248890
		);
		assert_eq!(
			Tokens::free_balance(DOT, &BOB),
			1000_0_000_000_000 + 10_000_000_000 + 20_0_000_000_000 + 73518497780 + 141759248890
		);
		expect_redeem_queue.remove(0);
		expect_redeem_queue.remove(0);
		assert_eq!(RedeemQueueByCurrency::<Runtime>::get(DOT).to_vec(), expect_redeem_queue);
		println!("expect_redeem_queue: {:?}", expect_redeem_queue);
	});
}

#[test]
fn rebond_should_work() {
	ExtBuilder::default().setup_funds().build().execute_with(|| {
		init_test();
		let amount = 14_1_759_248_890;
		let v_amount = 10_0_000_000_000;
		assert_ok!(VtokenMinting::redeem(RuntimeOrigin::signed(BOB), VDOT, v_amount));
		assert_ok!(VtokenMinting::redeem(RuntimeOrigin::signed(BOB), VDOT, v_amount));
		assert_ok!(VtokenMinting::redeem(RuntimeOrigin::signed(BOB), VDOT, v_amount));

		let redeem_fee = ConfigurationByCurrency::<Runtime>::get(DOT)
			.unwrap()
			.redeem_fee_rate
			.mul_floor(v_amount);

		let mut expect_redeem_queue = vec![
			RedeemOrder {
				id: 7943,
				creator: RedeemCreator::Substrate(BOB),
				currency: DOT,
				currency_amount: amount,
				remaining_currency_amount: amount,
				v_currency: VDOT,
				v_currency_amount: v_amount - redeem_fee,
				final_time_unit: TimeUnit::Era(1622 + 28),
				redeem_to: RedeemTo::Native(BOB),
			},
			RedeemOrder {
				id: 7944,
				creator: RedeemCreator::Substrate(BOB),
				currency: DOT,
				currency_amount: amount,
				remaining_currency_amount: amount,
				v_currency: VDOT,
				v_currency_amount: v_amount - redeem_fee,
				final_time_unit: TimeUnit::Era(1622 + 28),
				redeem_to: RedeemTo::Native(BOB),
			},
			RedeemOrder {
				id: 7945,
				creator: RedeemCreator::Substrate(BOB),
				currency: DOT,
				currency_amount: amount,
				remaining_currency_amount: amount,
				v_currency: VDOT,
				v_currency_amount: v_amount - redeem_fee,
				final_time_unit: TimeUnit::Era(1622 + 28),
				redeem_to: RedeemTo::Native(BOB),
			},
		];
		assert_eq!(RedeemQueueByCurrency::<Runtime>::get(DOT).to_vec(), expect_redeem_queue);

		assert_ok!(VtokenMinting::rebond_by_unlock_id(RuntimeOrigin::signed(BOB), DOT, 7943));
		expect_redeem_queue.remove(0);
		let expect_receive_amount1 =
			VtokenMinting::get_v_currency_amount_by_currency_amount(DOT, VDOT, amount).unwrap();
		assert_eq!(
			Tokens::free_balance(VDOT, &BOB),
			1000_0_000_000_000 - v_amount * 3 + expect_receive_amount1
		);
		assert_eq!(RedeemQueueByCurrency::<Runtime>::get(DOT).to_vec(), expect_redeem_queue);

		let (entrance_account, _) = VtokenMinting::get_entrance_and_exit_accounts();
		let entrance_balance = 10_000_000_000;
		assert_ok!(Tokens::set_balance(
			RuntimeOrigin::root(),
			entrance_account.clone(),
			DOT,
			entrance_balance,
			0
		));

		run_to_block(2);
		assert_eq!(Tokens::free_balance(DOT, &entrance_account), 0);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 1000_0_000_000_000 + entrance_balance);
		expect_redeem_queue[0].subtract_remaining_currency_amount(entrance_balance);
		assert_eq!(RedeemQueueByCurrency::<Runtime>::get(DOT).to_vec(), expect_redeem_queue);

		assert_ok!(VtokenMinting::rebond_by_unlock_id(RuntimeOrigin::signed(BOB), DOT, 7944));
		let expect_receive_amount2 = VtokenMinting::get_v_currency_amount_by_currency_amount(
			DOT,
			VDOT,
			expect_redeem_queue.remove(0).remaining_currency_amount,
		)
		.unwrap();
		assert_eq!(
			Tokens::free_balance(VDOT, &BOB),
			1000_0_000_000_000 - v_amount * 3 + expect_receive_amount1 + expect_receive_amount2
		);
		assert_eq!(RedeemQueueByCurrency::<Runtime>::get(DOT).to_vec(), expect_redeem_queue);
	});
}

// #[test]
// fn set_ongoing_time_unit_should_work() {
// 	ExtBuilder::default().setup_funds().build().execute_with(|| {
// 		env_logger::try_init().unwrap_or(());
//
// 		// set KSM ongoing time unit to be Era(1)
// 		OngoingTimeUnit::<Runtime>::insert(KSM, TimeUnit::Era(1));
// 		assert_eq!(OngoingTimeUnit::<Runtime>::get(KSM), Some(TimeUnit::Era(1)));
//
// 		// set_ongoing_time_unit the ongoing time unit of KSM to be Round(2)
// 		assert_ok!(VtokenMinting::set_ongoing_time_unit(
// 			RuntimeOrigin::signed(ALICE),
// 			KSM,
// 			TimeUnit::Round(2)
// 		));
// 		assert_eq!(OngoingTimeUnit::<Runtime>::get(KSM), Some(TimeUnit::Round(2)));
// 	})
// }
//
#[test]
fn mint_with_lock_should_work() {
	ExtBuilder::default().setup_funds().build().execute_with(|| {
		env_logger::try_init().unwrap_or(());

		pub const FEE: Permill = Permill::from_percent(5);
		assert_ok!(VtokenMinting::set_currency_config(RuntimeOrigin::root(), KSM, FEE, FEE, TimeUnit::Era(28), 0, 0, true, true));

		// mint exceeds bob's KSM balance
		assert_noop!(
			VtokenMinting::mint_with_lock(
				Some(BOB).into(),
				KSM,
				10000000000000,
				BoundedVec::default(),
				None
			),
			Error::<Runtime>::NotEnoughBalance
		);

		// Minimum Mint not set
		assert_noop!(
			VtokenMinting::mint_with_lock(Some(BOB).into(), KSM, 100, BoundedVec::default(), None),
			Error::<Runtime>::NotSupportTokenType
		);

		// Set minimum mint
		assert_ok!(VtokenMinting::set_currency_config(RuntimeOrigin::root(), KSM, FEE, FEE, TimeUnit::Era(28), 100, 0, true, true));

		// vtoken coefficient not set
		assert_noop!(
			VtokenMinting::mint_with_lock(Some(BOB).into(), KSM, 100, BoundedVec::default(), None),
			Error::<Runtime>::IncentiveCoefNotFound
		);

		// set vtoken coefficient
		assert_ok!(VtokenMinting::set_incentive_coef(RuntimeOrigin::signed(ALICE), VKSM, Some(1)));

		// pool not enough vKSM balance
		assert_noop!(
			VtokenMinting::mint_with_lock(Some(BOB).into(), KSM, 100, BoundedVec::default(), None),
			Error::<Runtime>::NotEnoughBalance
		);

		// set incentive pool balance
		assert_ok!(Tokens::deposit(
			VKSM,
			&VtokenMinting::incentive_pool_account(),
			100000000000000000000
		));

		// incentive lock blocks not set
		assert_noop!(
			VtokenMinting::mint_with_lock(Some(BOB).into(), KSM, 100, BoundedVec::default(), None),
			Error::<Runtime>::IncentiveLockBlocksNotSet
		);

		// set incentive lock blocks
		assert_ok!(VtokenMinting::set_vtoken_incentive_lock_blocks(
			RuntimeOrigin::signed(ALICE),
			VKSM,
			Some(100)
		));

		let bob_old_balance = Tokens::free_balance(VKSM, &BOB);
		// mint with lock
		assert_ok!(VtokenMinting::mint_with_lock(
			Some(BOB).into(),
			KSM,
			100000000000,
			BoundedVec::default(),
			None
		));

		// check the vksm balance of bob. Should be minted_amount + incentive amount + original
		// balance
		assert_eq!(Tokens::free_balance(VKSM, &BOB), 95000000000 + 9499999990 + bob_old_balance);

		// check the pool balance, should have been transferred 9499999990 to Bob account
		assert_eq!(
			Tokens::free_balance(VKSM, &VtokenMinting::incentive_pool_account()),
			100000000000000000000 - 9499999990
		);

		// check ledger
		let lock_ledger = VtokenLockLedger::<Runtime>::get(BOB, VKSM).unwrap();
		let list = BoundedVec::try_from(vec![(95000000000u128, 100u64)]).unwrap();
		let should_be_ledger = (95000000000u128, list);
		assert_eq!(lock_ledger, should_be_ledger);
	})
}

// #[test]
// fn unlock_incentive_minted_vtoken_should_work() {
// 	ExtBuilder::default().setup_funds().build().execute_with(|| {
// 		env_logger::try_init().unwrap_or(());
//
// 		pub const FEE: Permill = Permill::from_percent(5);
// 		assert_ok!(VtokenMinting::set_fees(RuntimeOrigin::root(), FEE, FEE));
// 		// Set minimum mint
// 		assert_ok!(VtokenMinting::set_minimum_mint(RuntimeOrigin::signed(ALICE), KSM, 100));
// 		// set vtoken coefficient
// 		assert_ok!(VtokenMinting::set_incentive_coef(RuntimeOrigin::signed(ALICE), VKSM, Some(1)));
// 		// set incentive pool balance
// 		assert_ok!(Tokens::deposit(
// 			VKSM,
// 			&VtokenMinting::incentive_pool_account(),
// 			100000000000000000000
// 		));
// 		// set incentive lock blocks
// 		assert_ok!(VtokenMinting::set_vtoken_incentive_lock_blocks(
// 			RuntimeOrigin::signed(ALICE),
// 			VKSM,
// 			Some(100)
// 		));
// 		// mint with lock
// 		assert_ok!(VtokenMinting::mint_with_lock(
// 			Some(BOB).into(),
// 			KSM,
// 			100000000000,
// 			BoundedVec::default(),
// 			None
// 		));
//
// 		run_to_block(101);
//
// 		// check ledger
// 		let lock_ledger = VtokenLockLedger::<Runtime>::get(BOB, VKSM).unwrap();
// 		let list = BoundedVec::try_from(vec![(95000000000u128, 100u64)]).unwrap();
// 		let should_be_ledger = (95000000000u128, list);
// 		assert_eq!(lock_ledger, should_be_ledger);
//
// 		let bob_vksm_balance = Tokens::free_balance(VKSM, &BOB);
// 		// Bob's account cannot withdraw the locked vksm
// 		assert_eq!(
// 			<Runtime as crate::Config>::MultiCurrency::ensure_can_withdraw(
// 				VKSM,
// 				&BOB,
// 				bob_vksm_balance
// 			),
// 			Err(Module(ModuleError {
// 				index: 1,
// 				error: [2, 0, 0, 0,],
// 				message: Some("LiquidityRestrictions",),
// 			},),)
// 		);
//
// 		// unlock incentive minted vtoken
// 		assert_ok!(VtokenMinting::unlock_incentive_minted_vtoken(RuntimeOrigin::signed(BOB), VKSM));
//
// 		// Bob's amount can withdraw the locked vksm now
// 		assert_ok!(<Runtime as crate::Config>::MultiCurrency::ensure_can_withdraw(
// 			VKSM,
// 			&BOB,
// 			bob_vksm_balance
// 		));
//
// 		// total amount should be remain the same
// 		let new_bob_vksm_balance = Tokens::free_balance(VKSM, &BOB);
// 		assert_eq!(new_bob_vksm_balance, bob_vksm_balance);
//
// 		// check ledger
// 		let lock_ledger = VtokenLockLedger::<Runtime>::get(BOB, VKSM);
// 		assert_eq!(lock_ledger, None);
// 	})
// }
//
// #[test]
// fn set_incentive_coef_should_work() {
// 	ExtBuilder::default().setup_funds().build().execute_with(|| {
// 		env_logger::try_init().unwrap_or(());
//
// 		// get vksm coefficient should return None
// 		assert_eq!(VtokenIncentiveCoef::<Runtime>::get(VKSM), None);
//
// 		// set vksm coefficient
// 		assert_ok!(VtokenMinting::set_incentive_coef(RuntimeOrigin::signed(ALICE), VKSM, Some(1)));
//
// 		// get vksm coefficient should return Some(1)
// 		assert_eq!(VtokenIncentiveCoef::<Runtime>::get(VKSM), Some(1));
//
// 		// set vksm coefficient to None
// 		assert_ok!(VtokenMinting::set_incentive_coef(RuntimeOrigin::signed(ALICE), VKSM, None));
//
// 		// get vksm coefficient should return None
// 		assert_eq!(VtokenIncentiveCoef::<Runtime>::get(VKSM), None);
// 	})
// }
//
// #[test]
// fn set_vtoken_incentive_lock_blocks_should_work() {
// 	ExtBuilder::default().setup_funds().build().execute_with(|| {
// 		env_logger::try_init().unwrap_or(());
//
// 		// get vksm lock blocks should return None
// 		assert_eq!(MintWithLockBlocks::<Runtime>::get(VKSM), None);
//
// 		// set vksm lock blocks
// 		assert_ok!(VtokenMinting::set_vtoken_incentive_lock_blocks(
// 			RuntimeOrigin::signed(ALICE),
// 			VKSM,
// 			Some(100)
// 		));
//
// 		// get vksm lock blocks should return Some(100)
// 		assert_eq!(MintWithLockBlocks::<Runtime>::get(VKSM), Some(100));
//
// 		// set vksm lock blocks to None
// 		assert_ok!(VtokenMinting::set_vtoken_incentive_lock_blocks(
// 			RuntimeOrigin::signed(ALICE),
// 			VKSM,
// 			None
// 		));
//
// 		// get vksm lock blocks should return None
// 		assert_eq!(MintWithLockBlocks::<Runtime>::get(VKSM), None);
// 	})
// }

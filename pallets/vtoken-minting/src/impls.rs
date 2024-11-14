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

use crate::{
	types::{ActiveState, CurrencyConfiguration, RedeemOrder, RedeemQueue, INCENTIVE_LOCK_ID},
	AccountIdOf, ActiveStateByCurrency, BalanceOf, Config, ConfigurationByCurrency, CurrencyIdOf,
	Error, Event, HookIterationLimit, MintWithLockBlocks, OnRedeemSuccess, Pallet, RedeemOrderId,
	RedeemQueueByCurrency, TimeUnitUnlockLedger, TokenPool, TokenUnlockLedger, UnlockingTotal,
	UserUnlockLedger, VtokenIncentiveCoef, VtokenLockLedger, WeightInfo,
};
use bb_bnc::traits::BbBNCInterface;
use bifrost_primitives::{
	currency::BNC, AstarChainId, CurrencyId, CurrencyIdExt, HydrationChainId, InterlayChainId,
	MantaChainId, RedeemCreator, RedeemTo, RedeemType, SlpxOperator, TimeUnit,
	VTokenMintRedeemProvider, VTokenSupplyProvider, VtokenMintingInterface, VtokenMintingOperator,
	FIL,
};
use frame_support::{
	pallet_prelude::{DispatchResultWithPostInfo, *},
	sp_runtime::{
		traits::{AccountIdConversion, CheckedAdd, CheckedSub, UniqueSaturatedInto, Zero},
		DispatchError, FixedU128, Permill, SaturatedConversion,
	},
	traits::LockIdentifier,
	transactional, BoundedVec,
};
use frame_system::pallet_prelude::*;
use orml_traits::{MultiCurrency, MultiLockableCurrency, XcmTransfer};
use sp_core::{H160, U256};
use sp_runtime::{
	helpers_128bit::multiply_by_rational_with_rounding,
	traits::{BlakeTwo256, Hash},
	Rounding,
};
use sp_std::{vec, vec::Vec};
use xcm::{prelude::*, v4::Location};

#[derive(Encode, Decode, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Operation {
	Set,
	Add,
	Sub,
}

impl<T: Config> Pallet<T> {
	pub fn update_config(
		currency_id: CurrencyId,
		config: CurrencyConfiguration<BalanceOf<T>>,
	) -> DispatchResult {
		ConfigurationByCurrency::<T>::insert(currency_id, config);
		Ok(())
	}

	pub fn update_active_state(
		currency_id: &CurrencyId,
		active_state: ActiveState<BalanceOf<T>>,
	) -> DispatchResult {
		ActiveStateByCurrency::<T>::insert(currency_id, active_state);
		Ok(())
	}

	pub fn update_redeem_queue(
		currency_id: &CurrencyId,
		redeem_queue: &RedeemQueue<T>,
	) -> DispatchResult {
		RedeemQueueByCurrency::<T>::insert(currency_id, redeem_queue);
		Ok(())
	}

	/// Transfer to by redeem type.
	/// Parameters:
	/// - `redeemer`: The redeemer account id.
	/// - `redeem_currency_id`: The redeem currency id.
	/// - `redeem_currency_amount`: The redeem currency amount.
	/// - `entrance_account_balance`: The entrance account balance.
	/// - `redeem_type`: The redeem type.
	/// Returns:
	/// - `(BalanceOf<T>, RedeemTo<T::AccountId>)`: The redeem currency amount, redeem to.
	pub fn transfer_to_by_redeem_to(
		currency_id: CurrencyId,
		currency_amount: BalanceOf<T>,
		redeem_to: RedeemTo<T::AccountId>,
		entrance_account_balance: BalanceOf<T>,
	) -> DispatchResult {
		let entrance_account = T::EntranceAccount::get().into_account_truncating();
		let ed = T::MultiCurrency::minimum_balance(currency_id);

		if let RedeemTo::Native(account_id) = redeem_to {
			let currency_amount = currency_amount.min(entrance_account_balance);
			if currency_amount >= ed {
				T::MultiCurrency::transfer(
					currency_id,
					&entrance_account,
					&account_id,
					currency_amount,
				)?;
			}
			return Ok(());
		}

		if entrance_account_balance >= currency_amount {
			let dest = match redeem_to {
				RedeemTo::Astar(receiver) => Location::new(
					1,
					[
						Parachain(AstarChainId::get()),
						AccountId32 { network: None, id: receiver.encode().try_into().unwrap() },
					],
				),
				RedeemTo::Hydradx(receiver) => Location::new(
					1,
					[
						Parachain(HydrationChainId::get()),
						AccountId32 { network: None, id: receiver.encode().try_into().unwrap() },
					],
				),

				RedeemTo::Interlay(receiver) => Location::new(
					1,
					[
						Parachain(InterlayChainId::get()),
						AccountId32 { network: None, id: receiver.encode().try_into().unwrap() },
					],
				),

				RedeemTo::Manta(receiver) => Location::new(
					1,
					[
						Parachain(MantaChainId::get()),
						AccountId32 { network: None, id: receiver.encode().try_into().unwrap() },
					],
				),

				RedeemTo::Moonbeam(receiver) => Location::new(
					1,
					[
						Parachain(T::MoonbeamChainId::get()),
						AccountKey20 { network: None, key: receiver.to_fixed_bytes() },
					],
				),
				RedeemTo::Native(_) => {
					unreachable!()
				},
			};
			if currency_id == FIL {
				let assets = vec![
					(currency_id, currency_amount),
					(BNC, T::BifrostSlpx::get_moonbeam_transfer_to_fee()),
				];

				T::XcmTransfer::transfer_multicurrencies(
					entrance_account.clone(),
					assets,
					1,
					dest,
					Unlimited,
				)?;
			} else {
				T::XcmTransfer::transfer(
					entrance_account.clone(),
					currency_id,
					currency_amount,
					dest,
					Unlimited,
				)?;
			};
		}
		Ok(())
	}

	#[transactional]
	pub fn handle_ledger_by_currency(currency_id: CurrencyId) -> DispatchResult {
		let mut redeem_queue = RedeemQueueByCurrency::<T>::get(currency_id);
		let length = redeem_queue.len();
		if length == 0 {
			return Ok(());
		}

		let hook_iter_limit = HookIterationLimit::<T>::get() as usize;
		let to_process = if length > hook_iter_limit { hook_iter_limit } else { length };
		for _ in 0..to_process {
			let entrance_account_balance = T::MultiCurrency::free_balance(
				currency_id,
				&T::EntranceAccount::get().into_account_truncating(),
			);
			if entrance_account_balance == BalanceOf::<T>::zero() {
				return Ok(());
			}

			let redeem_creator = match redeem_queue[0].clone().creator {
				RedeemCreator::Substrate(account_id) => account_id,
				RedeemCreator::Ethereum(address) => Self::h160_to_account_id(&address),
			};
			let redeem_to = redeem_queue[0].redeem_to.clone();
			let redeem_id = redeem_queue[0].id;

			let mut final_currency_amount = redeem_queue[0].remaining_currency_amount();
			if final_currency_amount <= entrance_account_balance {
				redeem_queue.remove(0);
			} else {
				if redeem_queue[0].is_native_redeem() {
					redeem_queue[0].subtract_remaining_currency_amount(entrance_account_balance);
					final_currency_amount = entrance_account_balance;
				}
			}
			let mut active_state =
				ActiveStateByCurrency::<T>::get(currency_id).ok_or(Error::<T>::NotEnoughBalance)?;
			active_state.subtract_stake_amount(final_currency_amount);

			Self::update_redeem_queue(&currency_id, &redeem_queue)?;
			Self::update_active_state(&currency_id, active_state)?;

			T::OnRedeemSuccess::on_redeem_success(
				currency_id,
				redeem_creator.clone(),
				final_currency_amount,
			);

			Self::transfer_to_by_redeem_to(
				currency_id,
				final_currency_amount,
				redeem_to.clone(),
				entrance_account_balance,
			)?;
			Self::deposit_event(Event::RedeemSuccess {
				redeemer: redeem_creator,
				unlock_id: redeem_id,
				currency_id,
				to: redeem_to,
				currency_amount: final_currency_amount,
			});
		}
		Ok(())
	}

	pub fn do_mint(
		minter: AccountIdOf<T>,
		currency_id: CurrencyIdOf<T>,
		currency_amount: BalanceOf<T>,
		remark: BoundedVec<u8, ConstU32<32>>,
		channel_id: Option<u32>,
	) -> Result<BalanceOf<T>, DispatchError> {
		let config = ConfigurationByCurrency::<T>::get(currency_id)
			.ok_or(Error::<T>::ConfigurationNotFound)?;

		ensure!(currency_amount >= config.min_mint_amount(), Error::<T>::BelowMinimumMint);
		let v_currency_id = currency_id.to_vtoken().map_err(|_| Error::<T>::NotSupportTokenType)?;

		let mint_fee = config.mint_fee_rate().mul_floor(currency_amount);
		// Charging fees
		T::MultiCurrency::transfer(currency_id, &minter, &T::FeeAccount::get(), mint_fee)?;

		let currency_amount_exclude_fee =
			currency_amount.checked_sub(&mint_fee).ok_or(Error::<T>::CalculationOverflow)?;
		let v_currency_amount = Self::get_v_currency_amount_by_currency_amount(
			currency_id,
			v_currency_id,
			currency_amount_exclude_fee,
		)?;
		// Issue the corresponding v_currency to the user's account.
		T::MultiCurrency::deposit(v_currency_id, &minter, v_currency_amount)?;

		// Increase the token pool amount.
		let mut active_state = ActiveStateByCurrency::<T>::get(currency_id)
			.ok_or(Error::<T>::UnlockDurationNotFound)?;
		active_state.add_stake_amount(currency_amount_exclude_fee);
		Self::update_active_state(&currency_id, active_state)?;

		// Transfer the user's token to EntranceAccount.
		T::MultiCurrency::transfer(
			currency_id,
			&minter,
			&T::EntranceAccount::get().into_account_truncating(),
			currency_amount_exclude_fee,
		)?;

		// record the minting information for ChannelCommission module
		T::ChannelCommission::record_mint_amount(channel_id, v_currency_id, v_currency_amount)?;

		Self::deposit_event(Event::Minted {
			minter,
			currency_id,
			currency_amount,
			v_currency_amount,
			mint_fee,
			remark,
			channel_id,
		});
		Ok(v_currency_amount)
	}

	pub fn h160_to_account_id(address: &H160) -> T::AccountId {
		let mut data = [0u8; 24];
		data[0..4].copy_from_slice(b"evm:");
		data[4..24].copy_from_slice(&address[..]);
		let hash = BlakeTwo256::hash(&data);

		let account_id_32 = sp_runtime::AccountId32::from(Into::<[u8; 32]>::into(hash));
		T::AccountId::decode(&mut account_id_32.as_ref()).expect("Fail to decode address")
	}

	pub fn do_redeem(
		redeem_creator: RedeemCreator<T::AccountId>,
		v_currency_id: CurrencyId,
		v_currency_amount: BalanceOf<T>,
		redeem_to: RedeemTo<T::AccountId>,
	) -> DispatchResult {
		let currency_id = v_currency_id.to_token().map_err(|_| Error::<T>::NotSupportTokenType)?;
		let config = ConfigurationByCurrency::<T>::get(currency_id)
			.ok_or(Error::<T>::NotSupportTokenType)?;
		ensure!(v_currency_amount >= config.min_redeem_amount(), Error::<T>::BelowMinimumRedeem);

		let redeemer = match redeem_creator.clone() {
			RedeemCreator::Substrate(account_id) => account_id,
			RedeemCreator::Ethereum(address) => Self::h160_to_account_id(&address),
		};

		// Charging fees
		let redeem_fee = config.redeem_fee_rate().mul_floor(v_currency_amount);
		T::MultiCurrency::transfer(
			v_currency_id,
			&redeemer,
			&T::RedeemFeeAccount::get(),
			redeem_fee,
		)?;

		// Calculate the currency amount by v_currency_amount
		let v_currency_amount = v_currency_amount
			.checked_sub(&redeem_fee)
			.ok_or(Error::<T>::CalculationOverflow)?;
		let currency_amount = Self::get_currency_amount_by_v_currency_amount(
			currency_id,
			v_currency_id,
			v_currency_amount,
		)?;

		// Withdraw the token from redeemer
		T::MultiCurrency::withdraw(v_currency_id, &redeemer, v_currency_amount)?;

		// Calculate final_time_unit
		let mut active_state =
			ActiveStateByCurrency::<T>::get(currency_id).ok_or(Error::<T>::NotEnoughBalance)?;
		let final_time_unit = active_state
			.ongoing_time_unit()
			.add(config.unlock_duration())
			.ok_or(Error::<T>::CalculationOverflow)?;

		let mut redeem_queue = RedeemQueueByCurrency::<T>::get(currency_id);
		let id = active_state.next_redeem_id();
		redeem_queue
			.try_push(RedeemOrder {
				id,
				creator: redeem_creator,
				currency: currency_id,
				currency_amount,
				remaining_currency_amount: currency_amount,
				v_currency: v_currency_id,
				v_currency_amount,
				final_time_unit,
				redeem_to,
			})
			.map_err(|_| Error::<T>::TooManyRedeems)?;

		// Decrease the token pool amount
		active_state.subtract_stake_amount(currency_amount);
		active_state.add_unstake_amount(currency_amount);
		active_state.add_next_redeem_id();
		Self::update_active_state(&currency_id, active_state)?;
		Self::update_redeem_queue(&currency_id, &redeem_queue)?;

		T::ChannelCommission::record_redeem_amount(v_currency_id, v_currency_amount)?;
		T::OnRedeemSuccess::on_redeemed(
			redeemer,
			currency_id,
			currency_amount,
			v_currency_amount,
			redeem_fee,
		);
		Self::deposit_event(Event::Redeemed { id });
		Ok(())
	}

	pub fn do_rebond_by_redeem_id(
		rebonder: T::AccountId,
		currency_id: CurrencyId,
		redeem_id: RedeemOrderId,
	) -> DispatchResult {
		let config =
			ConfigurationByCurrency::<T>::get(currency_id).ok_or(Error::<T>::Unexpected)?;
		ensure!(config.is_supported_restake(), Error::<T>::NotSupportTokenType);

		let mut redeem_queue = RedeemQueueByCurrency::<T>::get(currency_id);
		let index = redeem_queue
			.iter()
			.position(|x| x.id == redeem_id)
			.ok_or(Error::<T>::Unexpected)?;

		let redeem_order = redeem_queue.remove(index);
		ensure!(
			redeem_order.creator == RedeemCreator::Substrate(rebonder.clone()),
			Error::<T>::Unexpected
		);

		let v_currency_amount = Self::get_v_currency_amount_by_currency_amount(
			redeem_order.currency(),
			redeem_order.v_currency(),
			redeem_order.remaining_currency_amount(),
		)?;

		// Issue the corresponding v_currency to the user's account.
		T::MultiCurrency::deposit(redeem_order.v_currency(), &rebonder, v_currency_amount)?;

		let mut active_state =
			ActiveStateByCurrency::<T>::get(currency_id).ok_or(Error::<T>::Unexpected)?;
		active_state.subtract_unstake_amount(redeem_order.remaining_currency_amount());
		active_state.add_restake_amount(redeem_order.remaining_currency_amount());
		active_state.add_stake_amount(redeem_order.remaining_currency_amount());
		Self::update_active_state(&currency_id, active_state)?;
		Self::update_redeem_queue(&currency_id, &redeem_queue)?;

		Self::deposit_event(Event::RebondedByUnlockId {
			rebonder,
			currency_id,
			currency_amount: redeem_order.remaining_currency_amount(),
			v_currency_amount,
			fee: Default::default(),
			unlock_id: redeem_id,
		});
		Ok(())
	}

	pub fn redeem_queue_by_account(
		currency_id: CurrencyId,
		account: T::AccountId,
	) -> Vec<RedeemOrder<T::AccountId, BalanceOf<T>>> {
		RedeemQueueByCurrency::<T>::get(currency_id)
			.into_iter()
			.filter(|x| x.creator == RedeemCreator::Substrate(account.clone()))
			.collect()
	}

	pub fn incentive_pool_account() -> AccountIdOf<T> {
		T::IncentivePoolAccount::get().into_account_truncating()
	}

	// to lock user vtoken for incentive minting
	pub fn lock_vtoken_for_incentive_minting(
		minter: AccountIdOf<T>,
		v_currency_id: CurrencyIdOf<T>,
		v_currency_amount: BalanceOf<T>,
	) -> Result<(), Error<T>> {
		// first, lock the vtoken
		// second, record the lock in ledger

		// check whether the minter has enough vtoken
		T::MultiCurrency::ensure_can_withdraw(v_currency_id, &minter, v_currency_amount)
			.map_err(|_| Error::<T>::NotEnoughBalance)?;

		// new amount that should be locked
		let mut new_lock_total = v_currency_amount;

		// check the previous locked amount under the same v_currency_id from ledger
		// and revise ledger to set the new_amount to be previous_amount + v_currency_amount
		VtokenLockLedger::<T>::mutate_exists(
			&minter,
			&v_currency_id,
			|v_token_lock_ledger| -> Result<(), Error<T>> {
				// get the vtoken lock duration from VtokenIncentiveCoef
				let lock_duration = MintWithLockBlocks::<T>::get(v_currency_id)
					.ok_or(Error::<T>::IncentiveLockBlocksNotSet)?;
				let current_block = frame_system::Pallet::<T>::block_number();
				let due_block = current_block
					.checked_add(&lock_duration)
					.ok_or(Error::<T>::CalculationOverflow)?;

				match v_token_lock_ledger {
					Some((total_locked, lock_records)) => {
						// check the total locked amount
						new_lock_total = total_locked
							.checked_add(&v_currency_amount)
							.ok_or(Error::<T>::CalculationOverflow)?;

						*total_locked = new_lock_total;

						// push new item to the boundedvec of the ledger
						lock_records
							.try_push((v_currency_amount, due_block))
							.map_err(|_| Error::<T>::TooManyLocks)?;
					},
					None =>
						*v_token_lock_ledger = Some((
							v_currency_amount,
							BoundedVec::try_from(vec![(v_currency_amount, due_block)])
								.map_err(|_| Error::<T>::TooManyLocks)?,
						)),
				}

				// extend the locked amount to be new_lock_total
				T::MultiCurrency::set_lock(
					INCENTIVE_LOCK_ID,
					v_currency_id,
					&minter,
					new_lock_total,
				)
				.map_err(|_| Error::<T>::NotEnoughBalance)
			},
		)
	}

	pub fn calculate_incentive_vtoken_amount(
		minter: &AccountIdOf<T>,
		v_currency_id: CurrencyIdOf<T>,
		v_currency_amount: BalanceOf<T>,
	) -> Result<BalanceOf<T>, Error<T>> {
		// get the vtoken pool balance
		let vtoken_pool_balance =
			T::MultiCurrency::free_balance(v_currency_id, &Self::incentive_pool_account());
		ensure!(vtoken_pool_balance > BalanceOf::<T>::zero(), Error::<T>::NotEnoughBalance);

		// get current block number
		let current_block_number: BlockNumberFor<T> = frame_system::Pallet::<T>::block_number();
		// get the veBNC total amount
		let vebnc_total_issuance = T::BbBNC::total_supply(current_block_number)
			.map_err(|_| Error::<T>::VeBNCCheckingError)?;
		ensure!(vebnc_total_issuance > BalanceOf::<T>::zero(), Error::<T>::BalanceZero);

		// get the veBNC balance of the minter
		let minter_vebnc_balance =
			T::BbBNC::balance_of(minter, None).map_err(|_| Error::<T>::VeBNCCheckingError)?;
		ensure!(minter_vebnc_balance > BalanceOf::<T>::zero(), Error::<T>::NotEnoughBalance);

		// get the percentage of the veBNC balance of the minter to the total veBNC amount and
		// get the square root of the percentage
		let percentage = Permill::from_rational(minter_vebnc_balance, vebnc_total_issuance);
		let sqrt_percentage =
			FixedU128::from_inner(percentage * 1_000_000_000_000_000_000u128).sqrt();
		let percentage = Permill::from_rational(
			sqrt_percentage.into_inner(),
			1_000_000_000_000_000_000u128.into(),
		);
		// get the total issuance of the vtoken
		let v_currency_total_issuance = T::MultiCurrency::total_issuance(v_currency_id);

		// get the incentive coef for the vtoken
		let incentive_coef = VtokenIncentiveCoef::<T>::get(v_currency_id)
			.ok_or(Error::<T>::IncentiveCoefNotFound)?;

		// calculate the incentive amount, but mind the overflow
		// incentive_amount = vtoken_pool_balance * incentive_coef * v_currency_amount *
		// sqrt_percentage / v_currency_total_issuance
		let incentive_amount =
			U256::from(percentage.mul_ceil(vtoken_pool_balance).saturated_into::<u128>())
				.checked_mul(U256::from(incentive_coef))
				.and_then(|x| x.checked_mul(U256::from(v_currency_amount.saturated_into::<u128>())))
				// .and_then(|x| x.checked_mul(percentage))
				.and_then(|x| {
					x.checked_div(U256::from(v_currency_total_issuance.saturated_into::<u128>()))
				})
				// first turn into u128，then use unique_saturated_into BalanceOf<T>
				.map(|x| x.saturated_into::<u128>())
				.map(|x| x.unique_saturated_into())
				.ok_or(Error::<T>::CalculationOverflow)?;

		Ok(incentive_amount)
	}
}

impl<T: Config> VtokenMintingOperator<CurrencyId, BalanceOf<T>, AccountIdOf<T>, TimeUnit>
	for Pallet<T>
{
	fn total_stake_amount(currency_id: CurrencyId) -> BalanceOf<T> {
		ActiveStateByCurrency::<T>::get(currency_id)
			.map(|active_state| active_state.total_stake_amount())
			.unwrap_or_default()
	}

	fn increase_token_pool(
		currency_id: CurrencyId,
		currency_amount: BalanceOf<T>,
	) -> DispatchResult {
		ActiveStateByCurrency::<T>::mutate(currency_id, |active_state| -> DispatchResult {
			let active_state = active_state.as_mut().ok_or(Error::<T>::UnlockDurationNotFound)?;
			active_state.add_stake_amount(currency_amount);
			Ok(())
		})
	}

	fn decrease_token_pool(
		currency_id: CurrencyId,
		currency_amount: BalanceOf<T>,
	) -> DispatchResult {
		ActiveStateByCurrency::<T>::mutate(currency_id, |active_state| -> DispatchResult {
			let active_state = active_state.as_mut().ok_or(Error::<T>::UnlockDurationNotFound)?;
			active_state.subtract_stake_amount(currency_amount);
			Ok(())
		})
	}

	fn update_ongoing_time_unit(currency_id: CurrencyId, time_unit: TimeUnit) -> DispatchResult {
		ActiveStateByCurrency::<T>::mutate(currency_id, |active_state| -> DispatchResult {
			let active_state = active_state.as_mut().ok_or(Error::<T>::UnlockDurationNotFound)?;
			active_state.set_ongoing_time_unit(time_unit);
			Ok(())
		})
	}

	fn get_ongoing_time_unit(currency_id: CurrencyId) -> Option<TimeUnit> {
		ActiveStateByCurrency::<T>::get(currency_id)
			.map(|active_state| active_state.ongoing_time_unit())
	}

	fn get_entrance_and_exit_accounts() -> (AccountIdOf<T>, AccountIdOf<T>) {
		(
			T::EntranceAccount::get().into_account_truncating(),
			T::ExitAccount::get().into_account_truncating(),
		)
	}

	fn get_moonbeam_parachain_id() -> u32 {
		T::MoonbeamChainId::get()
	}
}

impl<T: Config> VtokenMintingInterface<AccountIdOf<T>, CurrencyIdOf<T>, BalanceOf<T>>
	for Pallet<T>
{
	fn mint(
		exchanger: AccountIdOf<T>,
		currency_id: CurrencyIdOf<T>,
		currency_amount: BalanceOf<T>,
		remark: BoundedVec<u8, ConstU32<32>>,
		channel_id: Option<u32>,
	) -> Result<BalanceOf<T>, DispatchError> {
		Self::do_mint(exchanger, currency_id, currency_amount, remark, channel_id)
	}

	fn redeem(
		redeemer: AccountIdOf<T>,
		v_currency_id: CurrencyIdOf<T>,
		v_currency_amount: BalanceOf<T>,
	) -> DispatchResult {
		let redeem_creator = RedeemCreator::Substrate(redeemer.clone());
		let redeem_to = RedeemTo::Native(redeemer);
		Self::do_redeem(redeem_creator, v_currency_id, v_currency_amount, redeem_to)
	}

	fn slpx_redeem(
		redeem_creator: RedeemCreator<T::AccountId>,
		v_currency_id: CurrencyIdOf<T>,
		v_currency_amount: BalanceOf<T>,
		redeem_to: RedeemTo<T::AccountId>,
	) -> DispatchResult {
		Self::do_redeem(redeem_creator, v_currency_id, v_currency_amount, redeem_to)
	}

	fn get_v_currency_amount_by_currency_amount(
		currency_id: CurrencyIdOf<T>,
		v_currency_id: CurrencyIdOf<T>,
		currency_amount: BalanceOf<T>,
	) -> Result<BalanceOf<T>, DispatchError> {
		let total_stake_amount = ActiveStateByCurrency::<T>::get(currency_id)
			.ok_or(Error::<T>::UnlockDurationNotFound)?
			.total_stake_amount();
		let v_currency_total_issuance = T::MultiCurrency::total_issuance(v_currency_id);

		if BalanceOf::<T>::zero().eq(&total_stake_amount) {
			Ok(currency_amount)
		} else {
			Ok(multiply_by_rational_with_rounding(
				currency_amount.saturated_into::<u128>(),
				v_currency_total_issuance.saturated_into::<u128>(),
				total_stake_amount.saturated_into::<u128>(),
				Rounding::Down,
			)
			.ok_or(Error::<T>::CalculationOverflow)?
			.unique_saturated_into())
		}
	}

	/// Get the v_currency amount by currency amount.
	/// Parameters:
	/// - `currency_id`: The currency id.
	/// - `v_currency_id`: The v_currency id.
	/// - `currency_amount`: The currency amount.
	/// Returns:
	/// - `Result`: The v_currency amount.
	fn get_currency_amount_by_v_currency_amount(
		currency_id: CurrencyIdOf<T>,
		v_currency_id: CurrencyIdOf<T>,
		v_currency_amount: BalanceOf<T>,
	) -> Result<BalanceOf<T>, DispatchError> {
		let total_stake_amount = ActiveStateByCurrency::<T>::get(currency_id)
			.ok_or(Error::<T>::UnlockDurationNotFound)?
			.total_stake_amount();
		let v_currency_total_issuance = T::MultiCurrency::total_issuance(v_currency_id);

		if BalanceOf::<T>::zero().eq(&v_currency_total_issuance) {
			Ok(v_currency_amount)
		} else {
			Ok(multiply_by_rational_with_rounding(
				v_currency_amount.saturated_into::<u128>(),
				total_stake_amount.saturated_into::<u128>(),
				v_currency_total_issuance.saturated_into::<u128>(),
				Rounding::Down,
			)
			.ok_or(Error::<T>::CalculationOverflow)?
			.unique_saturated_into())
		}
	}

	fn total_stake_amount(currency_id: CurrencyId) -> BalanceOf<T> {
		ActiveStateByCurrency::<T>::get(currency_id)
			.map(|active_state| active_state.total_stake_amount())
			.unwrap_or_default()
	}

	fn get_moonbeam_parachain_id() -> u32 {
		T::MoonbeamChainId::get()
	}
}

impl<T: Config> VTokenSupplyProvider<CurrencyIdOf<T>, BalanceOf<T>> for Pallet<T> {
	fn get_vtoken_supply(vtoken: CurrencyIdOf<T>) -> Option<BalanceOf<T>> {
		if CurrencyId::is_vtoken(&vtoken) {
			Some(T::MultiCurrency::total_issuance(vtoken))
		} else {
			None
		}
	}

	fn get_token_supply(token: CurrencyIdOf<T>) -> Option<BalanceOf<T>> {
		if CurrencyId::is_token(&token) | CurrencyId::is_native(&token) {
			Some(TokenPool::<T>::get(token))
		} else {
			None
		}
	}
}

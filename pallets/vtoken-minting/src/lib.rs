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
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod impls;
pub mod migration;
pub mod traits;
pub mod types;
pub mod weights;
pub use weights::WeightInfo;

use crate::impls::Operation;
use bb_bnc::traits::BbBNCInterface;
use bifrost_primitives::{
	CurrencyId, RedeemCreator, RedeemTo, RedeemType, SlpxOperator, TimeUnit,
	VTokenMintRedeemProvider,
};
use frame_support::{
	pallet_prelude::{DispatchResultWithPostInfo, *},
	sp_runtime::{
		traits::{CheckedAdd, CheckedSub, Saturating, Zero},
		DispatchError, Permill,
	},
	traits::LockIdentifier,
	BoundedVec, PalletId,
};
use frame_system::pallet_prelude::*;
use log;
use orml_traits::{MultiCurrency, MultiLockableCurrency, XcmTransfer};
pub use pallet::*;
use sp_std::vec;
pub use traits::*;
use types::{
	AccountIdOf, ActiveState, BalanceOf, CurrencyConfiguration, CurrencyIdOf, RedeemOrderId,
	INCENTIVE_LOCK_ID,
};

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use crate::types::RedeemQueue;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Set default weight.
		type WeightInfo: WeightInfo;
		/// The only origin that can edit token issuer list
		type ControlOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// The multi currency trait.
		type MultiCurrency: MultiCurrency<AccountIdOf<Self>, CurrencyId = CurrencyId>
			+ MultiLockableCurrency<AccountIdOf<Self>>;
		/// Handler to notify the runtime when redeem success
		/// If you don't need it, you can specify the type `()`.
		type OnRedeemSuccess: OnRedeemSuccess<
			AccountIdOf<Self>,
			CurrencyIdOf<Self>,
			BalanceOf<Self>,
		>;
		/// Xtokens xcm transfer interface
		type XcmTransfer: XcmTransfer<AccountIdOf<Self>, BalanceOf<Self>, CurrencyIdOf<Self>>;
		/// Slpx operator
		type BifrostSlpx: SlpxOperator<BalanceOf<Self>>;
		/// bbBNC interface
		type BbBNC: BbBNCInterface<
			AccountIdOf<Self>,
			CurrencyIdOf<Self>,
			BalanceOf<Self>,
			BlockNumberFor<Self>,
		>;
		/// Channel commission provider
		type ChannelCommission: VTokenMintRedeemProvider<CurrencyId, BalanceOf<Self>>;

		/// Maximum unlock id of user
		#[pallet::constant]
		type MaximumUnlockIdOfUser: Get<u32>;
		/// Maximum unlock id of time unit
		#[pallet::constant]
		type MaximumUnlockIdOfTimeUnit: Get<u32>;
		/// Maximum unlocked vtoken records minted in an incentive mode
		#[pallet::constant]
		type MaxLockRecords: Get<u32>;
		/// Currency receive account
		#[pallet::constant]
		type EntranceAccount: Get<PalletId>;
		/// Currency exit account
		#[pallet::constant]
		type ExitAccount: Get<PalletId>;
		/// Fee account
		#[pallet::constant]
		type FeeAccount: Get<Self::AccountId>;
		/// Redeem fee account
		#[pallet::constant]
		type RedeemFeeAccount: Get<Self::AccountId>;

		#[pallet::constant]
		type IncentivePoolAccount: Get<PalletId>;

		#[pallet::constant]
		type RelayChainToken: Get<CurrencyId>;

		#[pallet::constant]
		type MoonbeamChainId: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Vtoken minted successfully.
		Minted {
			/// The minter account.
			minter: AccountIdOf<T>,
			/// The currency id minted.
			currency_id: CurrencyIdOf<T>,
			/// The currency amount minted.
			currency_amount: BalanceOf<T>,
			/// The v_currency amount minted.
			v_currency_amount: BalanceOf<T>,
			/// The mint fee.
			mint_fee: BalanceOf<T>,
			/// The remark of minting.
			remark: BoundedVec<u8, ConstU32<32>>,
			/// The channel id of minting.
			channel_id: Option<u32>,
		},
		///	Vtoken redeemed successfully.
		Redeemed { id: RedeemOrderId },
		/// Process redeem successfully.
		RedeemSuccess {
			/// The redeemer account.
			redeemer: AccountIdOf<T>,
			/// The unlock_id redeemed.
			unlock_id: RedeemOrderId,
			/// The currency id redeemed.
			currency_id: CurrencyIdOf<T>,
			/// Will transfer to this account.
			to: RedeemTo<AccountIdOf<T>>,
			/// The redeem amount.
			currency_amount: BalanceOf<T>,
		},
		/// Vtoken rebonded successfully.
		Rebonded {
			/// The rebonder account.
			rebonder: AccountIdOf<T>,
			/// The currency id rebonded.
			currency_id: CurrencyIdOf<T>,
			/// The currency amount rebonded.
			currency_amount: BalanceOf<T>,
			/// The v_currency amount rebonded.
			v_currency_amount: BalanceOf<T>,
			/// Mint fee
			fee: BalanceOf<T>,
		},
		/// Vtoken rebonded by unlock_id successfully.
		RebondedByUnlockId {
			/// The rebonder account.
			rebonder: AccountIdOf<T>,
			/// The currency id rebonded.
			currency_id: CurrencyIdOf<T>,
			/// The currency amount rebonded.
			currency_amount: BalanceOf<T>,
			/// The v_currency amount rebonded.
			v_currency_amount: BalanceOf<T>,
			/// Mint fee
			fee: BalanceOf<T>,
			/// The unlock_id rebonded.
			unlock_id: RedeemOrderId,
		},
		/// Set unlock duration.
		UnlockDurationSet {
			/// The currency id set unlock duration.
			currency_id: CurrencyIdOf<T>,
			/// The unlock duration set.
			unlock_duration: TimeUnit,
		},
		/// Set minimum mint amount.
		MinimumMintAndRedeemSet {
			/// The currency id set minimum mint amount.
			currency_id: CurrencyIdOf<T>,
			/// The minimum mint amount set.
			min_mint_amount: BalanceOf<T>,
			min_redeem_amount: BalanceOf<T>,
		},
		/// Set minimum redeem amount.
		MinimumRedeemSet {
			/// The currency id set minimum redeem amount.
			currency_id: CurrencyIdOf<T>,
			/// The minimum redeem amount set.
			minimum_amount: BalanceOf<T>,
		},
		/// Support rebond token added.
		SupportRebondTokenAdded {
			/// The currency id support rebond.
			currency_id: CurrencyIdOf<T>,
		},
		/// Support rebond token removed.
		SupportRebondTokenRemoved {
			/// The currency id remove support rebond.
			currency_id: CurrencyIdOf<T>,
		},
		/// Set mint fee and redeem fee.
		FeeRateSet {
			/// The mint fee rate set.
			mint_fee_rate: Permill,
			/// The redeem fee rate set.
			redeem_fee_rate: Permill,
		},
		/// Set hook iteration limit.
		HookIterationLimitSet { limit: u32 },
		/// Set unlock total amount.
		TotalAmountSet {
			/// The currency id set unlock total amount.
			currency_id: CurrencyIdOf<T>,
			/// The unlock total amount set.
			total_stake_amount: BalanceOf<T>,
			total_unstake_amount: BalanceOf<T>,
			total_restake_amount: BalanceOf<T>,
		},
		/// Set minimum time unit.
		MinTimeUnitSet {
			/// The currency id set minimum time unit.
			currency_id: CurrencyIdOf<T>,
			/// The minimum time unit set.
			time_unit: TimeUnit,
		},
		/// Fast redeem failed.
		FastRedeemFailed { err: DispatchError },
		/// Set ongoing time unit.
		SetTimeUnit {
			/// The currency id set ongoing time unit.
			currency_id: CurrencyIdOf<T>,
			/// The ongoing time unit set.
			ongoing_time_unit: TimeUnit,
			min_time_unit: TimeUnit,
		},
		/// Incentivized minting.
		IncentivizedMinting {
			address: AccountIdOf<T>,
			currency_id: CurrencyIdOf<T>,
			currency_amount: BalanceOf<T>,
			locked_vtoken_amount: BalanceOf<T>,
			incentive_vtoken_amount: BalanceOf<T>,
		},
		/// Incentive coefficient set.
		VtokenIncentiveCoefSet { v_currency_id: CurrencyIdOf<T>, coefficient: Option<u128> },
		/// Incentive lock blocks set.
		VtokenIncentiveLockBlocksSet {
			v_currency_id: CurrencyIdOf<T>,
			blocks: Option<BlockNumberFor<T>>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Below minimum mint amount.
		BelowMinimumMint,
		/// Below minimum redeem amount.
		BelowMinimumRedeem,
		/// Invalid token to rebond.
		InvalidRebondToken,
		/// Token type not support.
		NotSupportTokenType,
		/// Not enough balance to unlock.
		NotEnoughBalanceToUnlock,
		/// Token unlock ledger not found.
		TokenToRebondNotZero,
		/// Ongoing time unit not set.
		OngoingTimeUnitNotSet,
		/// Token unlock ledger not found.
		TokenUnlockLedgerNotFound,
		/// User unlock ledger not found.
		UserUnlockLedgerNotFound,
		/// Time unit unlock ledger not found.
		TimeUnitUnlockLedgerNotFound,
		/// Unlock duration not found.
		UnlockDurationNotFound,
		/// Unexpected error.
		Unexpected,
		/// Calculation overflow.
		CalculationOverflow,
		/// Exceed maximum unlock id.
		ExceedMaximumUnlockId,
		/// Too many redeems.
		TooManyRedeems,
		/// Can not rebond.
		CanNotRebond,
		/// Not enough balance.
		NotEnoughBalance,
		/// veBNC checking error.
		VeBNCCheckingError,
		/// IncentiveCoef not found.
		IncentiveCoefNotFound,
		/// Too many locks.
		TooManyLocks,
		/// No unlock record.
		NoUnlockRecord,
		/// Fail to remove lock.
		FailToRemoveLock,
		/// Balance not zero.
		BalanceZero,
		/// IncentiveLockBlocksNotSet
		IncentiveLockBlocksNotSet,
		ConfigurationNotFound,
	}

	/// Token pool amount
	#[pallet::storage]
	pub type TokenPool<T: Config> =
		StorageMap<_, Twox64Concat, CurrencyIdOf<T>, BalanceOf<T>, ValueQuery>;

	/// According to currency_id and unlock_id, unlock information are stored.
	#[pallet::storage]
	pub type TokenUnlockLedger<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CurrencyIdOf<T>,
		Blake2_128Concat,
		RedeemOrderId,
		(
			// redeemer account
			T::AccountId,
			// redeem amount
			BalanceOf<T>,
			// lock to time unit
			TimeUnit,
			// redeem type
			RedeemType<AccountIdOf<T>>,
		),
		OptionQuery,
	>;

	/// According to the user's account, the locked amount and unlock id list are stored.
	#[pallet::storage]
	pub type UserUnlockLedger<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AccountIdOf<T>,
		Blake2_128Concat,
		CurrencyIdOf<T>,
		(
			// Total locked amount
			BalanceOf<T>,
			// RedeemOrderId list
			BoundedVec<RedeemOrderId, T::MaximumUnlockIdOfUser>,
		),
		OptionQuery,
	>;

	/// The total amount of tokens that are currently locked for unlocking.
	#[pallet::storage]
	pub type TimeUnitUnlockLedger<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		TimeUnit,
		Blake2_128Concat,
		CurrencyIdOf<T>,
		(
			// Total locked amount
			BalanceOf<T>,
			// RedeemOrderId list
			BoundedVec<RedeemOrderId, T::MaximumUnlockIdOfTimeUnit>,
			// CurrencyId
			CurrencyIdOf<T>,
		),
		OptionQuery,
	>;

	/// The total amount of tokens that are currently unlocking.
	#[pallet::storage]
	pub type UnlockingTotal<T: Config> =
		StorageMap<_, Twox64Concat, CurrencyIdOf<T>, BalanceOf<T>, ValueQuery>;

	/// The hook iteration limit
	#[pallet::storage]
	pub type HookIterationLimit<T: Config> = StorageValue<_, u32, ValueQuery>;

	//【vtoken -> Blocks】, the locked blocks for each vtoken when minted in an incentive mode
	#[pallet::storage]
	pub type MintWithLockBlocks<T: Config> =
		StorageMap<_, Blake2_128Concat, CurrencyId, BlockNumberFor<T>>;

	//【vtoken -> incentive coefficient】,the incentive coefficient for each vtoken when minted in
	// an incentive mode
	#[pallet::storage]
	pub type VtokenIncentiveCoef<T: Config> = StorageMap<_, Blake2_128Concat, CurrencyId, u128>;

	//【user + vtoken -> (total_locked, vec[(locked_amount, due_block_num)])】, the locked vtoken
	// records for each user
	#[pallet::storage]
	pub type VtokenLockLedger<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		AccountIdOf<T>,
		Blake2_128Concat,
		CurrencyId,
		(BalanceOf<T>, BoundedVec<(BalanceOf<T>, BlockNumberFor<T>), T::MaxLockRecords>),
		OptionQuery,
	>;

	//【vtoken -> Blocks】, the locked blocks for each vtoken when minted in an incentive mode
	#[pallet::storage]
	pub type ConfigurationByCurrency<T: Config> =
		StorageMap<_, Blake2_128Concat, CurrencyId, CurrencyConfiguration<BalanceOf<T>>>;

	#[pallet::storage]
	pub type ActiveStateByCurrency<T: Config> =
		StorageMap<_, Blake2_128Concat, CurrencyId, ActiveState<BalanceOf<T>>>;

	#[pallet::storage]
	pub type RedeemQueueByCurrency<T: Config> =
		StorageMap<_, Blake2_128Concat, CurrencyId, RedeemQueue<T>, ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
			for currency in ConfigurationByCurrency::<T>::iter_keys() {
				let result = Self::handle_ledger_by_currency(currency);
				match result {
					Ok(_) => (),
					Err(err) => {
						Self::deposit_event(Event::FastRedeemFailed { err });
						log::error!(
							target: "runtime::vtoken-minting",
							"Received invalid justification for {:?}",
							err,
						);
					},
				}
			}

			T::WeightInfo::on_initialize()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Mint v_currency by transferring currency to entrance_account.
		/// The minted v_currency will be deposited to the minter's account.
		/// Parameters:
		/// - `currency_id`: The currency to mint.
		/// - `currency_amount`: The amount of currency to mint.
		/// - `remark`: The remark of minting.
		/// - `channel_id`: The channel id of minting.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::mint())]
		pub fn mint(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
			currency_amount: BalanceOf<T>,
			remark: BoundedVec<u8, ConstU32<32>>,
			channel_id: Option<u32>,
		) -> DispatchResult {
			// Check origin
			let minter = ensure_signed(origin)?;
			Self::do_mint(minter, currency_id, currency_amount, remark, channel_id)?;
			Ok(())
		}

		/// Redeem currency by burning v_currency. But need to wait for the unlock period.
		/// The redeemed currency will be transferred to the redeemer's account.
		/// Parameters:
		/// - `v_currency_id`: The v_currency to redeem.
		/// - `v_currency_amount`: The amount of v_currency to redeem.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::redeem())]
		pub fn redeem(
			origin: OriginFor<T>,
			v_currency_id: CurrencyIdOf<T>,
			v_currency_amount: BalanceOf<T>,
		) -> DispatchResult {
			let redeemer = ensure_signed(origin)?;
			let redeem_creator = RedeemCreator::Substrate(redeemer.clone());
			let redeem_to = RedeemTo::Native(redeemer);
			Self::do_redeem(redeem_creator, v_currency_id, v_currency_amount, redeem_to)
		}

		/// Same function as Rebond. But need to provide unlock_id.
		/// Parameters:
		/// - `currency_id`: The currency to rebond.
		/// - `unlock_id`: The unlock_id to rebond.
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::rebond_by_unlock_id())]
		pub fn rebond_by_unlock_id(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
			redeem_id: RedeemOrderId,
		) -> DispatchResult {
			let rebonder = ensure_signed(origin)?;
			Self::do_rebond_by_redeem_id(rebonder, currency_id, redeem_id)
		}

		/// Set the minimum mint amount for a currency.
		/// Parameters:
		/// - `currency_id`: The currency to set minimum mint amount.
		/// - `minimum_amount`: The minimum mint amount to set.
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::set_minimum_mint())]
		pub fn set_currency_config(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
			mint_fee_rate: Permill,
			redeem_fee_rate: Permill,
			unlock_duration: TimeUnit,
			min_mint_amount: BalanceOf<T>,
			min_redeem_amount: BalanceOf<T>,
			is_supported_restake: bool,
			is_supported_fast_redeem: bool,
		) -> DispatchResult {
			T::ControlOrigin::ensure_origin(origin)?;

			ConfigurationByCurrency::<T>::mutate(currency_id, |config| -> DispatchResult {
				*config = Some(CurrencyConfiguration {
					mint_fee_rate,
					redeem_fee_rate,
					unlock_duration,
					min_mint_amount,
					min_redeem_amount,
					is_supported_restake,
					is_supported_fast_redeem,
				});
				Ok(())
			})
		}

		/// Set the hook iteration limit.
		/// Parameters:
		/// - `limit`: The hook iteration limit.
		#[pallet::call_index(10)]
		#[pallet::weight(T::WeightInfo::set_hook_iteration_limit())]
		pub fn set_hook_iteration_limit(origin: OriginFor<T>, limit: u32) -> DispatchResult {
			T::ControlOrigin::ensure_origin(origin)?;

			HookIterationLimit::<T>::mutate(|old_limit| {
				*old_limit = limit;
			});

			Self::deposit_event(Event::HookIterationLimitSet { limit });
			Ok(())
		}

		/// Set the total amount of tokens that are currently locked for unlocking.
		/// Parameters:
		/// - `currency_id`: The currency to set unlocking total.
		/// - `currency_amount`: The total amount of tokens that are currently locked for unlocking.
		#[pallet::call_index(11)]
		#[pallet::weight(T::WeightInfo::set_unlocking_total())]
		pub fn set_active_state(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
			total_stake_amount: BalanceOf<T>,
			total_unstake_amount: BalanceOf<T>,
			total_restake_amount: BalanceOf<T>,
			ongoing_time_unit: TimeUnit,
			next_redeem_id: RedeemOrderId,
		) -> DispatchResult {
			T::ControlOrigin::ensure_origin(origin)?;
			ActiveStateByCurrency::<T>::mutate(currency_id, |active_state| -> DispatchResult {
				let active_state = active_state.as_mut().ok_or(Error::<T>::Unexpected)?;
				*active_state = ActiveState {
					total_stake_amount,
					total_unstake_amount,
					total_restake_amount,
					ongoing_time_unit,
					next_redeem_id,
				};
				Self::deposit_event(Event::TotalAmountSet {
					currency_id,
					total_stake_amount,
					total_unstake_amount,
					total_restake_amount,
				});
				Ok(())
			})
		}

		// mint with lock to get incentive vtoken
		#[pallet::call_index(14)]
		#[pallet::weight(T::WeightInfo::mint_with_lock())]
		pub fn mint_with_lock(
			origin: OriginFor<T>,
			currency_id: CurrencyIdOf<T>,
			currency_amount: BalanceOf<T>,
			remark: BoundedVec<u8, ConstU32<32>>,
			channel_id: Option<u32>,
		) -> DispatchResult {
			// Check origin
			let minter = ensure_signed(origin)?;

			// check if the minter has at least currency_amount of currency_id which is transferable
			T::MultiCurrency::ensure_can_withdraw(currency_id, &minter, currency_amount)
				.map_err(|_| Error::<T>::NotEnoughBalance)?;

			// check whether the currency_id is supported
			// ensure!(MinimumMint::<T>::contains_key(currency_id),
			// Error::<T>::NotSupportTokenType);

			// check whether the user has veBNC
			let vebnc_balance =
				T::BbBNC::balance_of(&minter, None).map_err(|_| Error::<T>::VeBNCCheckingError)?;
			ensure!(vebnc_balance > BalanceOf::<T>::zero(), Error::<T>::NotEnoughBalance);

			// check whether the vtoken coefficient is set
			let v_currency_id =
				currency_id.to_vtoken().map_err(|_| Error::<T>::NotSupportTokenType)?;

			ensure!(
				VtokenIncentiveCoef::<T>::contains_key(v_currency_id),
				Error::<T>::IncentiveCoefNotFound
			);

			// check whether the pool has balance of v_currency_id
			let incentive_pool_account = &Self::incentive_pool_account();
			let vtoken_pool_balance =
				T::MultiCurrency::free_balance(v_currency_id, &incentive_pool_account);

			ensure!(vtoken_pool_balance > BalanceOf::<T>::zero(), Error::<T>::NotEnoughBalance);

			// mint vtoken
			let vtoken_minted =
				Self::do_mint(minter.clone(), currency_id, currency_amount, remark, channel_id)?;

			// lock vtoken and record the lock
			Self::lock_vtoken_for_incentive_minting(minter.clone(), v_currency_id, vtoken_minted)?;

			// calculate the incentive amount
			let incentive_amount =
				Self::calculate_incentive_vtoken_amount(&minter, v_currency_id, vtoken_minted)?;

			// Since the user has already locked the vtoken, we can directly transfer the incentive
			// vtoken. It won't fail. transfer the incentive amount to the minter
			T::MultiCurrency::transfer(
				v_currency_id,
				incentive_pool_account,
				&minter,
				incentive_amount,
			)
			.map_err(|_| Error::<T>::NotEnoughBalance)?;

			// deposit event
			Self::deposit_event(Event::IncentivizedMinting {
				address: minter,
				currency_id,
				currency_amount,
				locked_vtoken_amount: vtoken_minted,
				incentive_vtoken_amount: incentive_amount,
			});

			Ok(())
		}

		/// Unlock the vtoken minted in an incentive mode
		/// Parameters:
		/// - `v_currency_id`: The v_currency to unlock.
		#[pallet::call_index(15)]
		#[pallet::weight(T::WeightInfo::unlock_incentive_minted_vtoken())]
		pub fn unlock_incentive_minted_vtoken(
			origin: OriginFor<T>,
			v_currency_id: CurrencyIdOf<T>,
		) -> DispatchResult {
			let unlocker = ensure_signed(origin)?;

			// get the user's VtokenLockLedger
			ensure!(
				VtokenLockLedger::<T>::contains_key(&unlocker, v_currency_id),
				Error::<T>::UserUnlockLedgerNotFound
			);

			VtokenLockLedger::<T>::mutate_exists(
				&unlocker,
				v_currency_id,
				|maybe_ledger| -> Result<(), Error<T>> {
					let current_block = frame_system::Pallet::<T>::block_number();

					if let Some(ref mut ledger) = maybe_ledger {
						// check the total locked amount
						let (total_locked, mut lock_records) = ledger.clone();

						// unlock the vtoken
						let mut unlock_amount = BalanceOf::<T>::zero();
						let mut remove_index = 0;

						// enumerate lock_records
						for (index, (locked_amount, due_block_num)) in
							lock_records.iter().enumerate()
						{
							if current_block >= *due_block_num {
								unlock_amount += *locked_amount;
								remove_index = index + 1;
							} else {
								break;
							}
						}

						// remove all the records less than remove_index
						if remove_index > 0 {
							lock_records.drain(0..remove_index);
						}

						// check the unlock amount
						ensure!(unlock_amount > BalanceOf::<T>::zero(), Error::<T>::NoUnlockRecord);

						let remaining_locked_amount = total_locked
							.checked_sub(&unlock_amount)
							.ok_or(Error::<T>::CalculationOverflow)?;

						if remaining_locked_amount == BalanceOf::<T>::zero() {
							T::MultiCurrency::remove_lock(
								INCENTIVE_LOCK_ID,
								v_currency_id,
								&unlocker,
							)
							.map_err(|_| Error::<T>::FailToRemoveLock)?;

							// remove the ledger
							*maybe_ledger = None;
						} else {
							// update the ledger
							*ledger = (remaining_locked_amount, lock_records);

							// reset the locked amount to be remaining_locked_amount
							T::MultiCurrency::set_lock(
								INCENTIVE_LOCK_ID,
								v_currency_id,
								&unlocker,
								remaining_locked_amount,
							)
							.map_err(|_| Error::<T>::Unexpected)?;
						}

						Ok(())
					} else {
						Err(Error::<T>::UserUnlockLedgerNotFound)
					}
				},
			)?;

			Ok(())
		}

		/// Set the incentive coefficient for a vtoken when minted in an incentive mode
		/// Parameters:
		/// - `v_currency_id`: The v_currency to set incentive coefficient.
		/// - `new_coef_op`: The new incentive coefficient to set.
		#[pallet::call_index(16)]
		#[pallet::weight(T::WeightInfo::set_incentive_coef())]
		pub fn set_incentive_coef(
			origin: OriginFor<T>,
			v_currency_id: CurrencyIdOf<T>,
			new_coef_op: Option<u128>,
		) -> DispatchResult {
			T::ControlOrigin::ensure_origin(origin)?;

			if let Some(new_coef) = new_coef_op {
				VtokenIncentiveCoef::<T>::insert(v_currency_id, new_coef);
			} else {
				VtokenIncentiveCoef::<T>::remove(v_currency_id);
			}

			Self::deposit_event(Event::VtokenIncentiveCoefSet {
				v_currency_id,
				coefficient: new_coef_op,
			});

			Ok(())
		}

		/// Set the locked blocks for a vtoken when minted in an incentive mode
		/// Parameters:
		/// - `v_currency_id`: The v_currency to set locked blocks.
		/// - `new_blockes_op`: The new locked blocks to set.
		#[pallet::call_index(17)]
		#[pallet::weight(T::WeightInfo::set_vtoken_incentive_lock_blocks())]
		pub fn set_vtoken_incentive_lock_blocks(
			origin: OriginFor<T>,
			v_currency_id: CurrencyIdOf<T>,
			new_blockes_op: Option<BlockNumberFor<T>>,
		) -> DispatchResult {
			T::ControlOrigin::ensure_origin(origin)?;

			if let Some(new_blocks) = new_blockes_op {
				MintWithLockBlocks::<T>::insert(v_currency_id, new_blocks);
			} else {
				MintWithLockBlocks::<T>::remove(v_currency_id);
			}

			Self::deposit_event(Event::VtokenIncentiveLockBlocksSet {
				v_currency_id,
				blocks: new_blockes_op,
			});

			Ok(())
		}
	}
}

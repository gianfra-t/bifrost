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

use crate::{pallet, Config};
use bifrost_primitives::{CurrencyId, RedeemCreator, RedeemTo, TimeUnit};
use frame_support::{dispatch::DispatchResult, traits::LockIdentifier};
use orml_traits::MultiCurrency;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{H160, H256, U256};
use sp_runtime::{
	traits::{ConstU32, One},
	BoundedVec, Permill, RuntimeDebug, Saturating,
};
use sp_std::vec::Vec;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type CurrencyIdOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<
	<T as frame_system::Config>::AccountId,
>>::CurrencyId;
pub type BalanceOf<T> = <<T as Config>::MultiCurrency as MultiCurrency<AccountIdOf<T>>>::Balance;
pub type RedeemOrderId = u32;
// incentive lock id for vtoken minted by user
pub const INCENTIVE_LOCK_ID: LockIdentifier = *b"vmincntv";

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct CurrencyConfiguration<Balance> {
	pub mint_fee_rate: Permill,
	pub redeem_fee_rate: Permill,
	pub unlock_duration: TimeUnit,
	pub min_mint_amount: Balance,
	pub min_redeem_amount: Balance,
	pub is_supported_restake: bool,
	pub is_supported_fast_redeem: bool,
}

impl<Balance> CurrencyConfiguration<Balance>
where
	Balance: Saturating + One + Copy,
{
	pub fn mint_fee_rate(&self) -> Permill {
		self.mint_fee_rate
	}

	pub fn is_supported_restake(&self) -> bool {
		self.is_supported_restake
	}

	pub fn redeem_fee_rate(&self) -> Permill {
		self.redeem_fee_rate
	}

	pub fn is_supported_fast_redeem(&self) -> bool {
		self.is_supported_fast_redeem
	}

	pub fn unlock_duration(&self) -> TimeUnit {
		self.unlock_duration
	}

	pub fn min_mint_amount(&self) -> Balance {
		self.min_mint_amount
	}

	pub fn min_redeem_amount(&self) -> Balance {
		self.min_redeem_amount
	}

	pub fn set_mint_fee_rate(&mut self, rate: Permill) {
		self.mint_fee_rate = rate;
	}

	pub fn set_redeem_fee_rate(&mut self, rate: Permill) {
		self.redeem_fee_rate = rate;
	}

	pub fn set_unlock_duration(&mut self, duration: TimeUnit) {
		self.unlock_duration = duration;
	}

	pub fn set_min_mint_amount(&mut self, amount: Balance) {
		self.min_mint_amount = amount;
	}

	pub fn set_min_redeem_amount(&mut self, amount: Balance) {
		self.min_redeem_amount = amount;
	}

	pub fn set_supported_restake(&mut self, supported: bool) {
		self.is_supported_restake = supported;
	}
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct ActiveState<Balance> {
	pub total_stake_amount: Balance,
	pub total_unstake_amount: Balance,
	pub total_restake_amount: Balance,
	pub ongoing_time_unit: TimeUnit,
	pub next_redeem_id: RedeemOrderId,
}

impl<Balance> ActiveState<Balance>
where
	Balance: Saturating + One + Copy,
{
	pub fn total_stake_amount(&self) -> Balance {
		self.total_stake_amount
	}

	pub fn ongoing_time_unit(&self) -> TimeUnit {
		self.ongoing_time_unit
	}

	pub fn next_redeem_id(&self) -> RedeemOrderId {
		self.next_redeem_id
	}

	pub fn set_total_stake_amount(&mut self, amount: Balance) {
		self.total_stake_amount = amount;
	}

	pub fn set_total_restake_amount(&mut self, amount: Balance) {
		self.total_restake_amount = amount;
	}

	pub fn set_total_unstake_amount(&mut self, amount: Balance) {
		self.total_unstake_amount = amount;
	}

	pub fn set_ongoing_time_unit(&mut self, time_unit: TimeUnit) {
		self.ongoing_time_unit = time_unit;
	}

	pub fn add_stake_amount(&mut self, amount: Balance) {
		self.total_stake_amount.saturating_accrue(amount);
	}

	pub fn subtract_stake_amount(&mut self, amount: Balance) {
		self.total_stake_amount.saturating_reduce(amount);
	}

	pub fn add_unstake_amount(&mut self, amount: Balance) {
		self.total_unstake_amount.saturating_accrue(amount);
	}

	pub fn subtract_unstake_amount(&mut self, amount: Balance) {
		self.total_unstake_amount.saturating_reduce(amount);
	}

	pub fn add_restake_amount(&mut self, amount: Balance) {
		self.total_restake_amount.saturating_accrue(amount);
	}

	pub fn subtract_restake_amount(&mut self, amount: Balance) {
		self.total_restake_amount.saturating_reduce(amount);
	}

	pub fn add_next_redeem_id(&mut self) {
		self.next_redeem_id.saturating_accrue(RedeemOrderId::one());
	}
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct RedeemOrder<Account, Balance> {
	pub id: RedeemOrderId,
	pub creator: RedeemCreator<Account>,
	pub currency: CurrencyId,
	pub currency_amount: Balance,
	pub remaining_currency_amount: Balance,
	pub v_currency: CurrencyId,
	pub v_currency_amount: Balance,
	pub final_time_unit: TimeUnit,
	pub redeem_to: RedeemTo<Account>,
}

impl<Account, Balance> RedeemOrder<Account, Balance>
where
	Balance: Saturating + One + Copy,
{
	pub fn currency(&self) -> CurrencyId {
		self.currency
	}

	pub fn currency_amount(&self) -> Balance {
		self.currency_amount
	}

	pub fn v_currency(&self) -> CurrencyId {
		self.v_currency
	}

	pub fn v_currency_amount(&self) -> Balance {
		self.v_currency_amount
	}

	pub fn remaining_currency_amount(&self) -> Balance {
		self.remaining_currency_amount
	}

	pub fn redeem_to(&self) -> &RedeemTo<Account> {
		&self.redeem_to
	}

	pub fn subtract_currency_amount(&mut self, amount: Balance) {
		self.currency_amount.saturating_reduce(amount);
	}

	pub fn subtract_remaining_currency_amount(&mut self, amount: Balance) {
		self.remaining_currency_amount.saturating_reduce(amount);
	}

	pub fn is_native_redeem(&self) -> bool {
		match self.redeem_to {
			RedeemTo::Native(_) => true,
			_ => false,
		}
	}
}

pub type RedeemQueue<T> =
	BoundedVec<RedeemOrder<AccountIdOf<T>, BalanceOf<T>>, <T as Config>::MaxLockRecords>;

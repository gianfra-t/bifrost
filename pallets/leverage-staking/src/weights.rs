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
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for leverage_staking
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-11-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `mjl-legion`, CPU: `12th Gen Intel(R) Core(TM) i9-12900H`
//! WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-kusama-local"), DB CACHE: 1024

// Executed Command:
// target/release/bifrost
// benchmark
// pallet
// --chain=bifrost-kusama-local
// --steps=50
// --repeat=20
// --pallet=leverage_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/leverage-staking/src/weights.rs
// --template=./weight-template/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for leverage_staking.
pub trait WeightInfo {
	fn flash_loan_deposit() -> Weight;
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `LendMarket::AccountDeposits` (r:2 w:1)
	/// Proof: `LendMarket::AccountDeposits` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::Markets` (r:3 w:0)
	/// Proof: `LendMarket::Markets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::TotalSupply` (r:1 w:1)
	/// Proof: `LendMarket::TotalSupply` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `LendMarket::TotalBorrows` (r:2 w:1)
	/// Proof: `LendMarket::TotalBorrows` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::TotalReserves` (r:2 w:0)
	/// Proof: `LendMarket::TotalReserves` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `LendMarket::LastAccruedInterestTime` (r:2 w:2)
	/// Proof: `LendMarket::LastAccruedInterestTime` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::AccountBorrows` (r:2 w:1)
	/// Proof: `LendMarket::AccountBorrows` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VtokenMinting::TokenPool` (r:1 w:1)
	/// Proof: `VtokenMinting::TokenPool` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:2 w:2)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::CurrencyMetadatas` (r:2 w:0)
	/// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `VtokenMinting::MinimumMint` (r:1 w:0)
	/// Proof: `VtokenMinting::MinimumMint` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	/// Storage: `VtokenMinting::Fees` (r:1 w:0)
	/// Proof: `VtokenMinting::Fees` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `ChannelCommission::PeriodVtokenTotalMint` (r:1 w:1)
	/// Proof: `ChannelCommission::PeriodVtokenTotalMint` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::RewardSupplyState` (r:1 w:1)
	/// Proof: `LendMarket::RewardSupplyState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::RewardSupplySpeed` (r:1 w:0)
	/// Proof: `LendMarket::RewardSupplySpeed` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::RewardSupplierIndex` (r:1 w:1)
	/// Proof: `LendMarket::RewardSupplierIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::RewardAccured` (r:1 w:1)
	/// Proof: `LendMarket::RewardAccured` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::AccountEarned` (r:1 w:1)
	/// Proof: `LendMarket::AccountEarned` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Prices::EmergencyPrice` (r:2 w:0)
	/// Proof: `Prices::EmergencyPrice` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::MarketBond` (r:1 w:0)
	/// Proof: `LendMarket::MarketBond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::LiquidationFreeCollaterals` (r:1 w:0)
	/// Proof: `LendMarket::LiquidationFreeCollaterals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::RewardBorrowState` (r:1 w:1)
	/// Proof: `LendMarket::RewardBorrowState` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::RewardBorrowSpeed` (r:1 w:0)
	/// Proof: `LendMarket::RewardBorrowSpeed` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::RewardBorrowerIndex` (r:1 w:1)
	/// Proof: `LendMarket::RewardBorrowerIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LendMarket::BorrowIndex` (r:1 w:0)
	/// Proof: `LendMarket::BorrowIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn flash_loan_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3439`
		//  Estimated: `13955`
		// Minimum execution time: 306_718_000 picoseconds.
		Weight::from_parts(313_182_000, 13955)
			.saturating_add(RocksDbWeight::get().reads(43_u64))
			.saturating_add(RocksDbWeight::get().writes(21_u64))
	}
}

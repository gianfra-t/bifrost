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

//! Autogenerated weights for bifrost_slpx
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-14, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bifrost-jenkins`, CPU: `Intel(R) Xeon(R) CPU E5-26xx v4`
//! WASM-EXECUTION: Compiled, CHAIN: Some("bifrost-kusama-local"), DB CACHE: 1024

// Executed Command:
// target/release/bifrost
// benchmark
// pallet
// --chain=bifrost-kusama-local
// --steps=50
// --repeat=20
// --pallet=bifrost_slpx
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/bifrost-kusama/src/weights/bifrost_slpx.rs
// --template=./weight-template/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions for bifrost_slpx.
pub struct BifrostWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> bifrost_slpx::WeightInfo for BifrostWeight<T> {
	// Storage: Slpx WhitelistAccountId (r:1 w:1)
	// Proof: Slpx WhitelistAccountId (max_values: None, max_size: Some(338), added: 2813, mode: MaxEncodedLen)
	fn add_whitelist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `3803`
		// Minimum execution time: 33_664 nanoseconds.
		Weight::from_parts(34_525_000, 3803)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Slpx WhitelistAccountId (r:1 w:1)
	// Proof: Slpx WhitelistAccountId (max_values: None, max_size: Some(338), added: 2813, mode: MaxEncodedLen)
	fn remove_whitelist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `81`
		//  Estimated: `3803`
		// Minimum execution time: 37_038 nanoseconds.
		Weight::from_parts(38_028_000, 3803)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Slpx ExecutionFee (r:0 w:1)
	// Proof: Slpx ExecutionFee (max_values: None, max_size: Some(46), added: 2521, mode: MaxEncodedLen)
	fn set_execution_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 24_630 nanoseconds.
		Weight::from_parts(25_469_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Slpx TransferToFee (r:0 w:1)
	// Proof: Slpx TransferToFee (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	fn set_transfer_to_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 24_544 nanoseconds.
		Weight::from_parts(25_336_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Slpx WhitelistAccountId (r:1 w:0)
	// Proof: Slpx WhitelistAccountId (max_values: None, max_size: Some(338), added: 2813, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	// Storage: Slpx ExecutionFee (r:1 w:0)
	// Proof: Slpx ExecutionFee (max_values: None, max_size: Some(46), added: 2521, mode: MaxEncodedLen)
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:3 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: VtokenMinting MinimumMint (r:1 w:0)
	// Proof: VtokenMinting MinimumMint (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	// Storage: VtokenMinting TokenPool (r:1 w:1)
	// Proof: VtokenMinting TokenPool (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	// Storage: VtokenMinting Fees (r:1 w:0)
	// Proof: VtokenMinting Fees (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:0)
	// Proof Skipped: AssetRegistry CurrencyIdToLocations (max_values: None, max_size: None, mode: Measured)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2442`
		//  Estimated: `11362`
		// Minimum execution time: 355_230 nanoseconds.
		Weight::from_parts(360_766_000, 11362)
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	// Storage: Slpx WhitelistAccountId (r:1 w:0)
	// Proof: Slpx WhitelistAccountId (max_values: None, max_size: Some(338), added: 2813, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	// Storage: Slpx ExecutionFee (r:1 w:0)
	// Proof: Slpx ExecutionFee (max_values: None, max_size: Some(46), added: 2521, mode: MaxEncodedLen)
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:3 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: VtokenMinting MinimumMint (r:1 w:0)
	// Proof: VtokenMinting MinimumMint (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	// Storage: VtokenMinting TokenPool (r:1 w:1)
	// Proof: VtokenMinting TokenPool (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	// Storage: VtokenMinting Fees (r:1 w:0)
	// Proof: VtokenMinting Fees (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:0)
	// Proof Skipped: AssetRegistry CurrencyIdToLocations (max_values: None, max_size: None, mode: Measured)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn mint_with_channel_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2442`
		//  Estimated: `11362`
		// Minimum execution time: 355_230 nanoseconds.
		Weight::from_parts(360_766_000, 11362)
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	// Storage: Slpx WhitelistAccountId (r:1 w:0)
	// Proof: Slpx WhitelistAccountId (max_values: None, max_size: Some(338), added: 2813, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	// Storage: Slpx ExecutionFee (r:1 w:0)
	// Proof: Slpx ExecutionFee (max_values: None, max_size: Some(46), added: 2521, mode: MaxEncodedLen)
	// Storage: AssetRegistry CurrencyMetadatas (r:2 w:0)
	// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: VtokenMinting MinimumRedeem (r:1 w:0)
	// Proof: VtokenMinting MinimumRedeem (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	// Storage: Slp DelegationsOccupied (r:1 w:0)
	// Proof Skipped: Slp DelegationsOccupied (max_values: None, max_size: None, mode: Measured)
	// Storage: VtokenMinting Fees (r:1 w:0)
	// Proof: VtokenMinting Fees (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	// Storage: VtokenMinting TokenPool (r:1 w:0)
	// Proof: VtokenMinting TokenPool (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(38), added: 2513, mode: MaxEncodedLen)
	// Storage: VtokenMinting OngoingTimeUnit (r:1 w:0)
	// Proof: VtokenMinting OngoingTimeUnit (max_values: None, max_size: Some(27), added: 2502, mode: MaxEncodedLen)
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:0)
	// Proof Skipped: AssetRegistry CurrencyIdToLocations (max_values: None, max_size: None, mode: Measured)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn redeem() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2194`
		//  Estimated: `8134`
		// Minimum execution time: 243_971 nanoseconds.
		Weight::from_parts(250_684_000, 8134)
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Slpx WhitelistAccountId (r:1 w:0)
	// Proof: Slpx WhitelistAccountId (max_values: None, max_size: Some(338), added: 2813, mode: MaxEncodedLen)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	// Storage: Slpx ExecutionFee (r:1 w:0)
	// Proof: Slpx ExecutionFee (max_values: None, max_size: Some(46), added: 2521, mode: MaxEncodedLen)
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:0)
	// Proof Skipped: AssetRegistry CurrencyIdToLocations (max_values: None, max_size: None, mode: Measured)
	fn zenlink_swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1819`
		//  Estimated: `11362`
		// Minimum execution time: 220_866 nanoseconds.
		Weight::from_parts(223_033_000, 11362)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Slpx WhitelistAccountId (r:1 w:0)
	// Proof: Slpx WhitelistAccountId (max_values: None, max_size: Some(338), added: 2813, mode: MaxEncodedLen)
	// Storage: StableAsset Pools (r:1 w:0)
	// Proof Skipped: StableAsset Pools (max_values: None, max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:6 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(118), added: 2593, mode: MaxEncodedLen)
	// Storage: Slpx ExecutionFee (r:1 w:0)
	// Proof: Slpx ExecutionFee (max_values: None, max_size: Some(46), added: 2521, mode: MaxEncodedLen)
	// Storage: AssetRegistry CurrencyMetadatas (r:1 w:0)
	// Proof Skipped: AssetRegistry CurrencyMetadatas (max_values: None, max_size: None, mode: Measured)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: StableAsset TokenRateCaches (r:2 w:0)
	// Proof Skipped: StableAsset TokenRateCaches (max_values: None, max_size: None, mode: Measured)
	// Storage: AssetRegistry CurrencyIdToLocations (r:1 w:0)
	// Proof Skipped: AssetRegistry CurrencyIdToLocations (max_values: None, max_size: None, mode: Measured)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn stable_pool_swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2782`
		//  Estimated: `16548`
		// Minimum execution time: 552_131 nanoseconds.
		Weight::from_parts(564_804_000, 16548)
			.saturating_add(T::DbWeight::get().reads(17))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

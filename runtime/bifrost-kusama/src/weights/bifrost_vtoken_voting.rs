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

//! Autogenerated weights for bifrost_vtoken_voting
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
// --pallet=bifrost_vtoken_voting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/bifrost-kusama/src/weights/bifrost_vtoken_voting.rs
// --template=./weight-template/runtime-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions for bifrost_vtoken_voting.
pub struct BifrostWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> bifrost_vtoken_voting::WeightInfo for BifrostWeight<T> {
	// Storage: `VtokenVoting::UndecidingTimeout` (r:1 w:0)
	// Proof: `VtokenVoting::UndecidingTimeout` (`max_values`: None, `max_size`: Some(26), added: 2501, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::PendingVotingInfo` (r:1 w:1)
	// Proof: `VtokenVoting::PendingVotingInfo` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	// Storage: `VtokenMinting::TokenPool` (r:1 w:0)
	// Proof: `VtokenMinting::TokenPool` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::ReferendumInfoFor` (r:1 w:1)
	// Proof: `VtokenVoting::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:1 w:1)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::VotingFor` (r:1 w:1)
	// Proof: `VtokenVoting::VotingFor` (`max_values`: None, `max_size`: Some(18271), added: 20746, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::ClassLocksFor` (r:1 w:1)
	// Proof: `VtokenVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(7722), added: 10197, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Locks` (r:1 w:1)
	// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1271), added: 3746, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `VtokenVoting::VoteCapRatio` (r:1 w:0)
	// Proof: `VtokenVoting::VoteCapRatio` (`max_values`: None, `max_size`: Some(26), added: 2501, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::DelegatorVotes` (r:1 w:0)
	// Proof: `VtokenVoting::DelegatorVotes` (`max_values`: None, `max_size`: Some(5136), added: 7611, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::Delegators` (r:1 w:0)
	// Proof: `VtokenVoting::Delegators` (`max_values`: None, `max_size`: Some(224), added: 2699, mode: `MaxEncodedLen`)
	// Storage: `Slp::DelegatorsIndex2Multilocation` (r:1 w:0)
	// Proof: `Slp::DelegatorsIndex2Multilocation` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Slp::DelegatorLedgers` (r:1 w:0)
	// Proof: `Slp::DelegatorLedgers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `VtokenVoting::PendingDelegatorVotes` (r:1 w:1)
	// Proof: `VtokenVoting::PendingDelegatorVotes` (`max_values`: None, `max_size`: Some(5136), added: 7611, mode: `MaxEncodedLen`)
	// Storage: `XcmInterface::XcmWeightAndFee` (r:1 w:0)
	// Proof: `XcmInterface::XcmWeightAndFee` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::QueryCounter` (r:1 w:1)
	// Proof: `PolkadotXcm::QueryCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::PendingReferendumInfo` (r:0 w:1)
	// Proof: `VtokenVoting::PendingReferendumInfo` (`max_values`: None, `max_size`: Some(34), added: 2509, mode: `MaxEncodedLen`)
	// Storage: `PolkadotXcm::Queries` (r:0 w:1)
	// Proof: `PolkadotXcm::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `14044`
		//  Estimated: `21736`
		// Minimum execution time: 111_727 nanoseconds.
		Weight::from_parts(116_540_000, 21736)
			.saturating_add(T::DbWeight::get().reads(19))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	// Storage: `VtokenVoting::UndecidingTimeout` (r:1 w:0)
	// Proof: `VtokenVoting::UndecidingTimeout` (`max_values`: None, `max_size`: Some(26), added: 2501, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::PendingVotingInfo` (r:1 w:1)
	// Proof: `VtokenVoting::PendingVotingInfo` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	// Storage: `Tokens::TotalIssuance` (r:1 w:0)
	// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	// Storage: `VtokenMinting::TokenPool` (r:1 w:0)
	// Proof: `VtokenMinting::TokenPool` (`max_values`: None, `max_size`: Some(38), added: 2513, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::ReferendumInfoFor` (r:1 w:1)
	// Proof: `VtokenVoting::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:1 w:1)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::VotingFor` (r:1 w:1)
	// Proof: `VtokenVoting::VotingFor` (`max_values`: None, `max_size`: Some(18271), added: 20746, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::ClassLocksFor` (r:1 w:1)
	// Proof: `VtokenVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(7722), added: 10197, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Locks` (r:1 w:1)
	// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1271), added: 3746, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `VtokenVoting::VoteCapRatio` (r:1 w:0)
	// Proof: `VtokenVoting::VoteCapRatio` (`max_values`: None, `max_size`: Some(26), added: 2501, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::DelegatorVotes` (r:1 w:0)
	// Proof: `VtokenVoting::DelegatorVotes` (`max_values`: None, `max_size`: Some(5136), added: 7611, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::Delegators` (r:1 w:0)
	// Proof: `VtokenVoting::Delegators` (`max_values`: None, `max_size`: Some(224), added: 2699, mode: `MaxEncodedLen`)
	// Storage: `Slp::DelegatorsIndex2Multilocation` (r:1 w:0)
	// Proof: `Slp::DelegatorsIndex2Multilocation` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Slp::DelegatorLedgers` (r:1 w:0)
	// Proof: `Slp::DelegatorLedgers` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `VtokenVoting::PendingDelegatorVotes` (r:1 w:1)
	// Proof: `VtokenVoting::PendingDelegatorVotes` (`max_values`: None, `max_size`: Some(5136), added: 7611, mode: `MaxEncodedLen`)
	// Storage: `XcmInterface::XcmWeightAndFee` (r:1 w:0)
	// Proof: `XcmInterface::XcmWeightAndFee` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::QueryCounter` (r:1 w:1)
	// Proof: `PolkadotXcm::QueryCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `PolkadotXcm::Queries` (r:0 w:1)
	// Proof: `PolkadotXcm::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5567`
		//  Estimated: `21736`
		// Minimum execution time: 95_740 nanoseconds.
		Weight::from_parts(101_121_000, 21736)
			.saturating_add(T::DbWeight::get().reads(19))
			.saturating_add(T::DbWeight::get().writes(9))
	}
	// Storage: `VtokenVoting::ReferendumInfoFor` (r:1 w:0)
	// Proof: `VtokenVoting::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::PendingVotingInfo` (r:1 w:0)
	// Proof: `VtokenVoting::PendingVotingInfo` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::VotingFor` (r:1 w:1)
	// Proof: `VtokenVoting::VotingFor` (`max_values`: None, `max_size`: Some(18271), added: 20746, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::VoteLockingPeriod` (r:1 w:0)
	// Proof: `VtokenVoting::VoteLockingPeriod` (`max_values`: None, `max_size`: Some(26), added: 2501, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `VtokenVoting::ClassLocksFor` (r:1 w:1)
	// Proof: `VtokenVoting::ClassLocksFor` (`max_values`: None, `max_size`: Some(7722), added: 10197, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Locks` (r:1 w:1)
	// Proof: `Tokens::Locks` (`max_values`: None, `max_size`: Some(1271), added: 3746, mode: `MaxEncodedLen`)
	// Storage: `Tokens::Accounts` (r:1 w:1)
	// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(118), added: 2593, mode: `MaxEncodedLen`)
	// Storage: `AssetRegistry::CurrencyMetadatas` (r:1 w:0)
	// Proof: `AssetRegistry::CurrencyMetadatas` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1548`
		//  Estimated: `21736`
		// Minimum execution time: 48_890 nanoseconds.
		Weight::from_parts(50_241_000, 21736)
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: `VtokenVoting::DelegatorVotes` (r:1 w:0)
	// Proof: `VtokenVoting::DelegatorVotes` (`max_values`: None, `max_size`: Some(5136), added: 7611, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::ReferendumInfoFor` (r:1 w:0)
	// Proof: `VtokenVoting::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `XcmInterface::XcmWeightAndFee` (r:1 w:0)
	// Proof: `XcmInterface::XcmWeightAndFee` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::QueryCounter` (r:1 w:1)
	// Proof: `PolkadotXcm::QueryCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::PendingRemoveDelegatorVote` (r:0 w:1)
	// Proof: `VtokenVoting::PendingRemoveDelegatorVote` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	// Storage: `PolkadotXcm::Queries` (r:0 w:1)
	// Proof: `PolkadotXcm::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_delegator_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1047`
		//  Estimated: `8601`
		// Minimum execution time: 28_447 nanoseconds.
		Weight::from_parts(29_544_000, 8601)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: `VtokenVoting::ReferendumInfoFor` (r:1 w:1)
	// Proof: `VtokenVoting::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::ValidationData` (r:1 w:0)
	// Proof: `ParachainSystem::ValidationData` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::LastRelayChainBlockNumber` (r:1 w:0)
	// Proof: `ParachainSystem::LastRelayChainBlockNumber` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn kill_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `468`
		//  Estimated: `3553`
		// Minimum execution time: 10_898 nanoseconds.
		Weight::from_parts(11_442_000, 3553)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Slp::DelegatorsIndex2Multilocation` (r:1 w:0)
	// Proof: `Slp::DelegatorsIndex2Multilocation` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `VtokenVoting::Delegators` (r:1 w:1)
	// Proof: `VtokenVoting::Delegators` (`max_values`: None, `max_size`: Some(224), added: 2699, mode: `MaxEncodedLen`)
	fn add_delegator() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `411`
		//  Estimated: `3876`
		// Minimum execution time: 11_726 nanoseconds.
		Weight::from_parts(12_309_000, 3876)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `VtokenVoting::ReferendumInfoFor` (r:1 w:1)
	// Proof: `VtokenVoting::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(88), added: 2563, mode: `MaxEncodedLen`)
	fn set_referendum_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `386`
		//  Estimated: `3553`
		// Minimum execution time: 9_054 nanoseconds.
		Weight::from_parts(9_552_000, 3553)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `VtokenVoting::UndecidingTimeout` (r:0 w:1)
	// Proof: `VtokenVoting::UndecidingTimeout` (`max_values`: None, `max_size`: Some(26), added: 2501, mode: `MaxEncodedLen`)
	fn set_undeciding_timeout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_502 nanoseconds.
		Weight::from_parts(3_769_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `VtokenVoting::VoteLockingPeriod` (r:0 w:1)
	// Proof: `VtokenVoting::VoteLockingPeriod` (`max_values`: None, `max_size`: Some(26), added: 2501, mode: `MaxEncodedLen`)
	fn set_vote_locking_period() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_578 nanoseconds.
		Weight::from_parts(3_986_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `VtokenVoting::PendingVotingInfo` (r:1 w:0)
	// Proof: `VtokenVoting::PendingVotingInfo` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	// Storage: `VtokenVoting::PendingReferendumInfo` (r:1 w:0)
	// Proof: `VtokenVoting::PendingReferendumInfo` (`max_values`: None, `max_size`: Some(34), added: 2509, mode: `MaxEncodedLen`)
	fn notify_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `426`
		//  Estimated: `3599`
		// Minimum execution time: 9_486 nanoseconds.
		Weight::from_parts(9_857_000, 3599)
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: `VtokenVoting::PendingRemoveDelegatorVote` (r:1 w:0)
	// Proof: `VtokenVoting::PendingRemoveDelegatorVote` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn notify_remove_delegator_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `382`
		//  Estimated: `3501`
		// Minimum execution time: 7_572 nanoseconds.
		Weight::from_parts(7_956_000, 3501)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: `VtokenVoting::VoteCapRatio` (r:0 w:1)
	// Proof: `VtokenVoting::VoteCapRatio` (`max_values`: None, `max_size`: Some(26), added: 2501, mode: `MaxEncodedLen`)
	fn set_vote_cap_ratio() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_719 nanoseconds.
		Weight::from_parts(3_888_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
}

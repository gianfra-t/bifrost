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

use crate::*;
use bifrost_primitives::{CurrencyId, DerivativeIndex};
use frame_support::pallet_prelude::*;
use xcm::v4::Location;

use crate::{pallet::Error, traits::*};

/// VotingAgent implementation for Bifrost
pub struct BifrostAgent<T: pallet::Config> {
	vtoken: CurrencyIdOf<T>,
	location: Location,
}

impl<T: pallet::Config> BifrostAgent<T> {
	pub fn new(vtoken: CurrencyId) -> Result<Self, Error<T>> {
		if cfg!(feature = "polkadot") {
			let location = Pallet::<T>::convert_vtoken_to_dest_location(vtoken)?;
			Ok(Self { vtoken, location })
		} else {
			Err(Error::<T>::VTokenNotSupport)
		}
	}
}

impl<T: Config> VotingAgent<T> for BifrostAgent<T> {
	fn vtoken(&self) -> CurrencyIdOf<T> {
		self.vtoken
	}

	fn location(&self) -> Location {
		self.location.clone()
	}

	fn delegate_vote(
		&self,
		who: AccountIdOf<T>,
		vtoken: CurrencyIdOf<T>,
		poll_index: PollIndexOf<T>,
		_submitted: bool,
		new_delegator_votes: Vec<(DerivativeIndex, AccountVote<BalanceOf<T>>)>,
		maybe_old_vote: Option<(AccountVote<BalanceOf<T>>, BalanceOf<T>)>,
	) -> DispatchResult {
		let derivative_index = new_delegator_votes[0].0;
		let vote_calls = new_delegator_votes
			.iter()
			.map(|(_derivative_index, vote)| {
				pallet_conviction_voting::Call::<T>::vote {
					poll_index,
					vote: Pallet::<T>::transfer(*vote),
				}
				.into()
			})
			.collect::<Vec<<T as Config>::RuntimeCall>>();

		let vote_call = if vote_calls.len() == 1 {
			vote_calls.into_iter().nth(0).ok_or(Error::<T>::NoData)?
		} else {
			return Err(Error::<T>::NoPermissionYet.into());
		};

		let token = CurrencyId::to_token(&vtoken).map_err(|_| Error::<T>::NoData)?;
		let delegator: AccountIdOf<T> =
			T::DerivativeAccount::get_account_id(token, derivative_index)
				.ok_or(Error::<T>::NoData)?;
		let origin: <T as pallet::Config>::PalletsOrigin = RawOrigin::Signed(delegator).into();
		let success = vote_call.dispatch(origin.into()).is_ok();
		Pallet::<T>::handle_vote_result(
			success,
			who,
			vtoken,
			poll_index,
			maybe_old_vote,
			derivative_index,
		)?;

		if success {
			Ok(())
		} else {
			Err(Error::<T>::InvalidCallDispatch.into())
		}
	}

	fn vote_call_encode(
		&self,
		_new_delegator_votes: Vec<(DerivativeIndex, AccountVote<BalanceOf<T>>)>,
		_poll_index: PollIndexOf<T>,
		_derivative_index: DerivativeIndex,
	) -> Result<Vec<u8>, Error<T>> {
		Err(Error::<T>::VTokenNotSupport)
	}

	fn delegate_remove_delegator_vote(
		&self,
		vtoken: CurrencyIdOf<T>,
		poll_index: PollIndexOf<T>,
		class: PollClassOf<T>,
		derivative_index: DerivativeIndex,
	) -> DispatchResult {
		let call: <T as Config>::RuntimeCall = pallet_conviction_voting::Call::<T>::remove_vote {
			class: Some(class),
			index: poll_index,
		}
		.into();

		let token = CurrencyId::to_token(&vtoken).map_err(|_| Error::<T>::NoData)?;
		let delegator: AccountIdOf<T> =
			T::DerivativeAccount::get_account_id(token, derivative_index)
				.ok_or(Error::<T>::NoData)?;
		let origin: <T as pallet::Config>::PalletsOrigin = RawOrigin::Signed(delegator).into();
		let success = call.dispatch(origin.into()).is_ok();

		if success {
			Pallet::<T>::handle_remove_delegator_vote_success(vtoken, poll_index);
			Ok(())
		} else {
			Err(Error::<T>::InvalidCallDispatch.into())
		}
	}

	fn remove_delegator_vote_call_encode(
		&self,
		_class: PollClassOf<T>,
		_poll_index: PollIndexOf<T>,
		_derivative_index: DerivativeIndex,
	) -> Result<Vec<u8>, Error<T>> {
		Err(Error::<T>::VTokenNotSupport)
	}
}
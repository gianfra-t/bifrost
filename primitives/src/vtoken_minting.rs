use frame_support::{
	dispatch::DispatchResult,
	pallet_prelude::{ConstU32, Decode, Encode, MaxEncodedLen, TypeInfo},
	BoundedVec,
};
use sp_core::H160;
use sp_runtime::{traits::Zero, DispatchError, RuntimeDebug};

/// Delegator account
#[derive(Encode, Decode, MaxEncodedLen, Clone, Copy, Debug, PartialEq, Eq, TypeInfo)]
pub enum RedeemCreator<Account> {
	/// Substrate account
	Substrate(Account),
	/// Ethereum address.
	Ethereum(H160),
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum RedeemTo<Account> {
	/// Native chain.
	Native(Account),
	/// Astar chain.
	Astar(Account),
	/// Moonbeam chain.
	Moonbeam(H160),
	/// Hydradx chain.
	Hydradx(Account),
	/// Interlay chain.
	Interlay(Account),
	/// Manta chain.
	Manta(Account),
}

pub trait VtokenMintingInterface<AccountId, CurrencyId, Balance> {
	fn mint(
		exchanger: AccountId,
		token_id: CurrencyId,
		token_amount: Balance,
		remark: BoundedVec<u8, ConstU32<32>>,
		channel_id: Option<u32>,
	) -> Result<Balance, DispatchError>;
	fn redeem(
		exchanger: AccountId,
		vtoken_id: CurrencyId,
		vtoken_amount: Balance,
	) -> DispatchResult;
	fn slpx_redeem(
		redeem_creator: RedeemCreator<AccountId>,
		v_currency_id: CurrencyId,
		v_currency_amount: Balance,
		redeem_to: RedeemTo<AccountId>,
	) -> DispatchResult;
	fn get_v_currency_amount_by_currency_amount(
		token_id: CurrencyId,
		vtoken_id: CurrencyId,
		token_amount: Balance,
	) -> Result<Balance, DispatchError>;
	fn get_currency_amount_by_v_currency_amount(
		token_id: CurrencyId,
		vtoken_id: CurrencyId,
		vtoken_amount: Balance,
	) -> Result<Balance, DispatchError>;
	fn total_stake_amount(currency_id: CurrencyId) -> Balance;
	fn get_moonbeam_parachain_id() -> u32;
}

impl<AccountId, CurrencyId, Balance: Zero> VtokenMintingInterface<AccountId, CurrencyId, Balance>
	for ()
{
	fn mint(
		_exchanger: AccountId,
		_token_id: CurrencyId,
		_token_amount: Balance,
		_remark: BoundedVec<u8, ConstU32<32>>,
		_channel_id: Option<u32>,
	) -> Result<Balance, DispatchError> {
		Ok(Zero::zero())
	}

	fn redeem(
		_exchanger: AccountId,
		_vtoken_id: CurrencyId,
		_vtoken_amount: Balance,
	) -> DispatchResult {
		Ok(().into())
	}

	fn slpx_redeem(
		_exchanger: RedeemCreator<AccountId>,
		_vtoken_id: CurrencyId,
		_vtoken_amount: Balance,
		_redeem_type: RedeemTo<AccountId>,
	) -> DispatchResult {
		Ok(().into())
	}

	fn get_v_currency_amount_by_currency_amount(
		_token_id: CurrencyId,
		_vtoken_id: CurrencyId,
		_token_amount: Balance,
	) -> Result<Balance, DispatchError> {
		Ok(Zero::zero())
	}

	fn get_currency_amount_by_v_currency_amount(
		_token_id: CurrencyId,
		_vtoken_id: CurrencyId,
		_vtoken_amount: Balance,
	) -> Result<Balance, DispatchError> {
		Ok(Zero::zero())
	}

	fn total_stake_amount(_currency_id: CurrencyId) -> Balance {
		Zero::zero()
	}

	fn get_moonbeam_parachain_id() -> u32 {
		0
	}
}

/// The interface to call VtokenMinting module functions.
pub trait VtokenMintingOperator<CurrencyId, Balance, AccountId, TimeUnit> {
	/// Get the currency tokenpool amount.
	fn total_stake_amount(currency_id: CurrencyId) -> Balance;

	/// Increase the token amount for the storage "token_pool" in the VtokenMining module.
	fn increase_token_pool(currency_id: CurrencyId, token_amount: Balance) -> DispatchResult;

	/// Decrease the token amount for the storage "token_pool" in the VtokenMining module.
	fn decrease_token_pool(currency_id: CurrencyId, token_amount: Balance) -> DispatchResult;

	/// Update the ongoing era for a CurrencyId.
	fn update_ongoing_time_unit(currency_id: CurrencyId, time_unit: TimeUnit) -> DispatchResult;

	/// Get the current era of a CurrencyId.
	fn get_ongoing_time_unit(currency_id: CurrencyId) -> Option<TimeUnit>;

	/// Get currency Entrance and Exit accounts.【entrance_account, exit_account】
	fn get_entrance_and_exit_accounts() -> (AccountId, AccountId);

	fn get_moonbeam_parachain_id() -> u32;
}

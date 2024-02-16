#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

use frame_support::{
	traits::{
	Currency, LockIdentifier, LockableCurrency, WithdrawReasons, ReservableCurrency, 
	ExistenceRequirement::KeepAlive, OnUnbalanced
	},
	PalletId,
};

use frame_support::sp_runtime::{
	traits::{
	AccountIdConversion, CheckedDiv, CheckedMul, StaticLookup, CheckedAdd, Zero,
	},
	Saturating,
};

type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

pub type BalanceOf<T> = 
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	const LETTING_ID: LockIdentifier = *b"stklttgg";

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config 
		+ pallet_nfts::Config 
		+ pallet_whitelist::Config 
		+ pallet_nft_marketplace::Config 
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// The lockable currency type.
		type Currency: Currency<Self::AccountId>
			+ LockableCurrency<Self::AccountId, Moment = BlockNumberFor<Self>>
			+ ReservableCurrency<Self::AccountId>;
		/// The property management's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		/// Minimum amount that should be left on letting agent account.
		#[pallet::constant]
		type MinimumRemainingAmount: Get<BalanceOf<Self>>;
		/// Origin who can set a new letting agent.
		type AgentOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// The minimum amount of a letting agent that has to be staked.
		type MinStakingAmount: Get<BalanceOf<Self>>;
		/// Collection id type from pallet nfts.
		type CollectionId: IsType<<Self as pallet_nft_marketplace::Config>::CollectionId>
			+ Parameter
			+ From<u32>
			+ Default
			+ Ord
			+ Copy
			+ MaxEncodedLen
			+ Encode;

		/// Item id type from pallet nfts.
		type ItemId: IsType<<Self as pallet_nft_marketplace::Config>::ItemId>
			+ Parameter
			+ From<u32>
			+ Ord
			+ Copy
			+ MaxEncodedLen
			+ Encode;
		
		/// Handler for the unbalanced reduction when slashing a letting agent.
		type Slash: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// The maximum amount of properties that can be assigned to a letting agent.
		#[pallet::constant]
		type MaxProperties: Get<u32>;
	}

	pub type CollectionId<T> = <T as Config>::CollectionId;
	pub type ItemId<T> = <T as Config>::ItemId;

	/// Mapping from the real estate object to the letting agent
	#[pallet::storage]
	#[pallet::getter(fn letting_storage)]
	pub type LettingStorage<T> = StorageMap<
		_, 
		Blake2_128Concat,
		u32,
		AccountIdOf<T>,
		OptionQuery,
	>;

	/// Mapping from account to currently stored balance.
	#[pallet::storage]
	#[pallet::getter(fn stored_funds)]
	pub type StoredFunds<T> = StorageMap<
		_,
		Blake2_128Concat,
		AccountIdOf<T>,
		BalanceOf<T>,
		ValueQuery,
	>;

	/// Mapping from letting agent to the properties.
	#[pallet::storage]
	#[pallet::getter(fn letting_agent_properties)]
	pub type LettingAgentPropoerties<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		AccountIdOf<T>,
		BoundedVec<u32, T::MaxProperties>,
		ValueQuery,
	>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new letting agent got set.
		LettingAgentSet { 
			property_index: u32,
			who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Error by convertion to balance type.
		ConversionError,
		/// Error by multiplying a number.
		MultiplyError,
		ArithmeticOverflow,
		/// One person in the list is not an owner of the property.
		UserNotPropertyOwner,
		/// The call has no funds stored.
		UserHasNoFundsStored,
		/// The pallet has not enough funds.
		NotEnoughFunds,
		/// The letting agent already has too many assigned properties.
		TooManyAssignedProperties,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Sets the letting agent for an object real estate object.
		/// - Parameter are origin, collection id, nft id and account id.
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn assign_to_letting_agent(
			origin: OriginFor<T>, 
			property_id: u32,
			letting_agent: AccountIdOf<T>,
		) -> DispatchResult {
			T::AgentOrigin::ensure_origin(origin)?;
			LettingStorage::<T>::insert(property_id, letting_agent.clone());
			LettingAgentPropoerties::<T>::try_mutate(letting_agent.clone(), |keys| {
				keys.try_push(property_id).map_err(|_| Error::<T>::TooManyAssignedProperties)?;
				Ok::<(), DispatchError>(())
			})?;

 			Self::deposit_event(Event::<T>::LettingAgentSet {
				property_index: property_id,
				who: letting_agent,
			}); 
			Ok(())
		}

		/// Lets the letting agent stake his funds that could be slashed if he acts
		/// malicious
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn letting_agent_deposit(origin: OriginFor<T>) -> DispatchResult {
			let staker = ensure_signed(origin)?;
			<T as pallet::Config>::Currency::reserve(&staker, <T as Config>::MinStakingAmount::get())?;
			Ok(())
		}

		/// Lets someone slashing the letting agent.
		/// todo!
		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn slash_letting_agent(origin: OriginFor<T>, amount: BalanceOf<T>, letting_agent: AccountIdOf<T>) -> DispatchResult {
			let staker = ensure_signed(origin)?;
			<T as pallet::Config>::Currency::remove_lock(LETTING_ID, &letting_agent);
			<T as pallet::Config>::Slash::on_unbalanced(<T as pallet::Config>::Currency::slash_reserved(&letting_agent, amount).0);
			Ok(())
		}

		///	The letting agent can distribute the rental income.
		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn distribute_income(origin: OriginFor<T>, asset_id: u32, amount: BalanceOf<T>) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			<T as pallet::Config>::Currency::transfer(
				&origin,
				&Self::account_id(),
				amount.checked_mul(&Self::u64_to_balance_option(1/* 000000000000 */)?)
					.ok_or(Error::<T>::MultiplyError)?,
				KeepAlive,
			)
			.map_err(|_| Error::<T>::NotEnoughFunds)?;
			let owner_list = pallet_nft_marketplace::Pallet::<T>::property_owner(asset_id);
			for owner in owner_list {
				let token_amount = pallet_nft_marketplace::Pallet::<T>::property_owner_token(asset_id, owner.clone());
				let amount_for_owner = Self::u64_to_balance_option(token_amount as u64)?*amount/Self::u64_to_balance_option(100)?;
				let mut old_funds = Self::stored_funds(owner.clone());
				old_funds = old_funds.checked_add(&amount_for_owner).ok_or(Error::<T>::ArithmeticOverflow)?;
				StoredFunds::<T>::insert(owner, old_funds);
			};
			Ok(())
		}

		/// A property owner can withdraw the collected funds.
		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn withdraw_funds(origin: OriginFor<T>) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			ensure!(!Self::stored_funds(origin.clone()).is_zero(), Error::<T>::UserHasNoFundsStored);
			let user_funds = StoredFunds::<T>::take(origin.clone());
			<T as pallet::Config>::Currency::transfer(
				&Self::account_id(), 
				&origin, 
				user_funds, 
				KeepAlive,
			)
			.map_err(|_| Error::<T>::NotEnoughFunds)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Get the account id of the pallet
		pub fn account_id() -> AccountIdOf<T> {
			<T as pallet::Config>::PalletId::get().into_account_truncating()
		}
		/// Converts a u64 to a balance.
		pub fn u64_to_balance_option(input: u64) -> Result<BalanceOf<T>, Error<T>> {
			input.try_into().map_err(|_| Error::<T>::ConversionError)
		}
	}
}
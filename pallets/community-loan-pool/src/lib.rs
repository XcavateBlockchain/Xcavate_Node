#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
/// <https://docs.substrate.io/reference/frame-pallets/>
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
pub mod weights;
pub use weights::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::sp_runtime::{
	traits::{AccountIdConversion, StaticLookup, Zero},
	Permill, RuntimeDebug,
};

use frame_support::sp_runtime::Percent;

use frame_support::{
	sp_runtime,
//	inherent::Vec,
	pallet_prelude::*,
	traits::{Currency, Get, OnUnbalanced, ReservableCurrency, UnixTime, GenesisBuild, ExistenceRequirement::KeepAlive},
	PalletId,
};
use sp_std::vec::Vec;

use sp_std::prelude::*;

type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

pub type LoanApy = u64;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

pub type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::PositiveImbalance;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type BoundedProposedMilestones<T> =
    BoundedVec<ProposedMilestone, <T as Config>::MaxMilestonesPerProject>;

#[cfg(feature = "runtime-benchmarks")]
pub struct NftHelper;

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct ProposedMilestone {
	pub percentage_to_unlock: Percent,
}

#[cfg(feature = "runtime-benchmarks")]
pub trait BenchmarkHelper<CollectionId, ItemId> {
	fn to_collection(i: u32) -> CollectionId;
	fn to_nft(i: u32) -> ItemId;
}

#[cfg(feature = "runtime-benchmarks")]
impl<CollectionId: From<u32>, ItemId: From<u32>> BenchmarkHelper<CollectionId, ItemId>
	for NftHelper
{
	fn to_collection(i: u32) -> CollectionId {
		i.into()
	}
	fn to_nft(i: u32) -> ItemId {
		i.into()
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::sp_runtime::Saturating;
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;

	pub type ProposalIndex = u32;
	pub type LoanIndex = u32;

	/// A loan proposal
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Proposal<Balance, BlockNumber, T: Config> {
		proposer: AccountIdOf<T>,
		amount: Balance,
		milestones: BoundedProposedMilestones<T>,
		beneficiary: AccountIdOf<T>,
		bond: Balance,
		created_at: BlockNumber,
	}

	/// loan info
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct LoanInfo<Balance, CollectionId, ItemId, T: Config> {
		pub borrower: AccountIdOf<T>,
		pub total_amount: Balance,
		pub available_amount: Balance,
		pub borrowed_amount: Balance,
		pub milestones: BoundedProposedMilestones<T>,
		pub collection_id: CollectionId,
		pub item_id: ItemId,
		pub loan_apy: LoanApy,
		pub last_timestamp: u64,
	}

	/// AccountId storage
	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub struct PalletIdStorage<T: Config>{
		pallet_id: AccountIdOf<T>,
	}

	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub enum Vote {
		Yes,
		No,
	}

	#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
	#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
	pub struct VoteStats {
		pub yes_votes: u64,
		pub no_votes: u64,
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_uniques::Config
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The currency type.
		type Currency: Currency<AccountIdOf<Self>> + ReservableCurrency<AccountIdOf<Self>>;

		/// Origin from which rejections must come.
		type RejectOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin from which approves must come.
		type ApproveOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin who can add or remove committee members
		type CommitteeOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Origin who can delete loans
		type DeleteOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		/// Fraction of a proposal's value that should be bonded in order to place the proposal.
		/// An accepted proposal gets these back. A rejected proposal does not.
		#[pallet::constant]
		type ProposalBond: Get<Permill>;

		/// Minimum amount of funds that should be placed in a deposit for making a proposal.
		#[pallet::constant]
		type ProposalBondMinimum: Get<BalanceOf<Self>>;

		/// Maximum amount of funds that should be placed in a deposit for making a proposal.
		#[pallet::constant]
		type ProposalBondMaximum: Get<Option<BalanceOf<Self>>>;

		/// The treasury's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Handler for the unbalanced decrease when slashing for a rejected proposal or bounty.
		type OnSlash: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// The maximum amount of loans that can run at the same time.
		#[pallet::constant]
		type MaxOngoingLoans: Get<u32>;

		/// lose coupling of pallet timestamp
		type TimeProvider: UnixTime;

		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		#[cfg(feature = "runtime-benchmarks")]
		type Helper: crate::BenchmarkHelper<Self::CollectionId, Self::ItemId>;

		/// The amount of time given to vote for a proposal
		type VotingTime: Get<BlockNumberFor<Self>>;

		/// The maximum amount of commitee members
		type MaxCommitteeMembers: Get<u32>;

		type MaxMilestonesPerProject: Get<u32>;
	}

	/* 	#[pallet::storage]
	#[pallet::getter(fn loan_pool_account)]
	pub(super) type LoanPoolAccount<T> = StorageValue<_, PalletIdStorage<T::AccountId>, ValueQuery>; */

	/// Vec of admins who are able to vote
	#[pallet::storage]
	#[pallet::getter(fn voting_committee)]
	pub(super) type VotingCommittee<T: Config> =
		StorageValue<_, BoundedVec<AccountIdOf<T>, T::MaxOngoingLoans>, ValueQuery>;

	/// Number of proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn loan_count)]
	pub(super) type LoanCount<T> = StorageValue<_, ProposalIndex, ValueQuery>;

	/// Number of proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn proposal_count)]
	pub(super) type ProposalCount<T> = StorageValue<_, ProposalIndex, ValueQuery>;

	/// Number of milestone proposal that have benn made.
	#[pallet::storage]
	#[pallet::getter(fn milestone_proposal_count)]
	pub(super) type MilestoneProposalCount<T> = StorageValue<_, ProposalIndex, ValueQuery>;

	/// Proposals with won the voting
	#[pallet::storage]
	#[pallet::getter(fn evaluated_loans)]
	pub(super) type EvaluatedLoans<T: Config> =
		StorageValue<_, BoundedVec<ProposalIndex, T::MaxOngoingLoans>, ValueQuery>;

	/// Total amount of loan funds
	#[pallet::storage]
	#[pallet::getter(fn total_loan_amount)]
	pub(super) type TotalLoanAmount<T> = StorageValue<_, u64, ValueQuery>;

	/// All currently ongoing loans
	#[pallet::storage]
	#[pallet::getter(fn ongoing_loans)]
	pub(super) type OngoingLoans<T: Config> =
		StorageValue<_, BoundedVec<ProposalIndex, T::MaxOngoingLoans>, ValueQuery>;

	/// Proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub(super) type Proposals<T> = StorageMap<
		_,
		Twox64Concat,
		ProposalIndex,
		Proposal<BalanceOf<T>, BlockNumberFor<T>, T>,
		OptionQuery,
	>;

	/// Milestone proposal that has been made.
	#[pallet::storage]
	#[pallet::getter(fn milestone_proposals)]
	pub(super) type MilestoneProposals<T> = StorageMap<
		_,
		Twox64Concat,
		ProposalIndex,
		LoanIndex,
		OptionQuery,
	>;

	/// Mapping of ongoing loans
	#[pallet::storage]
	#[pallet::getter(fn loans)]
	pub(super) type Loans<T: Config> = StorageMap<
		_,
		Twox64Concat,
		LoanIndex,
		LoanInfo<BalanceOf<T>, T::CollectionId, T::ItemId, T>,
		OptionQuery,
	>;

	/// Mapping of ongoing votes
	#[pallet::storage]
	#[pallet::getter(fn ongoing_votes)]
	pub(super) type OngoingVotes<T: Config> =
		StorageMap<_, Twox64Concat, ProposalIndex, VoteStats, OptionQuery>;

	/// Mapping of user who voted for a proposal
	#[pallet::storage]
	#[pallet::getter(fn user_votes)]
	pub(super) type UserVotes<T: Config> =
		StorageMap<_, Twox64Concat, (ProposalIndex, AccountIdOf<T>), Vote, OptionQuery>;

	/// Stores the project keys and round types ending on a given block
	#[pallet::storage]
	pub type RoundsExpiring<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BlockNumberFor<T>,
		BoundedVec<ProposalIndex, T::MaxOngoingLoans>,
		ValueQuery,
	>;

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		#[serde(skip)]
		_config: sp_std::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			// Create Treasury account
			let account_id = <Pallet<T>>::account_id();
			let min = <T as pallet::Config>::Currency::minimum_balance();
			if <T as pallet::Config>::Currency::free_balance(&account_id) < min {
				let _ = <T as pallet::Config>::Currency::make_free_balance_be(&account_id, min);
			}
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Proposer's balance is too low
		InsufficientProposersBalance,
		/// Loan pool's balance is too low
		InsufficientLoanPoolBalance,
		/// No proposal index
		InvalidIndex,
		/// The caller doesn't have enough permission
		InsufficientPermission,
		/// Max amount of ongoing loan reached
		TooManyLoans,
		/// User has already voted
		AlreadyVoted,
		/// Loan got not approved
		NotApproved,
		/// The account is already a member in the voting committee
		AlreadyMember,
		/// There are already enough committee members
		TooManyMembers,
		/// There are not enough funds available in the loan
		NotEnoughFundsToWithdraw,
		/// The loan is still ongoing
		LoanStillOngoing,
		/// All milestones has been accomplished
		NoMilestonesLeft,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// New Proposal
		Proposed { proposal_index: ProposalIndex },
		/// Proposal has been approved
		Approved { proposal_index: ProposalIndex },
		/// Proposal has been rejected
		Rejected { proposal_index: ProposalIndex },
		/// Loan has been deleted
		Deleted { loan_index: LoanIndex },
		/// Charged APY
		ApyCharged { loan_index: LoanIndex },
		/// Loan has been updated
		LoanUpdated { loan_index: LoanIndex },
		/// User withdraw money
		Withdraw { loan_index: LoanIndex, amount: BalanceOf<T>}

	}

	// Work in progress, to be included in the future
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: frame_system::pallet_prelude::BlockNumberFor<T>) -> Weight {
			let mut weight = T::DbWeight::get().reads_writes(1, 1);

			let ended_votings = RoundsExpiring::<T>::take(n);

			ended_votings.iter().for_each(|item| {
				weight = weight.saturating_add(T::DbWeight::get().reads_writes(1, 1));
				let voting_result = <OngoingVotes<T>>::take(item);
				if let Some(voting_result) = voting_result {
					if voting_result.yes_votes > voting_result.no_votes {
						EvaluatedLoans::<T>::try_append(item).unwrap_or_default();
					} else {
						Self::reject_loan_proposal(*item).unwrap_or_default();
					}
					OngoingVotes::<T>::remove(item);
				}			
			});
			weight
		}

		fn on_finalize(_n: frame_system::pallet_prelude::BlockNumberFor<T>) {
			//let block = n.saturated_into::<u64>();
			//if block % 10 == 0 {
			Self::charge_apy().unwrap_or_default();
			//}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Apply for a loan. A deposit amount is reserved
		/// and slashed if the proposal is rejected. It is returned once the proposal is awarded.
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::propose())]
		pub fn propose(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			proposed_milestones: BoundedProposedMilestones<T>,
			beneficiary: AccountIdLookupOf<T>,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let beneficiary = T::Lookup::lookup(beneficiary)?;
			let proposal_index = Self::proposal_count() + 1;
			let bond = Self::calculate_bond(amount);
			<T as pallet::Config>::Currency::reserve(&origin, bond)
				.map_err(|_| Error::<T>::InsufficientProposersBalance)?;
			let current_block_number = <frame_system::Pallet<T>>::block_number();
			let expiry_block =
				current_block_number.saturating_add(<T as Config>::VotingTime::get());

			RoundsExpiring::<T>::try_mutate(expiry_block, |keys| {
				keys.try_push(proposal_index).map_err(|_| Error::<T>::TooManyLoans)?;
				Ok::<(), DispatchError>(())
			})?;

			let proposal = Proposal {
				proposer: origin,
				amount,
				milestones: proposed_milestones,
				beneficiary,
				bond,
				created_at: current_block_number,
			};
			let vote_stats = VoteStats { yes_votes: 0, no_votes: 0 };
			OngoingVotes::<T>::insert(proposal_index, vote_stats);
			Proposals::<T>::insert(proposal_index, proposal);
			ProposalCount::<T>::put(proposal_index);

			Self::deposit_event(Event::Proposed { proposal_index });
			Ok(())
		}

		/// Reject a proposed spend. The original deposit will be slashed.
		///
		/// May only be called from `T::RejectOrigin`.
		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::reject_proposal())]
		pub fn reject_proposal(
			origin: OriginFor<T>,
			proposal_index: ProposalIndex,
		) -> DispatchResult {
			T::RejectOrigin::ensure_origin(origin)?;
			Self::reject_loan_proposal(proposal_index)?;
			OngoingVotes::<T>::remove(proposal_index);
			let mut evaluated_loans = Self::evaluated_loans();
			let index = evaluated_loans.iter().position(|x| *x == proposal_index);
			if index.is_some() {
				evaluated_loans.remove(index.unwrap());
			};
			Ok(())
		}

		/// Approve a proposed spend. The original deposit will be released.
		/// It will call the create_loan function in the contract and deposit the loan amount in
		/// the contract.
		///
		/// May only be called from `T::ApproveOrigin`.

		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn approve_proposal(
			origin: OriginFor<T>,
			proposal_index: ProposalIndex,
			collection_id: T::CollectionId,
			collateral_price: BalanceOf<T>,
			item_id: T::ItemId,
			loan_apy: LoanApy,
			admin: AccountIdOf<T>,
		) -> DispatchResult {
			let _signer = ensure_signed(origin.clone())?;
			//T::ApproveOrigin::ensure_origin(origin.clone())?;
			let proposal = <Proposals<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let mut evaluated_loans = Self::evaluated_loans();
			ensure!(evaluated_loans.contains(&proposal_index), Error::<T>::NotApproved);
			let index = evaluated_loans.iter().position(|x| *x == proposal_index).unwrap();
			evaluated_loans.remove(index);

			EvaluatedLoans::<T>::put(evaluated_loans);
			let err_amount =
				<T as pallet::Config>::Currency::unreserve(&proposal.proposer, proposal.bond);
			debug_assert!(err_amount.is_zero());
			let user = proposal.beneficiary;
			let value = proposal.amount;
			let mut milestones = proposal.milestones;
			let timestamp = T::TimeProvider::now().as_secs();
			let amount = Self::balance_to_u64(value).unwrap() * milestones[0].percentage_to_unlock.deconstruct() as u64;
			milestones.remove(0);
			let available_amount = Self::u64_to_balance_option(amount).unwrap();
			let loan_info = LoanInfo {
				borrower: user.clone(),
				total_amount: value,
				available_amount,
				borrowed_amount: Default::default(),
				milestones,
				collection_id: collection_id.clone(),
				item_id,
				loan_apy,
				last_timestamp: timestamp,
			};

			let loan_index = Self::loan_count() + 1;

			Loans::<T>::insert(loan_index, loan_info);
			OngoingLoans::<T>::try_append(loan_index).map_err(|_| Error::<T>::TooManyLoans)?;
			// calls the create collection function from the uniques pallet, and set the admin as
			// the admin of the collection
			pallet_uniques::Pallet::<T>::do_create_collection(
				collection_id.clone(),
				admin.clone(),
				admin.clone(),
				T::CollectionDeposit::get(),
				false,
				pallet_uniques::Event::Created {
					creator: admin.clone(),
					owner: admin.clone(),
					collection: collection_id.clone(),
				},
			)?;
			// calls the mint collection function from the uniques pallet, mints a nft and puts
			// the loan contract as the owner
			pallet_uniques::Pallet::<T>::do_mint(collection_id.clone(), item_id, Self::account_id(), |_| Ok(()))?;

			let new_value = Self::total_loan_amount() + Self::balance_to_u64(value).unwrap();
			TotalLoanAmount::<T>::put(new_value);
			Proposals::<T>::remove(proposal_index);
			LoanCount::<T>::put(loan_index);
			Self::deposit_event(Event::<T>::Approved { proposal_index });
			Ok(())
		}

		#[pallet::call_index(99)]
		#[pallet::weight(0)]
		pub fn propose_milestone(origin: OriginFor<T>, loan_id: LoanIndex) -> DispatchResult{
			let origin = ensure_signed(origin)?;
			let loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(loan.milestones.len() > 0, Error::<T>::NoMilestonesLeft);
			let milestone_proposal_index = Self::milestone_proposal_count() + 1;
			MilestoneProposals::<T>::insert(milestone_proposal_index, loan_id);
			MilestoneProposalCount::<T>::put(milestone_proposal_index);
			Ok(())
		}

		/// Delete the loan after the loan is paid back. The collateral nft will be deleted.
		///
		/// May only be called from the loan contract.

		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn delete_loan(origin: OriginFor<T>, loan_id: LoanIndex) -> DispatchResult {
			let signer = ensure_signed(origin.clone())?;
			let loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			//ensure!(signer == loan.contract_account_id, Error::<T>::InsufficientPermission);
			ensure!(loan.borrowed_amount.is_zero(), Error::<T>::LoanStillOngoing);

			let collection_id = loan.collection_id;
			let item_id = loan.item_id;

			pallet_uniques::Pallet::<T>::do_burn(collection_id, item_id, |_, _| Ok(()))?;

			let mut loans = Self::ongoing_loans();
			let index = loans.iter().position(|x| *x == loan_id).unwrap();
			loans.remove(index);

			OngoingLoans::<T>::put(loans);
			Loans::<T>::remove(loan_id);
			Self::deposit_event(Event::<T>::Deleted { loan_index: loan_id });
			Ok(())
		}

		/// Updates the loan after an amount of the loan got repaid.
		///
		/// May only be called from the loan contract.

		#[pallet::call_index(4)]
		#[pallet::weight(0)]
		pub fn withdraw(
			origin: OriginFor<T>,
			loan_id: LoanIndex,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let signer = ensure_signed(origin.clone())?;
			let mut loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			ensure!(signer == loan.borrower, Error::<T>::InsufficientPermission);
			ensure!(amount <= loan.available_amount, Error::<T>::NotEnoughFundsToWithdraw);
			let loan_pallet = Self::account_id();
			let sending_amount = Self::balance_to_u64(amount).unwrap();
			<T as pallet::Config>::Currency::transfer(
				&loan_pallet,
				&signer,
				(sending_amount * 1000000000000).try_into().ok().unwrap(),
				KeepAlive,
			)
			.unwrap_or_default();
			loan.borrowed_amount = loan.borrowed_amount.saturating_add(amount);
			loan.available_amount = loan.available_amount.saturating_sub(amount);
			Loans::<T>::insert(loan_id, loan);
			Self::deposit_event(Event::<T>::Withdraw {loan_index: loan_id, amount});
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(0)]
		pub fn repay(
			origin: OriginFor<T>,
			loan_id: LoanIndex,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let signer = ensure_signed(origin.clone())?;
			let mut loan = <Loans<T>>::take(loan_id).ok_or(Error::<T>::InvalidIndex)?;
			//ensure!(signer == loan.contract_account_id, Error::<T>::InsufficientPermission);
			let loan_pallet = Self::account_id();
			let sending_amount = Self::balance_to_u64(amount).unwrap();
			<T as pallet::Config>::Currency::transfer(
				&signer,
				&loan_pallet,
				(sending_amount * 1000000000000).try_into().ok().unwrap(),
				KeepAlive,
			)
			.unwrap_or_default();
			loan.borrowed_amount = loan.borrowed_amount.saturating_sub(amount);
			Loans::<T>::insert(loan_id, loan);
			let new_value = Self::total_loan_amount() - Self::balance_to_u64(amount).unwrap();
			TotalLoanAmount::<T>::put(new_value);
			Self::deposit_event(Event::<T>::LoanUpdated { loan_index: loan_id });
			Ok(())
		}

		/// Let committee members vote for a proposal
		#[pallet::call_index(6)]
		#[pallet::weight(0)]
		pub fn vote_on_proposal(
			origin: OriginFor<T>,
			proposal_index: ProposalIndex,
			vote: Vote,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let current_members = Self::voting_committee();
			ensure!(current_members.contains(&origin), Error::<T>::InsufficientPermission);
			let mut current_vote =
				<OngoingVotes<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let voted = <UserVotes<T>>::get((proposal_index, origin.clone()));
			ensure!(voted.is_none(), Error::<T>::AlreadyVoted);
			if vote == Vote::Yes {
				current_vote.yes_votes += 1;
			} else {
				current_vote.no_votes += 1;
			};

			UserVotes::<T>::insert((proposal_index, origin), vote);
			OngoingVotes::<T>::insert(proposal_index, current_vote);
			Ok(())
		}

		/// Adding a new address to the vote committee
		#[pallet::call_index(7)]
		#[pallet::weight(0)]
		pub fn add_committee_member(
			origin: OriginFor<T>,
			member: AccountIdOf<T>,
		) -> DispatchResult {
			T::CommitteeOrigin::ensure_origin(origin)?;
			let current_members = Self::voting_committee();
			ensure!(!current_members.contains(&member), Error::<T>::AlreadyMember);
			VotingCommittee::<T>::try_append(member).map_err(|_| Error::<T>::TooManyMembers)?;
			Ok(())
		}
	}

	//** Our helper functions.**//

	impl<T: Config> Pallet<T> {
		pub fn account_id() -> AccountIdOf<T> {
			T::PalletId::get().into_account_truncating()
		}

		fn calculate_bond(value: BalanceOf<T>) -> BalanceOf<T> {
			let mut r = T::ProposalBondMinimum::get().max(T::ProposalBond::get() * value);
			if let Some(m) = T::ProposalBondMaximum::get() {
				r = r.min(m);
			}
			r
		}

		fn reject_loan_proposal(proposal_index: ProposalIndex) -> DispatchResult {
			let proposal = <Proposals<T>>::take(proposal_index).ok_or(Error::<T>::InvalidIndex)?;
			let value = proposal.bond;
			let imbalance =
				<T as pallet::Config>::Currency::slash_reserved(&proposal.proposer, value).0;
			T::OnSlash::on_unbalanced(imbalance);

			Proposals::<T>::remove(proposal_index);

			Self::deposit_event(Event::<T>::Rejected { proposal_index });
			Ok(())
		}

		// Work in progress, to be implmented in the future
		pub fn charge_apy() -> DispatchResult {
			let ongoing_loans = Self::ongoing_loans();
			for i in ongoing_loans {
				let loan_index = i;
				let mut loan = <Loans<T>>::take(loan_index).ok_or(Error::<T>::InvalidIndex)?;
				let current_timestamp = T::TimeProvider::now().as_secs();
				let time_difference = current_timestamp - loan.last_timestamp;
				let loan_amount = Self::balance_to_u64(loan.available_amount + loan.borrowed_amount).unwrap();
				let interests =
					loan_amount * time_difference * loan.loan_apy / 365 / 60 / 60 / 24 / 100;
				let interest_balance = Self::u64_to_balance_option(interests).unwrap();
				loan.borrowed_amount += interest_balance;
				loan.last_timestamp = current_timestamp;
				Loans::<T>::insert(loan_index, loan.clone());
				let new_value = Self::total_loan_amount() + interests;
				TotalLoanAmount::<T>::put(new_value);
				Self::deposit_event(Event::<T>::ApyCharged { loan_index });
			}
			Ok(())
		}

		pub fn balance_to_u64(input: BalanceOf<T>) -> Option<u64> {
			TryInto::<u64>::try_into(input).ok()
		}

		pub fn u64_to_balance_option(input: u64) -> Option<BalanceOf<T>> {
			input.try_into().ok()
		}
	}
}

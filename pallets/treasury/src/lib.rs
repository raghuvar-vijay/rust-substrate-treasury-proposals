#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		PalletId, dispatch::DispatchResult, pallet_prelude::*,
		traits::{ Currency, ReservableCurrency, Imbalance, OnUnbalanced, ExistenceRequirement::AllowDeath },
		sp_runtime::{ Permill, traits::{AccountIdConversion, StaticLookup }
		}
	};
	use frame_system::pallet_prelude::*;
	use primitives::Proposal;

	const PALLET_ID: PalletId = PalletId(*b"Odyssey1");

	pub type ProposalIndex = u32;

	type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::NegativeImbalance;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_memes::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		type ProposalBondMinimum: Get<BalanceOf<Self>>;

		type ProposalBond: Get<Permill>;

		#[pallet::constant]
		type MaxApprovals: Get<u32>;

		type ApproveOrigin: EnsureOrigin<Self::Origin>;

	}

	/// Number of proposals that have been made.
	#[pallet::storage]
	#[pallet::getter(fn proposal_count)]
	pub(super) type ProposalCount<T> = StorageValue<_, ProposalIndex, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub(super) type Proposals<T: Config> = StorageMap<
		_,
		Twox64Concat,
		ProposalIndex,
		Proposal<T::AccountId, BalanceOf<T>, T::Hash>,
		OptionQuery,
	>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Donor has made a charitable donation to the pot
		DonationReceived(T::AccountId, BalanceOf<T>, BalanceOf<T>),
		/// An imbalance from elsewhere in the runtime has been absorbed by the Pot
		ImbalanceAbsorbed(BalanceOf<T>, BalanceOf<T>),
		/// Pot has allocated funds to a cause
		FundsAllocated(T::AccountId, BalanceOf<T>, BalanceOf<T>),

		Proposed(ProposalIndex),

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		DonationFailed,
		InsufficientProposersBalance,
		TooManyApprovals,
		InvalidIndex,
		BondNotExist,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn donate(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let donor = ensure_signed(origin)?;

			T::Currency::transfer(&donor, &Self::account_id(), amount, AllowDeath)
				.map_err(|_| DispatchError::Other("Can't make donation"))?;

			Self::deposit_event(Event::DonationReceived(donor, amount, Self::pot()));
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn allocate(
			origin: OriginFor<T>,
			proposal_id: ProposalIndex,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			ensure!(<Proposals<T>>::contains_key(proposal_id), Error::<T>::InvalidIndex);

			let proposal: Proposal<T::AccountId, BalanceOf<T>, T::Hash> = <Proposals<T>>::get(proposal_id).unwrap();
			// Make the transfer requested
			T::Currency::transfer(&Self::account_id(), &proposal.beneficiary, proposal.value.clone(), AllowDeath)
				.map_err(|_| DispatchError::Other("Can't make allocation"))?;

			T::Currency::unreserve(&proposal.proposer, proposal.bond);

			Self::deposit_event(Event::FundsAllocated(proposal.beneficiary, proposal.value, Self::pot()));
			Ok(().into())
		}

		#[pallet::weight(10_000)]
		pub fn propose_spend(
			origin: OriginFor<T>,
			value: BalanceOf<T>,
			beneficiary: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResult {
			let proposer = ensure_signed(origin)?;
			let beneficiary = T::Lookup::lookup(beneficiary)?;

			let bond = Self::calculate_bond(value);
			T::Currency::reserve(&proposer, bond)
				.map_err(|_| Error::<T>::InsufficientProposersBalance)?;

			let meme_reference_hash = <pallet_memes::Pallet<T>>::get_meme_reference_hash(beneficiary.clone());

			let c = Self::proposal_count();
			<ProposalCount<T>>::put(c + 1);
			<Proposals<T>>::insert(c, Proposal { proposer, value, beneficiary, bond, meme_reference_hash });

			Self::deposit_event(Event::Proposed(c));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn account_id() -> T::AccountId {
			PALLET_ID.into_account()
		}

		fn pot() -> BalanceOf<T> {
			T::Currency::free_balance(&Self::account_id())
		}

		fn calculate_bond(value: BalanceOf<T>) -> BalanceOf<T> {
			T::ProposalBondMinimum::get().max(T::ProposalBond::get() * value)
		}
	}


	impl<T: Config> OnUnbalanced<NegativeImbalanceOf<T>> for Pallet<T> {
		fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<T>) {
			let numeric_amount = amount.peek();

			let _ = T::Currency::resolve_creating(&Self::account_id(), amount);

			Self::deposit_event(Event::ImbalanceAbsorbed(numeric_amount, Self::pot()));
		}
	}
}

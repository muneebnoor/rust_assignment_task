#![cfg_attr(not(feature = "std"), no_std)]

  pub use pallet::*;

  #[frame_support::pallet]
  pub mod pallet {
      use frame_support::pallet_prelude::*;
      use frame_system::pallet_prelude::*;
      use sp_runtime::traits::StaticLookup;
      use sp_std::prelude::*;

      #[pallet::pallet]
      #[pallet::generate_store(pub(super) trait Store)]
      pub struct Pallet<T>(_);

      #[pallet::config]
      pub trait Config: frame_system::Config {
          type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
          /// Root account can only edit clubs
          type AdminOrigin: EnsureOrigin<Self::Origin>;
          /// The minimum length a club name may be.
          #[pallet::constant]
          type MinLength: Get<u32>;
          /// The maximum length a club name may be.
          #[pallet::constant]
          type MaxLength: Get<u32>;
      }

      #[pallet::event]
      #[pallet::generate_deposit(pub(super) fn deposit_event)]
      pub enum Event<T: Config> {
        MemberAdded{target: T::AccountId},
        MemberRemoved{target: T::AccountId}
      }

      #[pallet::error]
      pub enum Error<T> {
        /// A name is too short.
        TooShort,
        /// A name is too long.
        TooLong,
        /// A member is not in any club
        NotInClub,
        /// A member is already part of a club
        AlreadyInClub,
      }

      #[pallet::storage]
      pub(super) type ClubOf<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BoundedVec<u8, T::MaxLength>,
        OptionQuery,
      >;

      #[pallet::call]
      impl<T: Config> Pallet<T> {
          #[pallet::weight(70_000_000)]
          pub fn add_member(
              origin: OriginFor<T>,
              target: <T::Lookup as StaticLookup>::Source,
              club_name: Vec<u8>,
          ) -> DispatchResult {
              T::AdminOrigin::ensure_origin(origin)?;
              
              let bounded_club: BoundedVec<_, _> =
              club_name.try_into().map_err(|()| Error::<T>::TooLong)?;
              
              ensure!(bounded_club.len() >= T::MinLength::get() as usize, Error::<T>::TooShort);
              
              let target = T::Lookup::lookup(target)?;
              
              ensure!(!ClubOf::<T>::contains_key(&target), Error::<T>::AlreadyInClub);

              <ClubOf<T>>::insert(&target, bounded_club);

              Self::deposit_event(Event::<T>::MemberAdded { target });
              
              Ok(())
          }
          
          #[pallet::weight(70_000_000)]
          pub fn remove_member(
              origin: OriginFor<T>,
              target: <T::Lookup as StaticLookup>::Source,
          ) -> DispatchResult {
              T::AdminOrigin::ensure_origin(origin)?;
              
              let target = T::Lookup::lookup(target)?;
              
              ensure!(ClubOf::<T>::contains_key(&target), Error::<T>::NotInClub);

              <ClubOf<T>>::remove(&target);
              
              Self::deposit_event(Event::<T>::MemberRemoved { target });
              
              Ok(())
          }
      }
  }
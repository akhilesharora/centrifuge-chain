//! Centrifuge Connectors pallet
//!
//! TODO(nuno): add rich description
//!
//!
#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode, HasCompact};
use frame_support::{
	dispatch::DispatchResult,
	traits::{
		fungibles::{self, Transfer as FungiblesTransfer},
		tokens::nonfungibles::{self, Inspect as _, Transfer as _},
	},
};
use frame_system::ensure_signed;
use scale_info::TypeInfo;
use sp_runtime::traits::{AccountIdConversion, AtLeast32BitUnsigned};
use sp_std::convert::TryInto;

pub use pallet::*;

pub mod weights;

// Type aliases
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

#[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Price<CurrencyId, Balance> {
	pub currency: CurrencyId,
	pub amount: Balance,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use crate::weights::WeightInfo;
	use frame_support::pallet_prelude::*;
	use frame_support::{transactional, PalletId};
	use frame_system::pallet_prelude::*;
	use frame_system::RawOrigin;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type WeightInfo: WeightInfo;

		type Balance:  Parameter
		+ Member
		+ AtLeast32BitUnsigned
		+ Default
		+ Copy
		+ MaybeSerializeDeserialize
		+ MaxEncodedLen;

		type PoolId: Member
		+ Parameter
		+ Default
		+ Copy
		+ HasCompact
		+ MaxEncodedLen
		+ core::fmt::Debug;

		type TrancheId: Member
		+ Parameter
		+ Default
		+ Copy
		+ MaxEncodedLen
		+ TypeInfo
		+ From<[u8; 16]>;

		type AdminOrigin: EnsureOrigin<Self::Origin>;

		//TODO(nuno)
		type Permissions: Member;

		//TODO(nuno)
		type PoolInspect: Member;
	}

	pub enum Domain {
		Centrifuge,
		Moonbeam,
		Ethereum,
		Avalanche,
		Gnosis,
	}

	struct DomainAddress(pub [u8; 32]);

	pub enum Message<T: Config> {
		AddPool { pool_id: T::PoolId }
		// More to come...
	}

	pub enum Router {
		Nomad(NomadRouter),
		XCM(XCMRouter),
	}


	// Storage
	pub(crate) type Routers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		Domain,
		Router,
	>;

	pub(crate) type LinkedAddressesByAccount<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		Domain,
		DomainAddress
	>;

	pub(crate) type LinkedAddresses<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Domain,
		Blake2_128Concat,
		DomainAddress,
		Bool
	>;

	pub(crate) type DomainBalances<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Domain,
		Blake2_128Concat,
		T::CurrencyId, // future proof to make it work for non tranche tokens
		T::Balance
	>;

	#[pallet::error]
	pub enum Error<T> {
		ToDo,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Add a pool to a given domain
		#[pallet::weight(<T as Config>::WeightInfo::add())]
		pub fn add_pool(
			origin: OriginFor<T>,
			pool_id: T::PoolId,
			domain: Domain,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;

			//TODO(nuno)

			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		// skeleton
	}
}

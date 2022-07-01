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
type PoolIdOf<T> = <T as pallet::Config>::PoolId;

#[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Price<CurrencyId, Balance> {
	pub currency: CurrencyId,
	pub amount: Balance,
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use sp_std::collections::btree_map::BTreeMap;
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

		type CurrencyId: Parameter + Member + Copy + MaybeSerializeDeserialize + Ord + TypeInfo;

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

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A pool was added to the domain
		AddedPool {
			pool_id: T::PoolId,
			domain: Domain,
		}
	}

	#[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub enum Domain {
		Centrifuge,
		Moonbeam,
		Ethereum,
		Avalanche,
		Gnosis,
	}

	#[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct DomainAddress(pub [u8; 32]);

	#[derive(Decode, Clone, PartialEq, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub enum Message<T: Config> {
		AddPool { pool_id: PoolIdOf<T> }
		// More to come...
	}

	impl<T: Config> Encode for Message<T>
	where T: Config,
	{
		fn encode(&self) -> Vec<u8> {
			match self {
				Message::AddPool { pool_id } => {
					let mut message: Vec<u8> = vec![0u8];
					message.append(&mut vec![1,2,3]); //todo(nuno): &mut pool_id.as_bytes().to_vec());
					message
				}
			}
		}
	}

	#[derive(Encode, Decode, Clone, PartialEq, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub enum Router {
		Nomad(NomadRouter),
		XCM(XCMRouter),
	}

	#[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct NomadRouter {
		forwardingContract: String // TODO(nuno): make it a MultiLocation
	}

	#[derive(Encode, Decode, Default, Clone, PartialEq, TypeInfo)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct XCMRouter {
		multilocations: () // TODO(nuno): make it a Map<Domain, MultiLocation>
	}

	#[pallet::storage]
	pub(crate) type Routers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		Domain,
		Router,
	>;

	#[pallet::storage]
	pub(crate) type LinkedAddressesByAccount<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		Domain,
		DomainAddress
	>;

	#[pallet::storage]
	pub(crate) type LinkedAddresses<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		Domain,
		Blake2_128Concat,
		DomainAddress,
		bool
	>;

	#[pallet::storage]
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
		#[pallet::weight(<T as Config>::WeightInfo::add_pool())]
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

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


#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	use scale_info::TypeInfo;
	use codec::MaxEncodedLen;


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		
	}
	
	pub type ProductName = [u8;1024];

	#[derive(Encode, Decode, Eq, PartialEq, Clone, TypeInfo,MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct Product<T:Config>{
    	// The name of the product
    	name:ProductName,
    	// The price of the product
    	price: u64,
    	// The ID of the user who added the product
    	owner: T::AccountId,
		
	}

	impl<T:Config> Product<T> {
		pub fn new(name: ProductName, price: u64, owner: T::AccountId)-> Self{
			Product::<T>{
				name,
				price,
				owner,
			}
		}
	}


	#[pallet::storage]
	#[pallet::getter(fn get_authorized_user )]
	pub(super) type AuthorizedUsers<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, bool, 
    ValueQuery >;


	#[pallet::storage]
	#[pallet::getter(fn get_product_info )]
	pub(super) type Products<T:Config>= StorageMap<_, Blake2_128,u128,Product<T>, 
    OptionQuery>;


	//global counter for product
	#[pallet::storage]
	#[pallet::getter(fn get_product_counter )]	
	pub(super) type ProductCounter<T> = StorageValue<_, u128, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		//even when new user is registered
		NewAuthorizedUser(T::AccountId),

		//event when new product is registered
		NewProduct{
			id : u128, 
			name: ProductName, 
			price: u64,
			owner: T::AccountId
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {

		BadOrigin,

		DeniedOperation,

		AuthorizedUserExist,

	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn add_authorized_user(origin: OriginFor<T>, new_user: T::AccountId) -> DispatchResult {

			if !ensure_root(origin.clone()).is_ok() {
				Self::ensure_authorized(origin)?;
			}

			ensure!(!AuthorizedUsers::<T>::contains_key(&new_user),Error::<T>::AuthorizedUserExist);

			<AuthorizedUsers<T>>::insert(&new_user, true);

			Self::deposit_event(Event::NewAuthorizedUser(new_user));
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn add_product(origin: OriginFor<T>, name: ProductName, price: u64 ) -> DispatchResult {

			Self::ensure_authorized(origin.clone())?;
			let sender = ensure_signed(origin)?;
			let p = Product::<T>::new(name , price , sender);


			let product_counter = match Self::get_product_counter() {
				Some(v)=> v+1,
				None=> 1
			} ;

			<ProductCounter<T>>::set(Some(product_counter));
			<Products<T>>::insert(product_counter,&p);
			
			Self::deposit_event(Event::NewProduct{
				id: product_counter,
				name:p.name,
				price:p.price,
				owner:p.owner
			});
			Ok(())

		}
	}


	impl <T:Config> Pallet<T> {

		pub fn ensure_authorized(origin: OriginFor<T>)-> DispatchResult{
			let sender= ensure_signed(origin)?;
			ensure!(AuthorizedUsers::<T>::contains_key(&sender) , DispatchError::BadOrigin);
			Ok(())
		}
		
	}

}
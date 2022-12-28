#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

pub mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;



#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, traits::{Currency, ReservableCurrency, LockableCurrency, ExistenceRequirement}};
	use frame_system::{pallet_prelude::*, Origin};
	use crate::types::{Product, ProductName, ProductPositionEnum, AccountIdOf, BalanceOf};
	use hex_literal::hex;


	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config  {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: Currency<AccountIdOf<Self>>;
		
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

	#[pallet::storage]
	#[pallet::getter(fn get_product_owner_account)]
	pub(super) type ProductOwnerAccount<T: Config> = StorageValue<_, AccountIdOf<T>, OptionQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		//even when new user is registered
		NewAuthorizedUser(T::AccountId),

		Product{
			id: u128,
			product:Product<T>,
		},
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {

		BadOrigin,

		AuthorizedUserExist,

		ProductDonotExist,

		NotReadyForRetailer,

		ProductIsSold,

		InsufficientBalance,

		ServerAccountNotFound,

		NotInResaleList,

		ProductIsNotSold,

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
		pub fn add_product(origin: OriginFor<T>, name: ProductName, price: BalanceOf<T>) -> DispatchResult {

			Self::ensure_authorized(origin.clone())?;
			let sender = ensure_signed(origin)?;
			let p = Product::<T>::new(name , price , sender, ProductPositionEnum::Manufacture, false);

			let product_counter = match Self::get_product_counter() {
				Some(v)=> v+1,
				None=> 1
			} ;

			<ProductCounter<T>>::set(Some(product_counter));
			<Products<T>>::insert(product_counter,&p);
			
			Self::deposit_event(Event::Product{
				id: product_counter,
				product: p,
			});
			Ok(())

		}
	

		#[pallet::call_index(2)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn update_position(origin: OriginFor<T>,id: u128,  position: ProductPositionEnum)-> DispatchResult{

			Self::ensure_authorized(origin.clone())?;

			ensure!(Products::<T>::contains_key(id),Error::<T>::ProductDonotExist);

			let mut p:Product<T> =  Self::get_product_info(id).unwrap();

			ensure!(!p.get_is_sold(), Error::<T>::ProductIsSold);
			
			p.set_position(position);
			<Products<T>>::insert(id,&p);
			Self::deposit_event(Event::Product{
				id:id,
				product: p,
			});
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn buy_product(origin:OriginFor<T>, id: u128 )-> DispatchResult {

			let buyer = ensure_signed(origin)?;

			//check if product exists
			let mut p:Product<T> =  Self::get_product_info(id).ok_or(Error::<T>::ProductDonotExist)?;


			//product can be bought only after ready for retailer
			ensure!(p.get_position() == 
				ProductPositionEnum::Retailer, 
				Error::<T>::NotReadyForRetailer
			);

			if(p.get_is_sold()&& !p.get_resale() ){
				Err(Error::<T>::NotInResaleList);
			}

			// check sender balance greater than min balance
			ensure!(
				<T as Config>::Currency::free_balance(&buyer) >
				p.get_price() + <T as Config>::Currency::minimum_balance(), 
				Error::<T>::InsufficientBalance
			);

			// check if owner account exists
			let owner_account  = Self::get_product_owner_account().
					ok_or(Error::<T>::ServerAccountNotFound)?;
			
			
			// transfer fund to product_owner_account
			<T as Config>::Currency::transfer(&buyer, 
							&owner_account, 
							p.get_price(), 
							ExistenceRequirement::AllowDeath
			)?;

			//change product owner
			p.set_owner(buyer);

			//set_sold_property
			p.set_is_sold(true);

			<Products<T>>::insert(id,&p);
			Self::deposit_event(Event::Product{
				id:id,
				product: p,
			});

			Ok(())
		}


		#[pallet::call_index(4)]
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn enable_resale(origin:OriginFor<T>,id: u128, price: BalanceOf<T>)-> DispatchResult{
			let owner =ensure_signed(origin)?;

			// check if product exists
			let mut p:Product<T> =  Self::get_product_info(id).ok_or(Error::<T>::ProductDonotExist)?;

			// validate owner
			ensure!(owner== p.get_owner(), Error::<T>::BadOrigin);

			// validate is sold
			ensure!(p.get_is_sold(), Error::<T>::ProductIsNotSold);

			// resale and price adjusted
			p.set_resale(true);
			p.set_price(price);
			<Products<T>>::insert(id,&p);
			Self::deposit_event(Event::Product{
				id:id,
				product: p,
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

	//genesis_pallet_account
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub product_owner_account: AccountIdOf<T>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			let product_owner_account_hex =
				hex!["eea38549ab839643085bc97194cd4701810f35255f2117c356ba629f4146461d"];
			let product_owner_account =
				AccountIdOf::<T>::decode(&mut &product_owner_account_hex[..]).unwrap();

			Self {
				product_owner_account,
			}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			ProductOwnerAccount::<T>::put(&self.product_owner_account);
		}
	}


}
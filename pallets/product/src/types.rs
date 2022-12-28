use crate as product;

use frame_support::{BoundedVec};
use scale_info::TypeInfo;
use product::pallet::Config;
use frame_support::traits::Currency;
use codec::{MaxEncodedLen, Encode, Decode};
use sp_core::ConstU32;


pub type ProductName = BoundedVec<u8, ConstU32<10>>;

pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;


#[derive(Encode, Decode, Eq, PartialEq, Clone, TypeInfo,MaxEncodedLen,Debug,Copy)]
pub enum ProductPositionEnum {
    Manufacture,
    Distribution,
    Retailer,
}


#[derive(Encode, Decode, Eq, PartialEq, Clone, TypeInfo,MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Product<T:Config>{

    // The name of the product
    name:ProductName,
    
    // The price of the product
    price:BalanceOf<T>,

    // The ID of the user who added the product
    owner: T::AccountId,

    // whether product is sold or not 
    is_sold: bool,


    //destination
    position: ProductPositionEnum,
    
}

impl<T:Config> Product<T> {
    pub fn new(name: ProductName, price:BalanceOf<T>, owner: T::AccountId,  position: ProductPositionEnum, is_sold: bool)-> Self{
        Product::<T>{
            name,
            price,
            owner,
            is_sold,
            position,
        }
    }

    pub fn get_position(&mut self)-> ProductPositionEnum{ return self.position}
    pub fn set_position(&mut self, position: ProductPositionEnum){self.position= position}

    pub fn get_owner(&mut self)->T::AccountId{return self.owner.clone()}
    pub fn set_owner(&mut self, owner: T::AccountId){self.owner= owner}

    pub fn get_is_sold(&mut self)->bool{ return self.is_sold}
    pub fn set_is_sold(&mut self, sold:bool){self.is_sold=sold}

    pub fn get_price(&mut self)->BalanceOf<T>{return self.price}
}

impl<T: Config> core::fmt::Debug for Product<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Product")
			.field("name", &self.name)
			.field("owner", &self.owner)
			.field("price", &self.price)
			.finish()
	}
}
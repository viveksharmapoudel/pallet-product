use crate as product;

use frame_support::BoundedVec;
use frame_system::Config;
use scale_info::TypeInfo;
use codec::{MaxEncodedLen, Encode, Decode};
use sp_core::ConstU32;


pub type ProductName = BoundedVec<u8, ConstU32<10>>;


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

impl<T: Config> core::fmt::Debug for Product<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Product")
			.field("name", &self.name)
			.field("owner", &self.owner)
			.field("price", &self.price)
			.finish()
	}
}
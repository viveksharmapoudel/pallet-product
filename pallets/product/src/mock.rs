use crate::{self as pallet_product, types::AccountIdOf};
use frame_support::{traits::{ConstU16, ConstU64}, parameter_types};
use frame_system as system;

pub const TEST_OWNER_ACCOUNT:u64=100;

use sp_core::{H256, ConstU32};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};


use pallet_balances;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		ProductModule: pallet_product,
		Balances: pallet_balances,
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 500;
}


impl pallet_product::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
}

impl pallet_balances::Config for Test {
	type Balance = u64;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ConstU32<10>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}

pub fn minimal_test_ext() -> sp_io::TestExternalities {
	use frame_support::traits::GenesisBuild;
	// use hex_literal::hex;
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap();
	// let product_owner_account_hex = hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"];
	// let product_owner_account =
	// AccountIdOf::<Test>::decode(&mut &product_owner_account_hex[..]).unwrap();
	pallet_product::GenesisConfig::<Test> {
		product_owner_account:TEST_OWNER_ACCOUNT,
	}
	.assimilate_storage(&mut t)
	.unwrap();
	t.into()
}



#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::{collections::BTreeMap, string::String};

use contract::contract_api::{runtime, storage, system};
use types::{Key, URef};

const KEY_ADMIN: &str = "admin";
const NAME_SWAP_HASH: &str = "swap_hash";
const NAME_SWAP_LOGIC_EXT: &str = "swap_logic_ext";
const KEY_CONTRACT_PURSE: &str = "swap_contract_purse";

#[no_mangle]
pub extern "C" fn swap_logic_ext() {
    swap_logic::delegate();
}

#[no_mangle]
pub extern "C" fn call() {
    // create map of references for stored contract
    let mut swapper_urefs: BTreeMap<String, Key> = BTreeMap::new();

    // Get caller's public key and store as admin
    let admin_uref: URef = storage::new_uref(runtime::get_caller());
    swapper_urefs.insert(String::from(KEY_ADMIN), admin_uref.into());

    // Create purse for contract
    let contract_purse = system::create_purse();
    swapper_urefs.insert(String::from(KEY_CONTRACT_PURSE), contract_purse.into());

    // Swap function storage
    let swap_function_pointer = storage::store_function_at_hash(NAME_SWAP_LOGIC_EXT, swapper_urefs);

    swap_proxy::deploy_swap_proxy();
    runtime::put_key(NAME_SWAP_HASH, swap_function_pointer.into());
}

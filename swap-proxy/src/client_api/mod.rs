mod error;

use alloc::{string::String, vec::Vec};

use contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use types::{account::PublicKey, ApiError, ContractRef, Key, U512};

use error::Error;

pub mod method_names {
    pub mod proxy {
        use super::swap;

        pub const NAME_SWAP_HASH: &str = "swap_hash";
        pub const METHOD_SET_SWAP_HASH: &str = swap::METHOD_SET_SWAP_HASH;
        pub const METHOD_INSERT_KYC_ALLOWANCE_CAP: &str = swap::METHOD_INSERT_KYC_ALLOWANCE_CAP;
        pub const METHOD_INSERT_SNAPSHOT_RECORD: &str = swap::METHOD_INSERT_SNAPSHOT_RECORD;
        pub const METHOD_INSERT_KYC_DATA: &str = swap::METHOD_INSERT_KYC_DATA;
        pub const METHOD_UPDATE_KYC_LEVEL: &str = swap::METHOD_UPDATE_KYC_LEVEL;
        pub const METHOD_GET_TOKEN: &str = swap::METHOD_GET_TOKEN;
    }
    pub mod swap {
        pub const METHOD_SET_SWAP_HASH: &str = "set_swap_hash";
        pub const METHOD_INSERT_KYC_ALLOWANCE_CAP: &str = "insert_kyc_allowance_cap";
        pub const METHOD_INSERT_SNAPSHOT_RECORD: &str = "insert_snapshot_record";
        pub const METHOD_INSERT_KYC_DATA: &str = "insert_kyc_data";
        pub const METHOD_UPDATE_KYC_LEVEL: &str = "update_kyc_level";
        pub const METHOD_GET_TOKEN: &str = "get_token";
    }
}

pub enum Api {
    SetSwapHash(Key),
    InsertKYCAllowanceCap(U512),
    InsertSnapshotRecord(String, U512),
    InsertKYCData(PublicKey, U512),
    UpdateKYCLevel(PublicKey, U512),
    GetToken(Key, Vec<String>, Vec<String>, Vec<String>),
}

fn get_contract_ref() -> ContractRef {
    let contract_hash = runtime::get_key(method_names::proxy::NAME_SWAP_HASH)
        .unwrap_or_revert_with(ApiError::GetKey);
    contract_hash.to_contract_ref().unwrap_or_revert()
}

impl Api {
    pub fn from_args() -> Self {
        let method_name: String = runtime::get_arg(0)
            .unwrap_or_revert_with(ApiError::MissingArgument)
            .unwrap_or_revert_with(ApiError::InvalidArgument);

        match method_name.as_str() {
            method_names::proxy::METHOD_SET_SWAP_HASH => {
                let swap_hash: Key = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);

                Api::SetSwapHash(swap_hash)
            }
            method_names::proxy::METHOD_INSERT_KYC_ALLOWANCE_CAP => {
                let cap_number: U512 = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);

                Api::InsertKYCAllowanceCap(cap_number)
            }
            method_names::proxy::METHOD_INSERT_SNAPSHOT_RECORD => {
                let ver1_address: String = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let amount: U512 = runtime::get_arg(2)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                Api::InsertSnapshotRecord(ver1_address, amount)
            }
            method_names::proxy::METHOD_INSERT_KYC_DATA => {
                let new_mainnet_address: PublicKey = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let kyc_level: U512 = runtime::get_arg(2)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);

                Api::InsertKYCData(new_mainnet_address, kyc_level)
            }
            method_names::proxy::METHOD_UPDATE_KYC_LEVEL => {
                let new_mainnet_address: PublicKey = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let kyc_level: U512 = runtime::get_arg(2)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                Api::UpdateKYCLevel(new_mainnet_address, kyc_level)
            }
            method_names::proxy::METHOD_GET_TOKEN => {
                let contract_hash: Key = runtime::get_arg(1)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let ver1_pubkey: Vec<String> = runtime::get_arg(2)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let message: Vec<String> = runtime::get_arg(3)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);
                let signature: Vec<String> = runtime::get_arg(4)
                    .unwrap_or_revert_with(ApiError::MissingArgument)
                    .unwrap_or_revert_with(ApiError::InvalidArgument);

                Api::GetToken(contract_hash, ver1_pubkey, message, signature)
            }
            _ => runtime::revert(Error::UnknownProxyApi),
        }
    }

    pub fn invoke(&self) {
        match self {
            Self::SetSwapHash(swap_hash) => {
                let contract_ref = swap_hash.to_contract_ref().unwrap_or_revert();
                runtime::call_contract(
                    contract_ref,
                    (method_names::proxy::METHOD_SET_SWAP_HASH, *swap_hash),
                )
            }
            Self::InsertKYCAllowanceCap(allowance_cap) => {
                let swap_ref = get_contract_ref();
                runtime::call_contract(
                    swap_ref,
                    (
                        method_names::proxy::METHOD_INSERT_KYC_ALLOWANCE_CAP,
                        *allowance_cap,
                    ),
                )
            }
            Self::InsertSnapshotRecord(ver1_address, amount) => {
                let swap_ref = get_contract_ref();
                runtime::call_contract(
                    swap_ref,
                    (
                        method_names::proxy::METHOD_INSERT_SNAPSHOT_RECORD,
                        ver1_address.clone(),
                        *amount,
                    ),
                )
            }
            Self::InsertKYCData(new_mainnet_address, kyc_level) => {
                let swap_ref = get_contract_ref();
                runtime::call_contract(
                    swap_ref,
                    (
                        method_names::proxy::METHOD_INSERT_KYC_DATA,
                        *new_mainnet_address,
                        *kyc_level,
                    ),
                )
            }
            Self::UpdateKYCLevel(new_mainnet_address, kyc_level) => {
                let swap_ref = get_contract_ref();
                runtime::call_contract(
                    swap_ref,
                    (
                        method_names::proxy::METHOD_UPDATE_KYC_LEVEL,
                        *new_mainnet_address,
                        *kyc_level,
                    ),
                )
            }
            Self::GetToken(swap_contract_hash, ver1_pubkey_arr, message_arr, signature_arr) => {
                let contract_ref = swap_contract_hash.to_contract_ref().unwrap_or_revert();

                runtime::call_contract(
                    contract_ref,
                    (
                        method_names::proxy::METHOD_GET_TOKEN,
                        ver1_pubkey_arr.clone(),
                        message_arr.clone(),
                        signature_arr.clone(),
                    ),
                )
            }
        }
    }
}

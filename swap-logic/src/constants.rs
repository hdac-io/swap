pub(crate) mod methods {
    pub const METHOD_INSERT_KYC_ALLOWANCE_CAP: &str = "insert_kyc_allowance_cap";
    pub const METHOD_INSERT_SNAPSHOT_RECORD: &str = "insert_snapshot_record";
    pub const METHOD_GET_CONTRACT_PURSE: &str = "get_contract_purse";
    pub const METHOD_INSERT_KYC_DATA: &str = "insert_kyc_data";
    pub const METHOD_UPDATE_KYC_LEVEL: &str = "update_kyc_level";
    pub const METHOD_GET_TOKEN: &str = "get_token";
}

pub(crate) mod keys {
    pub const KEY_PREV_BALANCE_KEY: &str = "prev_balance";
    pub const KEY_IS_SWAPPED: &str = "is_swapped";

    pub const KEY_KYC_LEVEL: &str = "kyc_level";
    pub const KEY_SWAPPED_AMOUNT: &str = "swapped_amount";

    pub const KEY_KYC_BORDER_ALLOWANCE_CAP: &str = "kyc_border_allowance_cap";

    pub const KEY_CONTRACT_PURSE: &str = "swap_contract_purse";
}

pub(crate) mod users {
    pub const KEY_ADMIN: &str = "admin";
}

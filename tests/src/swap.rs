extern crate alloc;
use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
};
use core::{convert::TryFrom, fmt::Write};

use engine_core::engine_state::genesis::GenesisAccount;
use engine_shared::{account::Account, motes::Motes, stored_value::StoredValue};
use engine_test_support::{
    internal::{utils, ExecuteRequestBuilder, InMemoryWasmTestBuilder},
    DEFAULT_ACCOUNT_INITIAL_BALANCE,
};
use types::{account::PublicKey, ApiError, CLValue, Key, U512};

const CONTRACT_POS_VOTE: &str = "swap_install.wasm";
const BIGSUN_TO_HDAC: u64 = 1_000_000_000_000_000_000_u64;

const ADMIN_PUBKEY: PublicKey = PublicKey::ed25519_from([1u8; 32]);
const ACCOUNT_1_PUBKEY: PublicKey = PublicKey::ed25519_from([2u8; 32]);

const GENESIS_VALIDATOR_STAKE: u64 = 5u64 * BIGSUN_TO_HDAC;

const VER1_ADDRESS: &str = "HLkXSESzSaDZgU25CQrmxkjRayKfs5xBFK";
const VER1_PUBKEY: &str = "02c4ef70543e18889167ca67c8aa28c1d4c259e89cb34483a8ed6cfd3a03e8246b";
const VER1_MESSAGE_HASHED: &str =
    "69046d44e3d75d48436377626372a44a5066966b5d72c00b67769c1cc6a8619a";
const VER1_SIGNATURE: &str =
    "24899366fd3d5dfe6740df1e5f467a53f1a3aaafce26d8df1497a925c55b5c266339a95fe6\
                              507bd611b0e3b6e74e3bb7f19eeb1165615e5cebe7f40e5765bc41";

const VER1_ADDRESS_2: &str = "H9EtjvP88K51nTSevyNW2p9VkSbuzhwgWQ";
// const VER1_PUBKEY_2: &str = "02fce4c49d848d3389f71d7dfd28b0a7fea9861e9b0343fe2572e31178d116f35f";

const VER1_AMOUNT_1: u64 = 10_000;
const VER1_AMOUNT_2: u64 = 10_000;
const SWAP_CAP_1: u64 = 5_000;
// const SWAP_CAP_2: u64 = 15_000;

fn get_account(builder: &InMemoryWasmTestBuilder, account: PublicKey) -> Account {
    match builder
        .query(None, Key::Account(account), &[])
        .expect("should query system account")
    {
        StoredValue::Account(res_account) => res_account,
        _ => panic!("should get an account"),
    }
}

fn get_swap_hash(builder: &InMemoryWasmTestBuilder) -> [u8; 32] {
    // query client_api_proxy_hash from SYSTEM_ACCOUNT
    let admin_account = get_account(builder, ADMIN_PUBKEY);

    admin_account
        .named_keys()
        .get("swap_proxy")
        .expect("should get swap key")
        .into_hash()
        .expect("should be hash")
}

fn get_swap_stored_hash(builder: &InMemoryWasmTestBuilder) -> Key {
    // query client_api_proxy_hash from SYSTEM_ACCOUNT
    let admin_account = get_account(builder, ADMIN_PUBKEY);

    *admin_account
        .named_keys()
        .get("swap_hash")
        .expect("should get swap key")
}

fn to_hex_string(address: PublicKey) -> String {
    let bytes = address.value();
    let mut ret = String::with_capacity(64);
    for byte in &bytes[..32] {
        write!(ret, "{:02x}", byte).expect("Writing to a string cannot fail");
    }

    ret
}

#[ignore]
#[test]
fn should_run_insert_update_info_and_swap_step() {
    // Genesis setting
    let accounts = vec![
        GenesisAccount::new(
            ADMIN_PUBKEY,
            Motes::new(DEFAULT_ACCOUNT_INITIAL_BALANCE.into()),
            Motes::new(GENESIS_VALIDATOR_STAKE.into()),
        ),
        GenesisAccount::new(
            ACCOUNT_1_PUBKEY,
            Motes::new(U512::from(0)),
            Motes::new(GENESIS_VALIDATOR_STAKE.into()),
        ),
    ];

    let genesis_config = utils::create_genesis_config(accounts, Default::default());
    let mut builder = InMemoryWasmTestBuilder::default();
    let result = builder.run_genesis(&genesis_config).commit().finish();

    // Swap install phase
    println!("1-1. Swap install");
    let swap_install_request =
        ExecuteRequestBuilder::standard(ADMIN_PUBKEY, CONTRACT_POS_VOTE, ()).build();
    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(swap_install_request)
        .expect_success()
        .commit()
        .finish();

    let swap_contract_hash = get_swap_hash(&builder);

    // Swap install pahse
    println!("1-2. Input swap allowance cap by KYC level");
    let set_swap_cap = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        ("insert_kyc_allowance_cap", U512::from(SWAP_CAP_1)),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(set_swap_cap)
        .expect_success()
        .commit()
        .finish();

    // Input existing information
    println!("2. Ver1 Token info insert");
    let ver1_token_info_insert_request = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        (
            "insert_snapshot_record",
            VER1_ADDRESS,
            U512::from(VER1_AMOUNT_1),
        ),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(ver1_token_info_insert_request)
        .expect_success()
        .commit()
        .finish();

    let ver1_token_info_insert_request2 = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        (
            "insert_snapshot_record",
            VER1_ADDRESS_2,
            U512::from(VER1_AMOUNT_2),
        ),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(ver1_token_info_insert_request2)
        .expect_success()
        .commit()
        .finish();

    let contract_ref = get_swap_stored_hash(&builder);
    let value: BTreeMap<String, String> = CLValue::try_from(
        builder
            .query(
                Some(builder.get_post_state_hash()),
                contract_ref,
                &[VER1_ADDRESS],
            )
            .expect("cannot derive stored value"),
    )
    .expect("should have CLValue")
    .into_t()
    .expect("should convert successfully");

    assert_eq!(value.is_empty(), false);

    // Input existing information
    println!("2-1. Insert KYC data");
    let insert_kyc = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        ("insert_kyc_data", ACCOUNT_1_PUBKEY, U512::from(1)),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder.exec(insert_kyc).expect_success().commit().finish();

    let contract_ref = get_swap_stored_hash(&builder);
    let value: BTreeMap<String, String> = CLValue::try_from(
        builder
            .query(
                Some(builder.get_post_state_hash()),
                contract_ref,
                &[&to_hex_string(ACCOUNT_1_PUBKEY)],
            )
            .expect("cannot derive stored value"),
    )
    .expect("should have CLValue")
    .into_t()
    .expect("should convert successfully");

    assert_eq!(value.is_empty(), false);
    assert_eq!(value.get("kyc_level").unwrap(), "1");

    let contract_ref = get_swap_stored_hash(&builder);
    println!("3. Get token without upper level KYC. It should fail");
    let get_token_request = ExecuteRequestBuilder::contract_call_by_hash(
        ACCOUNT_1_PUBKEY,
        swap_contract_hash,
        (
            "get_token",
            contract_ref,
            vec![VER1_PUBKEY],
            vec![VER1_MESSAGE_HASHED],
            vec![VER1_SIGNATURE],
        ),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder.exec(get_token_request).commit().finish();

    let response = result
        .builder()
        .get_exec_response(0)
        .expect("should have a response")
        .to_owned();

    let error_message = utils::get_error_message(response);

    assert!(error_message.contains(&format!("Revert({})", u32::from(ApiError::User(2),))));

    let contract_ref = get_swap_stored_hash(&builder);
    let value: BTreeMap<String, String> = CLValue::try_from(
        builder
            .query(
                Some(builder.get_post_state_hash()),
                contract_ref,
                &[&to_hex_string(ACCOUNT_1_PUBKEY)],
            )
            .expect("cannot derive stored value"),
    )
    .expect("should have CLValue")
    .into_t()
    .expect("should convert successfully");

    assert_eq!(value.get("swapped_amount").unwrap(), "0",);

    // Update KYC level
    println!("4-1. Upgrade KYC level");
    let update_kyc_level_request = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        ("update_kyc_level", ACCOUNT_1_PUBKEY, U512::from(2u64)),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(update_kyc_level_request)
        .expect_success()
        .commit()
        .finish();

    let before_balance = builder.get_purse_balance(
        builder
            .get_account(ACCOUNT_1_PUBKEY)
            .expect("should have account")
            .main_purse(),
    );

    // Update KYC step
    let contract_ref = get_swap_stored_hash(&builder);
    println!("4-2. Get token with upper level KYC. Should success now");
    let get_token_request = ExecuteRequestBuilder::contract_call_by_hash(
        ACCOUNT_1_PUBKEY,
        swap_contract_hash,
        (
            "get_token",
            contract_ref,
            vec![VER1_PUBKEY],
            vec![VER1_MESSAGE_HASHED],
            vec![VER1_SIGNATURE],
        ),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(get_token_request)
        .expect_success()
        .commit()
        .finish();

    let contract_ref = get_swap_stored_hash(&builder);
    let value: BTreeMap<String, String> = CLValue::try_from(
        builder
            .query(
                Some(builder.get_post_state_hash()),
                contract_ref,
                &[&to_hex_string(ACCOUNT_1_PUBKEY)],
            )
            .expect("cannot derive stored value"),
    )
    .expect("should have CLValue")
    .into_t()
    .expect("should convert successfully");

    assert_eq!(
        value.get("swapped_amount").unwrap(),
        &VER1_AMOUNT_1.to_string(),
    );

    let after_balance = builder.get_purse_balance(
        builder
            .get_account(ACCOUNT_1_PUBKEY)
            .expect("should have account")
            .main_purse(),
    );

    assert_eq!(
        // U512::from(BIGSUN_TO_HDAC / 10): Tx fee in test
        (U512::from(BIGSUN_TO_HDAC / 10) + after_balance - before_balance) % U512::from(100_000),
        U512::from(VER1_AMOUNT_1),
    );

    let contract_ref = get_swap_stored_hash(&builder);
    println!("4-3. Try to swap with same wallet. Should fail");
    let get_token_request = ExecuteRequestBuilder::contract_call_by_hash(
        ACCOUNT_1_PUBKEY,
        swap_contract_hash,
        (
            "get_token",
            contract_ref,
            vec![VER1_PUBKEY],
            vec![VER1_MESSAGE_HASHED],
            vec![VER1_SIGNATURE],
        ),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder.exec(get_token_request).commit().finish();

    let response = result
        .builder()
        .get_exec_response(0)
        .expect("should have a response")
        .to_owned();

    let error_message = utils::get_error_message(response);

    assert!(error_message.contains(&format!("Revert({})", u32::from(ApiError::User(9)),)));

    let contract_ref = get_swap_stored_hash(&builder);
    let value: BTreeMap<String, String> = CLValue::try_from(
        builder
            .query(
                Some(builder.get_post_state_hash()),
                contract_ref,
                &[&to_hex_string(ACCOUNT_1_PUBKEY)],
            )
            .expect("cannot derive stored value"),
    )
    .expect("should have CLValue")
    .into_t()
    .expect("should convert successfully");

    assert_eq!(
        value.get("swapped_amount").unwrap(),
        &VER1_AMOUNT_1.to_string(),
    );
}

#[ignore]
#[test]
fn should_fail_swaprequest_if_kyc_is_not_inserted() {
    // Genesis setting
    let accounts = vec![
        GenesisAccount::new(
            ADMIN_PUBKEY,
            Motes::new(DEFAULT_ACCOUNT_INITIAL_BALANCE.into()),
            Motes::new(GENESIS_VALIDATOR_STAKE.into()),
        ),
        GenesisAccount::new(
            ACCOUNT_1_PUBKEY,
            Motes::new(DEFAULT_ACCOUNT_INITIAL_BALANCE.into()),
            Motes::new(GENESIS_VALIDATOR_STAKE.into()),
        ),
    ];

    let genesis_config = utils::create_genesis_config(accounts, Default::default());
    let mut builder = InMemoryWasmTestBuilder::default();
    let result = builder.run_genesis(&genesis_config).commit().finish();

    // Swap install phase
    println!("1-1. Swap install");
    let swap_install_request =
        ExecuteRequestBuilder::standard(ADMIN_PUBKEY, CONTRACT_POS_VOTE, ()).build();
    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(swap_install_request)
        .expect_success()
        .commit()
        .finish();

    let swap_contract_hash = get_swap_hash(&builder);

    // Swap install pahse
    println!("1-2. Input swap allowance cap by KYC level");
    let set_swap_cap = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        ("insert_kyc_allowance_cap", U512::from(SWAP_CAP_1)),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(set_swap_cap)
        .expect_success()
        .commit()
        .finish();

    // Input existing information
    println!("2. Ver1 Token info insert");
    let ver1_token_info_insert_request = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        (
            "insert_snapshot_record",
            VER1_ADDRESS,
            U512::from(VER1_AMOUNT_1),
        ),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(ver1_token_info_insert_request)
        .expect_success()
        .commit()
        .finish();

    let contract_ref = get_swap_stored_hash(&builder);
    let value: BTreeMap<String, String> = CLValue::try_from(
        builder
            .query(
                Some(builder.get_post_state_hash()),
                contract_ref,
                &[VER1_ADDRESS],
            )
            .expect("cannot derive stored value"),
    )
    .expect("should have CLValue")
    .into_t()
    .expect("should convert successfully");

    assert_eq!(value.is_empty(), false);

    let contract_ref = get_swap_stored_hash(&builder);
    println!("3. Swap request without KYC info. Should fail");
    let get_token_request = ExecuteRequestBuilder::contract_call_by_hash(
        ACCOUNT_1_PUBKEY,
        swap_contract_hash,
        (
            "get_token",
            contract_ref,
            vec![VER1_PUBKEY],
            vec![VER1_MESSAGE_HASHED],
            vec![VER1_SIGNATURE],
        ),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder.exec(get_token_request).commit().finish();

    let response = result
        .builder()
        .get_exec_response(0)
        .expect("should have a response")
        .to_owned();

    let error_message = utils::get_error_message(response);
    assert!(error_message.contains(&format!("Revert({})", u32::from(ApiError::GetKey),)));
}

#[ignore]
#[test]
fn should_fail_swaprequest_if_there_is_no_snapshot() {
    // Genesis setting
    let accounts = vec![
        GenesisAccount::new(
            ADMIN_PUBKEY,
            Motes::new(DEFAULT_ACCOUNT_INITIAL_BALANCE.into()),
            Motes::new(GENESIS_VALIDATOR_STAKE.into()),
        ),
        GenesisAccount::new(
            ACCOUNT_1_PUBKEY,
            Motes::new(U512::from(0)),
            Motes::new(GENESIS_VALIDATOR_STAKE.into()),
        ),
    ];

    let genesis_config = utils::create_genesis_config(accounts, Default::default());
    let mut builder = InMemoryWasmTestBuilder::default();
    let result = builder.run_genesis(&genesis_config).commit().finish();

    // Swap install phase
    println!("1-1. Swap install");
    let swap_install_request =
        ExecuteRequestBuilder::standard(ADMIN_PUBKEY, CONTRACT_POS_VOTE, ()).build();
    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(swap_install_request)
        .expect_success()
        .commit()
        .finish();

    let swap_contract_hash = get_swap_hash(&builder);

    // Swap install pahse
    println!("1-2. Input swap allowance cap by KYC level");
    let set_swap_cap = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        ("insert_kyc_allowance_cap", U512::from(SWAP_CAP_1)),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder
        .exec(set_swap_cap)
        .expect_success()
        .commit()
        .finish();

    // Input existing information
    println!("2. Insert KYC data");
    let insert_kyc = ExecuteRequestBuilder::contract_call_by_hash(
        ADMIN_PUBKEY,
        swap_contract_hash,
        ("insert_kyc_data", ACCOUNT_1_PUBKEY, U512::from(1)),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let result = builder.exec(insert_kyc).expect_success().commit().finish();

    let contract_ref = get_swap_stored_hash(&builder);
    let value: BTreeMap<String, String> = CLValue::try_from(
        builder
            .query(
                Some(builder.get_post_state_hash()),
                contract_ref,
                &[&to_hex_string(ACCOUNT_1_PUBKEY)],
            )
            .expect("cannot derive stored value"),
    )
    .expect("should have CLValue")
    .into_t()
    .expect("should convert successfully");

    assert_eq!(value.is_empty(), false);
    assert_eq!(value.get("kyc_level").unwrap(), "1");

    let contract_ref = get_swap_stored_hash(&builder);
    println!("3. Swap request without snapshot. Finishes as success but nothing swapped.");
    let get_token_request = ExecuteRequestBuilder::contract_call_by_hash(
        ACCOUNT_1_PUBKEY,
        swap_contract_hash,
        (
            "get_token",
            contract_ref,
            vec![VER1_PUBKEY],
            vec![VER1_MESSAGE_HASHED],
            vec![VER1_SIGNATURE],
        ),
    )
    .build();

    let mut builder = InMemoryWasmTestBuilder::from_result(result);
    let _result = builder
        .exec(get_token_request)
        .expect_success()
        .commit()
        .finish();

    let after_balance = builder.get_purse_balance(
        builder
            .get_account(ACCOUNT_1_PUBKEY)
            .expect("should have account")
            .main_purse(),
    );

    assert_eq!(
        // No token is swapped then it goes to zero
        after_balance % U512::from(100_000),
        U512::from(0),
    );
}

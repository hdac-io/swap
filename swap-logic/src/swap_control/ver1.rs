extern crate hex;

use super::error::Error as SwapError;
use alloc::{string::String, vec::Vec};
use contract::contract_api::runtime;
use ripemd160::Ripemd160;
use secp256k1::{self, Message, PublicKey as Ver1PubKey, Signature};
use sha2::{Digest, Sha256};

pub fn signature_verification(
    ver1_pubkey_hex: String,
    message: String,
    signature_hex: String,
) -> bool {
    let ver1_pubkey_bytes = match hex::decode(ver1_pubkey_hex) {
        Ok(val) => val,
        Err(_) => runtime::revert(SwapError::PublicKeyDecodeFail),
    };
    let mut ver1_pubkey_byted_arr: [u8; 33] = [0u8; 33];
    ver1_pubkey_byted_arr.copy_from_slice(&ver1_pubkey_bytes.as_slice()[0..33]);
    let ver1_pubkey: Ver1PubKey = match Ver1PubKey::parse_compressed(&ver1_pubkey_byted_arr) {
        Ok(val) => val,
        Err(_) => runtime::revert(SwapError::InvalidHexOfPublicKey),
    };

    // Message is already hashed. Don't have to hash again in here.
    let message_bytes = match hex::decode(message) {
        Ok(val) => val,
        Err(_) => runtime::revert(SwapError::MessageDecodeFail),
    };
    let mut hashed_msg: [u8; 32] = [0u8; 32];
    hashed_msg.copy_from_slice(&message_bytes);
    let message_struct = Message::parse(&hashed_msg);

    // 64-byted signature, not DER-encoded 71 byte
    let signature_vec = match hex::decode(signature_hex) {
        Ok(val) => val,
        Err(_) => runtime::revert(SwapError::SignatureHexDecodeFail),
    };
    let signature_byte: &[u8] = signature_vec.as_slice();
    let signature_obj = match Signature::parse_slice(signature_byte) {
        Ok(val) => val,
        Err(_) => runtime::revert(SwapError::InvalidVer1Signature),
    };

    secp256k1::verify(&message_struct, &signature_obj, &ver1_pubkey)
}

pub fn derive_ver1_address(ver1_pubkey_hex: String) -> String {
    let ver1_pubkey_bytes = match hex::decode(ver1_pubkey_hex) {
        Ok(val) => val,
        Err(_) => runtime::revert(SwapError::PublicKeyDecodeFail),
    };

    // hash160
    let mut sha256hasher = Sha256::new();
    sha256hasher.input(ver1_pubkey_bytes);
    let sha256res = sha256hasher.result();

    let mut ripemd160hasher = Ripemd160::new();
    ripemd160hasher.input(sha256res);
    let hash160res = ripemd160hasher.result();

    // payload
    let mut payload: Vec<u8> = Vec::new();
    let prefix: u8 = 0x28;
    payload.push(prefix);
    for item in hash160res.iter() {
        payload.push(*item);
    }

    // Hdac checksum
    // 1. checksum body
    let mut sha256hasher = Sha256::new();
    sha256hasher.input(payload.clone());
    let sha256res_for_checksum_1st = sha256hasher.result();
    let mut sha256hasher = Sha256::new();
    sha256hasher.input(sha256res_for_checksum_1st);
    let mut sha256res_for_checksum_2nd = sha256hasher.result();
    let (sha256res_for_checksum_sliced, _) = sha256res_for_checksum_2nd.split_at_mut(4);
    sha256res_for_checksum_sliced.reverse();

    // 2. 48444143
    let mut dummy_bytes = match hex::decode("48444143") {
        Ok(val) => val,
        Err(_) => runtime::revert(SwapError::MessageDecodeFail),
    };
    dummy_bytes.reverse();

    // 3. XOR
    let mut buffered: Vec<u8> = Vec::new();
    for idx in 0..4 {
        buffered.push(sha256res_for_checksum_sliced[idx] ^ dummy_bytes[idx]);
    }
    buffered.reverse();

    // 'H' + hash160 + Hdac ver1 checksum = 1 + 20 + 4 = 25 bytes
    let res = {
        let mut res = Vec::with_capacity(payload.len() + buffered.len());
        res.extend(payload);
        res.extend(buffered);
        res
    };

    bs58::encode(res).into_string()
}

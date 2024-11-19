use core::panic;
use soroban_sdk::{Env, BytesN};

use crate::storage_types::DataKey;

pub fn has_land_wasm_hash(e: &Env) -> bool {
    let key = DataKey::LandWASMHash;
    e.storage().instance().has(&key)
}

pub fn write_land_wasm_hash(e: &Env, land_wasm_hash: &BytesN<32>) {
    let key = DataKey::LandWASMHash;
    e.storage().instance().set(&key, land_wasm_hash);
}

pub fn read_land_wasm_hash(e: &Env) -> BytesN<32> {
    if !has_land_wasm_hash(&e) {
        panic!("land wasm hash not set");
    }
    let key = DataKey::LandWASMHash;
    e.storage().instance().get(&key).unwrap()
}

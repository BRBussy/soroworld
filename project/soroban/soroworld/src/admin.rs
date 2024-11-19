use core::panic;
use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;

pub fn has_admin(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().instance().has(&key)
}

pub fn read_admin(env: &Env) -> Address {
    if !has_admin(&env) {
        panic!("admin not set");
    }
    
    let key = DataKey::Admin;
    env.storage().instance().get(&key).unwrap()
}

pub fn write_admin(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().instance().set(&key, id);
}

pub fn require_admin_auth(env: &Env) {
    read_admin(&env).require_auth();
}
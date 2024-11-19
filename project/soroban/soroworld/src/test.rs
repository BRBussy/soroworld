#![cfg(test)]

use crate::{contract::Soroworld, SoroworldClient, storage_types::DataKey};
use soroban_sdk::{
    Env,
    testutils::Address as _,
    Address,
};

fn create_soroworld<'a>(e: &Env, admin: &Address) -> SoroworldClient<'a> {
    let soroworld = SoroworldClient::new(e, &e.register_contract(None, Soroworld{}));
    soroworld.init(admin);
    soroworld
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    // prepare keys
    let admin = Address::generate(&e);

    // prepare instance of the soroworld contract
    let soroworld = create_soroworld(&e, &admin);

    // check admin
    assert_eq!(
        admin,
        e.as_contract(&soroworld.address, || {
            let key = DataKey::Admin;
            e.storage().instance().get::<_, Address>(&key).unwrap()
        }),
    );
}

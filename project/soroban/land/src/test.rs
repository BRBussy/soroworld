#![cfg(test)]
extern crate std;

use crate::{contract::Land, LandClient, storage_types::DataKey};
use soroban_sdk::{
    String,
    testutils::Address as _,
    Address, Env
};

fn create_land<'a>(e: &Env, admin: &Address) -> LandClient<'a> {
    let land = LandClient::new(e, &e.register_contract(None, Land {}));
    land.init(admin);
    land
}

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    // prepare keys
    let admin = Address::generate(&env);
    let other_key = Address::generate(&env);

    // prepare instance of the land contract
    let land = create_land(
        &env,
        &admin,
    );

    // check admin
    assert_eq!(
        admin,
        env.as_contract(&land.address, || {
            let key = DataKey::Admin;
            env.storage().instance().get::<_, Address>(&key).unwrap()
        }),
    );

    // check token metadata
    assert_eq!(
        0,
        land.decimals(),
    );
    assert_eq!(
        String::from_str(&env, "Soroworld Land"),
        land.name(),
    );
    assert_eq!(
        String::from_str(&env, "SRWLDLAND"),
        land.symbol(),
    );    

    // balance check
    assert_eq!(
        1,
        land.balance(&admin),
    );
    assert_eq!(
        0,
        land.balance(&other_key),
    );

    // transfer to other user
    land.transfer(&admin, &other_key, &1);

    // check admin moved
    assert_eq!(
        other_key,
        env.as_contract(&land.address, || {
            let key = DataKey::Admin;
            env.storage().instance().get::<_, Address>(&key).unwrap()
        }),
    );

    // balance check
    assert_eq!(
        0,
        land.balance(&admin),
    );
    assert_eq!(
        1,
        land.balance(&other_key),
    );
}

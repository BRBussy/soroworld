#![cfg(test)]

use crate::{contract::Soroworld, SoroworldClient, storage_types::DataKey, storage_types::Coordinates, current_land_coordinates::increment_coordinates_for_test};
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

#[test]
fn test_increment_coordinates_for_test() {
    let mut depth: u128 = 0;
    let mut coordinates: Coordinates = Coordinates{x: 0, y: 0};

    let test_cases = [
        // depth 1
        (true, Coordinates{x: 1, y: 0}),
        (false, Coordinates{x: 1, y: 1}),
        (false, Coordinates{x: 0, y: 1}),

        // depth 2
        (true, Coordinates{x: 0, y: 2}),
        (false, Coordinates{x: 1, y: 2}),
        (false, Coordinates{x: 2, y: 2}),
        (false, Coordinates{x: 2, y: 1}),
        (false, Coordinates{x: 2, y: 0}),

        // depth 3
        (true, Coordinates{x: 3, y: 0}),
        (false, Coordinates{x: 3, y: 1}),
        (false, Coordinates{x: 3, y: 2}),
        (false, Coordinates{x: 3, y: 3}),
        (false, Coordinates{x: 2, y: 3}),
        (false, Coordinates{x: 1, y: 3}),  
        (false, Coordinates{x: 0, y: 3}),

        // depth 4
        (true, Coordinates{x: 0, y: 4}),
        (false, Coordinates{x: 1, y: 4}),
        (false, Coordinates{x: 2, y: 4}),
        (false, Coordinates{x: 3, y: 4}),
        (false, Coordinates{x: 4, y: 4}),
        (false, Coordinates{x: 4, y: 3}),
        (false, Coordinates{x: 4, y: 2}),
        (false, Coordinates{x: 4, y: 1}),
        (false, Coordinates{x: 4, y: 0}),
    ];

    for (expected_increment, expected_coordinates) in test_cases.iter() {
        let increment = increment_coordinates_for_test(
            &mut coordinates,
            depth,
        );
        assert_eq!(
            *expected_increment,
            increment,
        );
        assert_eq!(
            *expected_coordinates,
            coordinates,
        );

        if increment {
            depth+=1;
        }
    }
}
#![cfg(test)]
extern crate std;

use crate::{contract::Land, LandClient, storage_types::DataKey, storage_types::Coordinates};
use soroban_sdk::{
    String,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, Symbol,
};

fn create_land<'a>(e: &Env, admin: &Address, coordinates: &Coordinates) -> LandClient<'a> {
    let land = LandClient::new(e, &e.register_contract(None, Land {}));
    land.initialize(admin, coordinates);
    land
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    // prepare keys
    let admin = Address::generate(&e);
    let other_key = Address::generate(&e);

    // prepare instance of the land contract
    let land = create_land(
        &e,
        &admin,
        &Coordinates { x: 12, y: 11 }
    );

    // check admin
    assert_eq!(
        admin,
        e.as_contract(&land.address, || {
            let key = DataKey::Admin;
            e.storage().instance().get::<_, Address>(&key).unwrap()
        }),
    );

    // check coordinates
    assert_eq!(
        Coordinates { x: 12, y: 11 },
        e.as_contract(&land.address, || {
            let key = DataKey::Coordinates;
            e.storage().instance().get::<_, Coordinates>(&key).unwrap()
        }),
    );

    // check token metadata
    assert_eq!(
        0,
        land.decimals(),
    );
    assert_eq!(
        String::from_str(&e, "Soroworld Land"),
        land.name(),
    );
    assert_eq!(
        String::from_str(&e, "SRWLDLAND"),
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
        e.as_contract(&land.address, || {
            let key = DataKey::Admin;
            e.storage().instance().get::<_, Address>(&key).unwrap()
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

    // token.mint(&user1, &1000);
    // assert_eq!(
    //     e.auths(),
    //     std::vec![(
    //         admin1.clone(),
    //         AuthorizedInvocation {
    //             function: AuthorizedFunction::Contract((
    //                 token.address.clone(),
    //                 symbol_short!("mint"),
    //                 (&user1, 1000_i128).into_val(&e),
    //             )),
    //             sub_invocations: std::vec![]
    //         }
    //     )]
    // );
    // assert_eq!(token.balance(&user1), 1000);

    // token.approve(&user2, &user3, &500, &200);
    // assert_eq!(
    //     e.auths(),
    //     std::vec![(
    //         user2.clone(),
    //         AuthorizedInvocation {
    //             function: AuthorizedFunction::Contract((
    //                 token.address.clone(),
    //                 symbol_short!("approve"),
    //                 (&user2, &user3, 500_i128, 200_u32).into_val(&e),
    //             )),
    //             sub_invocations: std::vec![]
    //         }
    //     )]
    // );
    // assert_eq!(token.allowance(&user2, &user3), 500);

    // token.transfer(&user1, &user2, &600);
    // assert_eq!(
    //     e.auths(),
    //     std::vec![(
    //         user1.clone(),
    //         AuthorizedInvocation {
    //             function: AuthorizedFunction::Contract((
    //                 token.address.clone(),
    //                 symbol_short!("transfer"),
    //                 (&user1, &user2, 600_i128).into_val(&e),
    //             )),
    //             sub_invocations: std::vec![]
    //         }
    //     )]
    // );
    // assert_eq!(token.balance(&user1), 400);
    // assert_eq!(token.balance(&user2), 600);

    // token.transfer_from(&user3, &user2, &user1, &400);
    // assert_eq!(
    //     e.auths(),
    //     std::vec![(
    //         user3.clone(),
    //         AuthorizedInvocation {
    //             function: AuthorizedFunction::Contract((
    //                 token.address.clone(),
    //                 Symbol::new(&e, "transfer_from"),
    //                 (&user3, &user2, &user1, 400_i128).into_val(&e),
    //             )),
    //             sub_invocations: std::vec![]
    //         }
    //     )]
    // );
    // assert_eq!(token.balance(&user1), 800);
    // assert_eq!(token.balance(&user2), 200);

    // token.transfer(&user1, &user3, &300);
    // assert_eq!(token.balance(&user1), 500);
    // assert_eq!(token.balance(&user3), 300);

    // token.set_admin(&admin2);
    // assert_eq!(
    //     e.auths(),
    //     std::vec![(
    //         admin1.clone(),
    //         AuthorizedInvocation {
    //             function: AuthorizedFunction::Contract((
    //                 token.address.clone(),
    //                 symbol_short!("set_admin"),
    //                 (&admin2,).into_val(&e),
    //             )),
    //             sub_invocations: std::vec![]
    //         }
    //     )]
    // );

    // // Increase to 500
    // token.approve(&user2, &user3, &500, &200);
    // assert_eq!(token.allowance(&user2, &user3), 500);
    // token.approve(&user2, &user3, &0, &200);
    // assert_eq!(
    //     e.auths(),
    //     std::vec![(
    //         user2.clone(),
    //         AuthorizedInvocation {
    //             function: AuthorizedFunction::Contract((
    //                 token.address.clone(),
    //                 symbol_short!("approve"),
    //                 (&user2, &user3, 0_i128, 200_u32).into_val(&e),
    //             )),
    //             sub_invocations: std::vec![]
    //         }
    //     )]
    // );
    // assert_eq!(token.allowance(&user2, &user3), 0);
}

// #[test]
// fn test_burn() {
//     let e = Env::default();
//     e.mock_all_auths();

//     let admin = Address::generate(&e);
//     let user1 = Address::generate(&e);
//     let user2 = Address::generate(&e);
//     let token = create_land(&e, &admin);

//     token.mint(&user1, &1000);
//     assert_eq!(token.balance(&user1), 1000);

//     token.approve(&user1, &user2, &500, &200);
//     assert_eq!(token.allowance(&user1, &user2), 500);

//     token.burn_from(&user2, &user1, &500);
//     assert_eq!(
//         e.auths(),
//         std::vec![(
//             user2.clone(),
//             AuthorizedInvocation {
//                 function: AuthorizedFunction::Contract((
//                     token.address.clone(),
//                     symbol_short!("burn_from"),
//                     (&user2, &user1, 500_i128).into_val(&e),
//                 )),
//                 sub_invocations: std::vec![]
//             }
//         )]
//     );

//     assert_eq!(token.allowance(&user1, &user2), 0);
//     assert_eq!(token.balance(&user1), 500);
//     assert_eq!(token.balance(&user2), 0);

//     token.burn(&user1, &500);
//     assert_eq!(
//         e.auths(),
//         std::vec![(
//             user1.clone(),
//             AuthorizedInvocation {
//                 function: AuthorizedFunction::Contract((
//                     token.address.clone(),
//                     symbol_short!("burn"),
//                     (&user1, 500_i128).into_val(&e),
//                 )),
//                 sub_invocations: std::vec![]
//             }
//         )]
//     );

//     assert_eq!(token.balance(&user1), 0);
//     assert_eq!(token.balance(&user2), 0);
// }

// #[test]
// #[should_panic(expected = "insufficient balance")]
// fn transfer_insufficient_balance() {
//     let e = Env::default();
//     e.mock_all_auths();

//     let admin = Address::generate(&e);
//     let user1 = Address::generate(&e);
//     let user2 = Address::generate(&e);
//     let token = create_land(&e, &admin);

//     token.mint(&user1, &1000);
//     assert_eq!(token.balance(&user1), 1000);

//     token.transfer(&user1, &user2, &1001);
// }

// #[test]
// #[should_panic(expected = "insufficient allowance")]
// fn transfer_from_insufficient_allowance() {
//     let e = Env::default();
//     e.mock_all_auths();

//     let admin = Address::generate(&e);
//     let user1 = Address::generate(&e);
//     let user2 = Address::generate(&e);
//     let user3 = Address::generate(&e);
//     let token = create_land(&e, &admin);

//     token.mint(&user1, &1000);
//     assert_eq!(token.balance(&user1), 1000);

//     token.approve(&user1, &user3, &100, &200);
//     assert_eq!(token.allowance(&user1, &user3), 100);

//     token.transfer_from(&user3, &user1, &user2, &101);
// }

// #[test]
// #[should_panic(expected = "already initialized")]
// fn initialize_already_initialized() {
//     let e = Env::default();
//     let admin = Address::generate(&e);
//     let token = create_land(&e, &admin);

//     token.initialize(&admin, &10, &"name".into_val(&e), &"symbol".into_val(&e));
// }

// #[test]
// #[should_panic(expected = "Decimal must not be greater than 18")]
// fn decimal_is_over_eighteen() {
//     let e = Env::default();
//     let admin = Address::generate(&e);
//     let token = LandClient::new(&e, &e.register_contract(None, Land {}));
//     token.initialize(&admin, &19, &"name".into_val(&e), &"symbol".into_val(&e));
// }

// #[test]
// fn test_zero_allowance() {
//     // Here we test that transfer_from with a 0 amount does not create an empty allowance
//     let e = Env::default();
//     e.mock_all_auths();

//     let admin = Address::generate(&e);
//     let spender = Address::generate(&e);
//     let from = Address::generate(&e);
//     let token = create_land(&e, &admin);

//     token.transfer_from(&spender, &from, &spender, &0);
//     assert!(token.get_allowance(&from, &spender).is_none());
// }

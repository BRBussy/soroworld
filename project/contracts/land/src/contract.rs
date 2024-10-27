//! This contract demonstrates a sample implementation of the Soroban token
//! interface.
use core::panic;

use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::metadata::{read_name, read_symbol, write_metadata};
use crate::storage_types::INSTANCE_LIFETIME_THRESHOLD;
use crate::storage_types::INSTANCE_BUMP_AMOUNT;
use soroban_sdk::token::{self, Interface as _};
use soroban_sdk::{contract, contractimpl, Address, Env, String};
use soroban_token_sdk::metadata::TokenMetadata;
use soroban_token_sdk::TokenUtils;

#[contract]
pub struct Land;

#[contractimpl]
impl Land {
    pub fn initialize(e: Env, admin: Address, x: u32, y: u32) {
        if has_administrator(&e) {
            panic!("already initialized")
        }

        write_administrator(&e, &admin);

        write_metadata(
            &e,
            TokenMetadata {
                decimal: 0,
                // FIXME: include x and y cooordinate here
                // FIXME: add world name maybe
                name: String::from_str(&e, "Soroworld Land"),
                // FIXME: allow world short name to be used here
                symbol: String::from_str(&e, "SRWLDLAND"),
            },
        )
    }
}

#[contractimpl]
impl token::Interface for Land {
    fn allowance(_e: Env, _from: Address, _spender: Address) -> i128 {
        panic!("not implemented")
    }

    fn approve(_e: Env, _from: Address, _spender: Address, _amount: i128, _expiration_ledger: u32) {
        panic!("not implemented")
    }

    fn balance(e: Env, id: Address) -> i128 {
        let admin = read_administrator(&e);

        if admin == id {
            1
        } else {
            0
        }
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        // confirm that contract is initialised
        if !has_administrator(&e) {
            panic!("not initialised")
        }

        // get the administrator
        let admin = read_administrator(&e);

        // only the administrator can transfer
        if from != admin {
            panic!("only administrator can transfer");
        }

        // confirm from (i.e. admin) has signed
        from.require_auth();

        // amount must be 1
        if amount != 1 {
            panic!("amount must be 1")
        }

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);


        // update administrator to implement transfer
        write_administrator(&e, &to);

        // emit token transfered event
        TokenUtils::new(&e).events().transfer(from, to, amount);
    }

    fn transfer_from(_e: Env, _spender: Address, _from: Address, _to: Address, _amount: i128) {
       panic!("not implemented")
    }

    fn burn(_e: Env, _from: Address, _amount: i128) {
        panic!("we cannot be burning land")
    }

    fn burn_from(_e: Env, _spender: Address, _from: Address, _amount: i128) {
        panic!("we cannot be burning land")
    }

    fn decimals(_e: Env) -> u32 {
        0
    }

    fn name(e: Env) -> String {
        read_name(&e)
    }

    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }
}

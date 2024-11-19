use core::panic;
use soroban_sdk::{contract, contractimpl, Address, Env, BytesN};
use crate::admin::{write_admin, has_admin, require_admin_auth};
use crate::land_wasm_hash::{write_land_wasm_hash, read_land_wasm_hash};
use crate::current_land_coordinates::increment_current_land_coordinate;

#[contract]
pub struct Soroworld;

#[contractimpl]
impl Soroworld {
    pub fn init(
        env: Env,
        admin: Address,
    ) {
        if has_admin(&env) {
            panic!("already initialised")
        }
        write_admin(&env, &admin);
    }

    pub fn set_land_wasm(
        env: Env,
        land_wasm_hash: BytesN<32>,
    ) {
        require_admin_auth(&env);
        write_land_wasm_hash(&env, &land_wasm_hash);
    }    

    pub fn mint_land(
        env: Env,
        owner: Address,
    ) -> Address {
            // the new land owner needs to have signed
            owner.require_auth();

            // increment land coordinates
            let next_coordinate = increment_current_land_coordinate(&env);

            // deploy new land contract using the given land hash
            let deployed_address = env
            .deployer()
            .with_address(
                env.current_contract_address(),
                read_land_wasm_hash(&env),
            )
            .deploy(read_land_wasm_hash(&env));

            deployed_address
    }
}

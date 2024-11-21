use core::panic;
use soroban_sdk::{contract, contractimpl, vec, Address, BytesN, Env, Symbol, Val};
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

            // prepare salt with x and y coordinates next to one another
            let mut salt = [0u8; 32];
            salt[..16].copy_from_slice(&next_coordinate.x.to_le_bytes());
            salt[16..].copy_from_slice(&next_coordinate.y.to_le_bytes());

            // deploy new land contract using stored land hash
            // NOTE: address of new contract is predictable based on:
            // - this contract address (deployer of new land)
            // - x & y coordinates as salt
            let deployed_address = env
            .deployer()
            .with_address(
                env.current_contract_address(),
                BytesN::<32>::from_array(&env, &salt),
            )
            .deploy(read_land_wasm_hash(&env));

            // initialise new land with owner
            let _: Val = env.invoke_contract(
                &deployed_address,
                &Symbol::new(&env, "init"),
                vec![&env, owner.to_val()],
            );

            // and return address of deployed land
            deployed_address
    }
}

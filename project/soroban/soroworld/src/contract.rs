use soroban_sdk::{contract, contractimpl, Address, Env, BytesN};
use super::storage_types::DataKey;
use super::admin::write_admin;

#[contract]
pub struct Soroworld;

#[contractimpl]
impl Soroworld {
    pub fn init(
        env: Env,
        admin: Address,
    ) {
        write_admin(&env, &admin);
    }

    pub fn mint_land(
        env: Env,
        owner: Address,
        land_wasm_hash: BytesN<32>,
    ) -> Address {
            // deploy new land contract using the given land hash
            let deployed_address = env
            .deployer()
            .with_address(
                env.current_contract_address(),
                [
                    1,2,3,4,5,6,7,8,9,10,
                    1,2,3,4,5,6,7,8,9,10,
                    1,2,3,4,5,6,7,8,9,10,
                    3,4
                ],
            )
            .deploy(land_wasm_hash);

            deployed_address
    }
}

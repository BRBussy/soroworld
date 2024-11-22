echo "Build Contracts"
stellar contract build

echo "Optimise Contracts"
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/soroworld_world_contract.wasm
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/soroworld_land_contract.wasm

echo "Deploying with alice"
stellar keys address alice

echo "Install Contracts"
stellar contract install \
    --network testnet \
    --source alice \
    --wasm target/wasm32-unknown-unknown/release/soroworld_world_contract.optimized.wasm
stellar contract install \
    --network testnet \
    --source alice \
    --wasm target/wasm32-unknown-unknown/release/soroworld_land_contract.optimized.wasm
    
        
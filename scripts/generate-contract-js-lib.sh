# build js project
echo "Prepare @soroworld/land-js build dir"
mkdir project/soroban/land/js-build
echo "Run @soroworld/land-js code gen"
stellar contract bindings typescript \
    --output-dir project/soroban/land/js-build \
    --wasm target/wasm32-unknown-unknown/release/soroworld_land_contract.wasm  \
    --id CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
    --overwrite
echo "Move @soroworld/land-js to project"
mv project/soroban/land/js-build/src/index.ts project/soroban/land/js/src/index.soroworldgen.ts

echo "Clean..."
rm -rf project/soroban/land/js-build
rm -rf project/soroban/land/js/dist

echo "Build @soroworld/land-js"
yarn workspace @soroworld/land-js build
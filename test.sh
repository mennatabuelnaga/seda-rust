cd ./wasm/cli

cargo build

cd ..
cd ./test/demo-cli/

cargo build

cd ..
cd ./promise-wasm-bin/

cargo build

cd ..

cargo test --workspace --exclude cli --exclude demo-cli --exclude promise-wasm-bin
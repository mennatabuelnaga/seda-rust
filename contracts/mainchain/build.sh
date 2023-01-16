#!/usr/bin/env sh
TARGET="${CARGO_TARGET_DIR:-../target}"
set -e
cd "$(dirname $0)"
cargo build --target wasm32-unknown-unknown --release
cp $TARGET/wasm32-unknown-unknown/release/seda_mainchain.wasm ./res/

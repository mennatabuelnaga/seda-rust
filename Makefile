.PHONY: build build-wasm check clean fmt run run-build run-build-all run-build-wasm test test-build wasm

MKFILE_PATH := $(abspath $(lastword $(MAKEFILE_LIST)))
MKFILE_DIR := $(dir $(MKFILE_PATH))

SEDA_BIN := seda
SEDA_BIN_PATH := $(MKFILE_DIR)target/debug/$(SEDA_BIN)

WASM_MODULES := $(notdir $(filter-out $(MKFILE_DIR)wasm/test,$(wildcard $(MKFILE_DIR)wasm/*)))
WASM_TEST_MODULES := $(notdir $(wildcard $(MKFILE_DIR)wasm/test/*))

build:
	cargo build

build-wasm: wasm
	cargo build

check:
	RUSTFLAGS="-D warnings" cargo clippy --all-features

clean:
	cargo clean

fmt:
	cargo +nightly fmt --all

# Just runs the prebuilt binary.
run:
	$(SEDA_BIN_PATH) $(ARGS)

# Builds only seda-before running
run-build: build
	$(SEDA_BIN_PATH) $(ARGS)

# Builds everything before running
run-build-all: build-wasm
	$(SEDA_BIN_PATH) $(ARGS)

# Builds only the wasm's before re-running
run-build-wasm: wasm
	$(SEDA_BIN_PATH) $(ARGS)

test:
	cargo test --workspace --exclude demo-cli --exclude seda-cli --exclude promise-wasm-bin

test-build: wasm
	cargo test --workspace --exclude demo-cli --exclude seda-cli --exclude promise-wasm-bin

wasm:
	$(foreach module, $(WASM_MODULES), cargo build -p $(module) --target wasm32-wasi;)
	$(foreach module, $(WASM_TEST_MODULES), cargo build -p $(module) --target wasm32-wasi;)

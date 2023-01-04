<p align="center">
  <a href="https://seda.xyz/">
    <img width="90%" alt="seda-rust" src="link to seda icon">
  </a>
</p>

<h3 align="center">
   Seda Rust
</h3>

# seda-rust

Open source Rust implementation of the SEDA Protocol.
Designed to run on the NEAR chain.

## Dependencies

### [Rust](https://www.rust-lang.org/tools/install)

- [rustup](https://www.rust-lang.org/tools/install)
- `stable` toolchain
	- `rustup install stable`
	- `rustup default stable`
- `nightly` toolchain
	- `rustup install stable`
- `wasm32-wasi` toolchain
	- `cargo install cargo-wasi`

### [Make](https://www.gnu.org/software/make/)

- Windows
	- install [scoop](https://scoop.sh/)
	- `scoop bucket add main`
	- `scoop install make`
- macOS
	- `xcode-select --install` or with brew `brew install make`
- Linux
	- Likely already comes installed.
	- However, please see your distribution specific installer.


## Developing

For how to build, run, format, and clean the code base.
To learn how to contribute please read [here](CONTRIBUTING.md).

### Build Commands

- `make build` builds only the `seda` binary.
- `make build-wasm` builds the wasm binaries and the `seda` binary.
- `make wasm` builds the wasm binaries.

### Formatting & Cleanliness Commands

- `make clean` runs `cargo clean`
- `make check` runs `clippy` with the deny flag like in our CI.
- `make fmt` runs `cargo +nightly fmt --all`

### Run Commands

Be sure to [configure](#configuration) your node before running.

The run commands takes additional arguments to pass to the node. For an example of how to pass arguments `make run -- --help`.

For more command documentation please see the documentation [here](CLI.md).

- `make run` runs the already built binary but has no dependencies, so you have to run `build-wasm` first or `make run-build-all`.
- `make run-build` builds `seda` and then runs it.
- `make run-build-all` builds the wasm binaries, then `seda`, and finally runs.
- `make run-build-wasm` builds the wasm binaries, then runs `seda`.

### Test Commands

- `make test` runs `cargo test --workspace --exclude demo-cli --exclude seda-cli --exclude promise-wasm-bin` and assumes the wasm binaries have already been built.
- `make test-build` builds the wasm binaries and then runs the same command as `make test`.

## Configuration

### TOML File

Config is written in [TOML](https://toml.io/en/) and read by default from your operating systems app config location.

- Unix Systems(Mac & Linux) `/etc/seda-rust/config.toml`
- Windows `C:\\ProgramData\\seda-rust\\config.toml`

A default configuration is generated on first run, though some fields are needed to be filled in either via the command line interface or from environment variables.

Some fields below are not required, as they have defaults, or can be passed through [environment variables](#env) or the [CLI](CLI.md). For the sake of documentation here the names of variables will be followed by parentheses with symbols inside:
- ? - means the field is entirely optional and has a default value.
- ! - means its overwrittable by an environment variable.
- \* - means its overwrittable by the cli.

You can look at the example configuration [here](example.config.toml).

#### Fields

<!-- TODO need to specify which are overwriteable by CLI once its merged and config changes. -->
- seda_server_url(!) - Defines the URL for seda to run it's RPC server on.
<!-- TODO this should be for testing only but for now its required sooo
	- another_chain
	- chain_rpc_url(!) -->
- near_chain - All config fields related to the near chain.
	- chain_rpc_url(!) - The near server URL.
- node - All config fields related to the seda node.
	- contract_account_id(!) - Your near contract account id.
	- deposit(?!) - The deposit amount.
	- gas(?!) - The gas amount.
	- job_manager_interval_ms(?) - How often the node runs jobs.
	- p2p_server_address - The address to run the p2p server on.
	- p2p_known_peers - The list of known peers for the node.
	- public_key(!) - Your near public key.
	- rpc_server_address(!) - The same as the seda_server_url without the `ws://`.
	- runtime_worker_threads(?) - The number of threads the node can use to spin up jobs.
	- secret_key(!) - Your near secret key.
	- signer_account_id(!) - Your near signer account id.
- logging - All config fields related to the seda logger.
	- log_file_path(!) - The path where the log file will write.

### ENV

Seda configuration uses the follow ENV variables if they exist.

|Name|Description|
|-|-|
|`CONTRACT_ACCOUNT_ID`|Overwrites the config `node.contract_account_id` field.|
|`DEPOSIT`|Overwrites the config `node.deposit` field.|
|`GAS`|Overwrites the config `node.gas` field.|
|`LOG_FILE_PATH`|Overwrites the config `logging.log_file_path` field.|
|`NEAR_SERVER_URL`|Overwrites the config `near_chain.chain_rpc_url` field.|
|`PUBLIC_KEY`|Overwrites the config `node.public_key` field.|
|`SECRET_KEY`|Overwrites the config `node.secret_key` field.|
|`SEDA_CONFIG_PATH`|Defines an alternative path for the seda configuration file to be.|
|`SEDA_SERVER_URL`|Overwrites the config `seda_server_url` field.|
|`SIGNER_ACCOUNT_ID`|Overwrites the config `node.signer_account_id` field.|
|`RUST_LOG`|Controlled via the [tracing_subscriber](https://docs.rs/tracing-subscriber/0.3.16/tracing_subscriber/struct.EnvFilter.html) crate.|

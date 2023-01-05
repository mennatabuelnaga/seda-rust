<p align="center">
  <a href="https://seda.xyz/">
    <img width="90%" alt="seda-rust" src="link to seda icon">
  </a>
</p>

<h3 align="center">
   Seda Rust
</h3>

# seda-rust

Open source Rust implementation of the SEDA Protocol. Designed to run on the
NEAR chain.

**NOTE** this repo adheres to the [GPLv3 license](LICENSE.md).

To learn how to build a local version, please read [developing](DEVELOPING.md).
To learn how to contribute, please read [contributing](CONTRIBUTING.md).

## Dependencies

There are currently no dependencies.

## Configuration

The configuration for the `node` and the `CLI` commands.

### TOML File

Config is written in [TOML](https://toml.io/en/) and read by default from your
operating systems app config location.

- Unix Systems(Mac & Linux) `/etc/seda-rust/config.toml`
- Windows `C:\\ProgramData\\seda-rust\\config.toml`

On the first run, a default configuration generates. You need to specify some
fields via the command line interface or from environment variables.

Some fields below are not required, as they have default values, or you can pass
them through the [environment variables](#env) or the [CLI](CLI.md). For the
sake of documentation, here the names of variables will be followed by
parentheses with symbols inside:

- ? - means the field is entirely optional and has a default value.
- ! - means it's overwritable by an environment variable.
- \* - means it's overwritable by the cli.

You can look at the example configuration [here](example.config.toml).

#### Fields

<!-- TODO needs to specify which are overwritable by CLI once it's merged and config changes. -->

- seda_server_url(!) - Defines the URL for seda to run its RPC server on.

<!-- TODO this should be for testing only but for now, it's required, so we should remove this before we go public.
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
  - runtime_worker_threads(?) - The number of threads the node can use to spin
    up jobs.
  - secret_key(!) - Your near secret key.
  - signer_account_id(!) - Your near signer account id.
- logging - All config fields related to the seda logger.
  - log_file_path(!) - The path where the log file will write.

### ENV

Seda configuration uses the following ENV variables if they exist.

| Name                  | Description                                                                                                                        |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `CONTRACT_ACCOUNT_ID` | Overwrites the config `node.contract_account_id` field.                                                                            |
| `DEPOSIT`             | Overwrites the config `node.deposit` field.                                                                                        |
| `GAS`                 | Overwrites the config `node.gas` field.                                                                                            |
| `LOG_FILE_PATH`       | Overwrites the config `logging.log_file_path` field.                                                                               |
| `NEAR_SERVER_URL`     | Overwrites the config `near_chain.chain_rpc_url` field.                                                                            |
| `PUBLIC_KEY`          | Overwrites the config `node.public_key` field.                                                                                     |
| `SECRET_KEY`          | Overwrites the config `node.secret_key` field.                                                                                     |
| `SEDA_CONFIG_PATH`    | Defines an alternative path for the seda configuration file to be.                                                                 |
| `SEDA_SERVER_URL`     | Overwrites the config `seda_server_url` field.                                                                                     |
| `SIGNER_ACCOUNT_ID`   | Overwrites the config `node.signer_account_id` field.                                                                              |
| `RUST_LOG`            | Controlled via the [tracing_subscriber](https://docs.rs/tracing-subscriber/0.3.16/tracing_subscriber/struct.EnvFilter.html) crate. |

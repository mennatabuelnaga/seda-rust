<p align="center">
  <a href="https://seda.xyz/">
    <img width="90%" alt="seda-rust" src="https://www.seda.xyz/images/footer/footer-image.png">
  </a>
</p>

<h1 align="center">
   Seda Rust
</h1>

[![Build Status][actions-badge]][actions-url]
[![GitHub Stars][github-stars-badge]](https://github.com/sedaprotocol/seda-rust)
[![GitHub Contributors][github-contributors-badge]](https://github.com/sedaprotocol/seda-rust/graphs/contributors)
[![Discord chat][discord-badge]][discord-url]
[![Twitter][twitter-badge]][twitter-url]

<!-- once we publish seda:
[![Crates.io][crates-badge]][crates-url]
[crates-badge]: https://img.shields.io/crates/v/seda
[crates-url]: https://crates.io/crates/seda
 -->

[actions-badge]: https://github.com/sedaprotocol/seda-rust/actions/workflows/push.yml/badge.svg
[actions-url]: https://github.com/sedaprotocol/seda-rust/actions/workflows/push.yml+branch%3Amain
[github-stars-badge]: https://img.shields.io/github/stars/sedaprotocol/seda-rust.svg?style=flat-square&label=github%20stars
[github-contributors-badge]: https://img.shields.io/github/contributors/sedaprotocol/seda-rust.svg?style=flat-square
[discord-badge]: https://img.shields.io/discord/500028886025895936.svg?logo=discord&style=flat-square
[discord-url]: https://discord.gg/seda
[twitter-badge]: https://img.shields.io/twitter/url/https/twitter.com/SedaProtocol.svg?style=social&label=Follow%20%40SedaProtocol
[twitter-url]: https://twitter.com/SedaProtocol

Open source Rust implementation of the
[SEDA Protocol](https://docs.seda.xyz/seda-network/introduction/the-oracle-problem).

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

- seda_server_address(?!) - Defines the address for seda to run its RPC server on.
- seda_server_port(?!) - Defines the port for seda to run its RPC server on.
- chains - All config fields related to the supported chains.
  - near - All config fields related to the near chain.
    - chain_rpc_url(!\*) - The near server URL.
- node - All config fields related to the seda node.
  - contract_account_id(\*) - Your near contract account id.
  - deposit(?\*) - The deposit amount.
  - gas(?\*) - The gas amount.
  - job_manager_interval_ms(?\*) - How often the node runs jobs.
  - p2p_server_address(?\*) - The address to run the p2p server on.
  - p2p_known_peers(?\*) - The list of known peers for the node.
  - public_key(\*) - Your near public key.
  - runtime_worker_threads(?\*) - The number of threads the node can use to spin
    up jobs.
  - secret_key(!\*) - Your near secret key.
  - signer_account_id(\*) - Your near signer account id.
- logging - All config fields related to the seda logger.
  - log_file_path(?!\*) - The path where the log file will write.

### ENV

Seda configuration uses the following ENV variables if they exist.

| Name                  | Description                                                                                                                        |
| --------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| `SEDA_CONFIG_PATH`    | Defines an alternative path for the seda configuration file to be.                                                                 |
| `SEDA_LOG_FILE_PATH`  | Overwrites the config `logging.log_file_path` field.                                                                               |
| `SEDA_NEAR_RPC_URL`   | Overwrites the config `near_chain.chain_rpc_url` field.                                                                            |
| `SEDA_SECRET_KEY`     | Overwrites the config `node.secret_key` field.                                                                                     |
| `SEDA_SERVER_ADDRESS` | Overwrites the config `seda_server_address` field.                                                                                 |
| `SEDA_SERVER_PORT`    | Overwrites the config `seda_server_port` field.                                                                                    |
| `RUST_LOG`            | Controlled via the [tracing_subscriber](https://docs.rs/tracing-subscriber/0.3.16/tracing_subscriber/struct.EnvFilter.html) crate. |

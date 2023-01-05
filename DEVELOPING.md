# Developing

For setting up your environment to develop `seda-rust`. Shows how to build, run,
format, and clean the code base. To learn how to contribute please read
[here](CONTRIBUTING.md).

## Dev-Container

If you are using [VSCode](https://code.visualstudio.com/) and
[docker](https://www.docker.com/) you can open the project in a dev
[container](https://containers.dev/) where all deps will be installed already.
Otherwise please see the [dev dependencies](#dev-dependencies).

## Dev Dependencies

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

## Building

- `make build` builds only the `seda` binary.
- `make build-wasm` builds the wasm binaries and the `seda` binary.
- `make wasm` builds the wasm binaries.

## Formatting & Cleanliness

- `make clean` runs `cargo clean`
- `make check` runs `clippy` with the deny flag like in our CI.
- `make fmt` runs `cargo +nightly fmt --all`

## Running

Be sure to [configure](#configuration) your node before running.

The run commands takes additional arguments to pass to the node. For an example
of how to pass arguments `make run -- --help`.

For more command documentation please see the documentation [here](CLI.md).

- `make run` runs the already built binary but has no dependencies, so you have
  to run `build-wasm` first or `make run-build-all`.
- `make run-build` builds `seda` and then runs it.
- `make run-build-all` builds the wasm binaries, then `seda`, and finally runs.
- `make run-build-wasm` builds the wasm binaries, then runs `seda`.

## Testing

- `make test` runs
  `cargo test --workspace --exclude demo-cli --exclude seda-cli --exclude promise-wasm-bin`
  and assumes the wasm binaries have already been built.
- `make test-build` builds the wasm binaries and then runs the same command as
  `make test`.

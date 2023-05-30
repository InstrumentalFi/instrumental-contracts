# Pablo Vault

[![Artifacts](https://github.com/shapeshed/pablo-vault/actions/workflows/artifacts.yml/badge.svg)](https://github.com/shapeshed/pablo-vault/actions/workflows/artifacts.yml)
[![Main](https://github.com/shapeshed/pablo-vault/actions/workflows/main.yml/badge.svg)](https://github.com/shapeshed/pablo-vault/actions/workflows/main.yml)
[![Coverage](https://github.com/shapeshed/pablo-vault/actions/workflows/coverage.yml/badge.svg)](https://github.com/shapeshed/pablo-vault/actions/workflows/coverage.yml)
[![codecov](https://codecov.io/github/shapeshed/pablo-vault/branch/main/graph/badge.svg?token=dH6ikLs46M)](https://codecov.io/github/shapeshed/pablo-vault)

This repository contains the source code for the Pablo Vault and Fee Distribution Contracts.

## Environment set up

- Install [rustup][4]. Once installed, make sure you have the wasm32 target:

  ```bash
  rustup default stable
  rustup update stable
  rustup target add wasm32-unknown-unknown
  ```

- Install [cargo-make][5]

  ```bash
  cargo install --force cargo-make
  ```

- Install [Docker][6]

- Install [Node.js v16][7]

- Install [Yarn][8]

- Create the build folder:

```bash
yarn build
```

- Compile all contracts:

```bash
cargo make rust-optimizer
```

- Formatting:

```bash
yarn format
yarn lint
```

This compiles and optimizes all contracts, storing them in `/artifacts` directory along with `checksum.txt` which contains sha256 hashes of each of the `.wasm` files (The script just uses CosmWasm's [rust-optimizer][9]).

**Note:** Intel/Amd 64-bit processor is required. While there is experimental ARM support for CosmWasm/rust-optimizer, it's discouraged to use in production.

## Deployment

When the deployment scripts run for the first time, it will upload code IDs for each contract, instantiate each contract, initialize assets, and set oracles. If you want to redeploy, you must locally delete the `osmo-test-4.json` file in the artifacts directory.

Everything related to deployment must be ran from the `scripts` directory.

Each outpost has a config file for its respective deployment and assets.

For Osmosis:

```bash
cd scripts

# for testnet deployment with deployerAddr set as owner & admin:
yarn deploy:osmosis-testnet

# for testnet deployment with multisigAddr set as owner & admin:
yarn deploy:osmosis-testnet-multisig

# for mainnet deployment:
yarn deploy:osmosis-mainnet
```

## Schemas

```bash
cargo make --makefile Makefile.toml generate-all-schemas
```

Creates JSON schema files for relevant contract calls, queries and query responses (See: [cosmwams-schema][10]).

## Linting

`rustfmt` is used to format any Rust source code:

```bash
cargo +nightly fmt
```

`clippy` is used as a linting tool:

```bash
cargo make clippy
```

## Testing

Integration tests (task `integration-test` or `test`) use `.wasm` files. They have to be generated with `cargo make build`.

Run unit tests:

```bash
cargo make unit-test
```

Run integration tests:

```bash
cargo make integration-test
```

Run all tests:

```bash
cargo make test
```

## Deployments

TODO

## License

Contents of this repository are open source under [GNU General Public License v3](./LICENSE) or later.

[4]: https://rustup.rs/
[5]: https://github.com/sagiegurari/cargo-make
[6]: https://docs.docker.com/get-docker/
[7]: https://github.com/nvm-sh/nvm
[8]: https://classic.yarnpkg.com/lang/en/docs/install/#mac-stable
[9]: https://github.com/CosmWasm/rust-optimizer
[10]: https://github.com/CosmWasm/cosmwasm/tree/main/packages/schema

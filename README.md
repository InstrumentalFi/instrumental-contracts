# Pablo Vault

[![Artifacts](https://github.com/InstrumentalFi/instrumental-contracts//actions/workflows/artifacts.yml/badge.svg)](https://github.com/InstrumentalFi/instrumental-contracts/actions/workflows/artifacts.yml)
[![Main](https://github.com/InstrumentalFi/instrumental-contracts/actions/workflows/main.yml/badge.svg)](https://github.com/InstrumentalFi/instrumental-contracts/actions/workflows/main.yml)
[![Coverage](https://github.com/InstrumentalFi/instrumental-contracts/actions/workflows/coverage.yml/badge.svg)](https://github.com/InstrumentalFi/instrumental-contracts/actions/workflows/coverage.yml)
[![codecov](https://codecov.io/github/InstrumentalFi/instrumental-contracts/branch/main/graph/badge.svg?token=dH6ikLs46M)](https://codecov.io/github/InstrumentalFi/instrumental-contracts/

This repository contains the source code for the Pablo Vault and Fee
Distribution Contracts.

## Behaviour

This section elaborates on the functionalities and interactions of CosmWasm
contracts used for staking and fee distribution within the Cosmos network.

### 1. Distributor Contract

- **Functionality**: Receives and allocates tokens based on predefined rules.
- **Features**:
  - Supports various Cosmos denominations (denoms), accounts, and
    percentage-based distributions.
  - Enables the transfer of tokens to multiple contracts or addresses, ensuring
    versatile distribution.
  - Suitable for various applications including staking and the Osmosis adapter
    pool (note: no integration with the pool).

### 2. Fee Collector Contract

- **Functionality**: Serves as a recipient for tokens from the Distributor
  contract, acting as a secure storage.
- **Characteristics**:
  - Receives a specific portion of tokens from the Distributor contract.
  - Implements checks and permissions to ensure secure and authorized
    withdrawals of tokens.
  - Allows the Staking contract to withdraw tokens.

### 3. Staking Contract

- **Functionality**: Manages the staking of tokens and facilitates the
  distribution of rewards.
- **Features**:
  - "Tokens per interval" parameter sets a maximum limit for distribution within
    a specified timeframe.
  - Distributes the lesser of total tokens in the collector or "time x tokens
    per interval."
  - Issues stTokens to users in representation of their stake.
  - stTokens are value-accruing, allowing holders to collect a portion of fees
    proportional to their stToken holdings.
- **Distribution**:
  - Occurs at predefined intervals with immediate accrual of user shares upon
    staking.
  - Adopts a pro-rata distribution model, ensuring fee distribution is
    proportional to each user’s stToken holdings.
  - Utilizes a precise user share tracking mechanism akin to other reward
    systems.

### Special Considerations

- **Passive Stakers**: Further investigation is necessary to ensure that passive
  stakers are adequately protected and fairly treated compared to those who
  stake at the last minute. The case of one chain accumulates and sends
  transfers peridocally is important.

## Environment set up

- Install [rustup][4]. Once installed, make sure you have the wasm32 target:

  ```bash
  rustup default stable
  rustup update stable
  rustup target add wasm32-unknown-unknown
  ```

- Install [Docker][6]

- Install [cargo-make][5]

  ```bash
  cargo install --force cargo-make
  ```

- Compile all contracts:

```bash
cargo make rust-optimizer
```

- Download CW20 contract:

```bash
wget -P ./artifacts/ https://github.com/CosmWasm/cw-plus/releases/download/v1.1.0/cw20_base.wasm
```

This compiles and optimizes all contracts, storing them in `/artifacts`
directory along with `checksum.txt` which contains sha256 hashes of each of the
`.wasm` files (The script just uses CosmWasm's [rust-optimizer][9]).

**Note:** Intel/Amd 64-bit processor is required. While there is experimental
ARM support for CosmWasm/rust-optimizer, it's discouraged to use in production.

**NOTE:** on Apple devices an architecture suffix may be required.

## Deployment

See deployment [scripts](./scripts/README.md).

## Schemas

```bash
cargo make --makefile Makefile.toml generate-all-schemas
```

Creates JSON schema files for relevant contract calls, queries and query
responses (See: [cosmwams-schema][10]).

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

Integration tests (task `integration-test` or `test`) use `.wasm` files. They
have to be generated with `cargo make build`.

Run unit tests:

```bash
cargo make unit-test
```

Run all tests:

```bash
cargo make test
```

## Deployments

| chain   | contract    | code id | git commit                               | store tx                                                                                                                                                                         |
| ------- | ----------- | ------- | ---------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| osmosis | liquidator  | 398     | 27aad556a2c6ac6dbb3a4c078fc523982c2b02dc | [9734C67B5E046BA7AE57D566BFB8FCD7611C842A1D4063CA03D81D4636670C12](https://celatone.osmosis.zone/osmosis-1/txs/9734C67B5E046BA7AE57D566BFB8FCD7611C842A1D4063CA03D81D4636670C12) |
| neutron | collector   | 603     | 27aad556a2c6ac6dbb3a4c078fc523982c2b02dc | [D2CDBB27AC03976D239852E01ED43CEB5100574AA4192677E64AC1E4248515A8](https://neutron.celat.one/neutron-1/txs/D2CDBB27AC03976D239852E01ED43CEB5100574AA4192677E64AC1E4248515A8)     |
| neutron | staking     | 604     | 27aad556a2c6ac6dbb3a4c078fc523982c2b02dc | [6DA9CAC7377E8D9EBD567E2372A79D8AAD2354D31871586E6AC2927F4E238B95](https://neutron.celat.one/neutron-1/txs/6DA9CAC7377E8D9EBD567E2372A79D8AAD2354D31871586E6AC2927F4E238B95)     |
| neutron | distributor | 605     | 27aad556a2c6ac6dbb3a4c078fc523982c2b02dc | [FD8A8BA705181B0FFD351B27AEE8DA405D844F972EF67535C5BD7595205D3051](https://neutron.celat.one/neutron-1/txs/FD8A8BA705181B0FFD351B27AEE8DA405D844F972EF67535C5BD7595205D3051)     |


## License

Contents of this repository are open source under
[GNU General Public License v3](./LICENSE) or later.

[4]: https://rustup.rs/
[5]: https://github.com/sagiegurari/cargo-make
[6]: https://docs.docker.com/get-docker/
[7]: https://github.com/nvm-sh/nvm
[8]: https://classic.yarnpkg.com/lang/en/docs/install/#mac-stable
[9]: https://github.com/CosmWasm/rust-optimizer
[10]: https://github.com/CosmWasm/cosmwasm/tree/main/packages/schema

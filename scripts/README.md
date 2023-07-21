# Deployment and Scenario Scripts

This directory contains a number of scripts that enable the deployment of Instrumental contracts to both local and external networks.

## Pre-Requisites

In order to run the scripts locally you must:

* Have followed all the instructions contained in the README of this repository
* Have funded accounts on a CosmWasm compatible network (see test mnemonics)
* Javascript / Node Environment
  * Node v16.14.2
  * npm 6.14.7

## Deploy and Run Locally

1. Build and download relevant contract
2. Define .env file
3. Run deployment scripts

First enter the scripts directory:

```bash
cd scripts
```

Create a `.env` file.

**Note:** this file will contain a mnemonic, for production deployments it is recommended to use an alternative method of key management.

```bash
touch .env
```

Inside define the required variables, e.g.:

```bash
MNEMONIC=donor library crawl fiscal scrub blouse whale hire cannon planet engage bar panther live gym potato weather easily admit comfort bacon flame visit depend
NETWORK=testnet
CHAINID=osmo-test-5
RPC=https://rpc.testnet.com
PREFIX=osmo
```

Next build all the `wasm` files for the relevant contracts.

**Note:** it is recommended to use `workspace-optimizer` docker file for production builds.

```bash
./scripts/build_artifacts.sh
```

Now download a copy of `cw20-base.wasm` into the artifacts directory.

```bash
wget -P ../artifacts https://github.com/CosmWasm/cw-plus/releases/download/v1.1.0/cw20_base.wasm
```

Deploy contracts to cosmwasm testnet:

Then install npm packages and run deployment script:

```bash
npm install
node --loader ts-node/esm deploy.ts
```

All contract addresses should be output to file `contract-address.json`.

## Test Mnemonics

| Account   | Address                                       | Mnemonic                                                                                                                                                                   |
| --------- | --------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Owner     | `osmo164sw03784vjpqtf6ef8zs09zznrf7rnusw8v0e` | `donor library crawl fiscal scrub blouse whale hire cannon planet engage bar panther live gym potato weather easily admit comfort bacon flame visit depend` |
| Staking   | `osmo1qc5pen6am58wxuj58vw97m72vv5tp74remsul7` | `dolphin art village sword mountain vibrant saddle carry kitchen wet burger kangaroo elite online cause fluid harsh ticket board wave fetch noise display pill` |
| Manager   | `osmo1rrmlcs4nr52uy239ljthnkhl9cvgfzvwdsjlch` | `wrap chair grocery stuff cycle fold hammer damage blossom sand neutral color galaxy obvious calm famous sun level mandate luxury episode culture remain benefit` |

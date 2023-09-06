# Liquidator Helper Script

This script is to ease interaction with the Liquidator contract. The script is
meant for acceptance testing and demonstration purposes.

The `liquidator` script is a thin wrapper around `osmosisd` so as you may find
eventually you do not need this script.

## Prerequisites

- [jq][1]
- [osmosisd][2]

[1]: https://jqlang.github.io/jq/
[2]: https://docs.osmosis.zone/osmosis-core/osmosisd/

## osmosisd Configuration

`osmosisd` should be configured to point to your desired chain.

If you have installed `osmosisd` in the default location the file is
`~/.osmosisd/config/client.toml`.

```toml
chain-id = "osmo-test-5"
node = "https://rpc.testnet.osmosis.zone:443"
```

Verify your installation with

```sh
osmosisd query epochs current-epoch day
```

If you see the current epoch you are good to go.

You will need to set up some osmosisd account and request funds from the faucet
before proceeding.

## Installation

The `liquidator` script is a POSIX compliant shell script so should work with
zsh, bash, dash and any POSIX compliant shell.

You can run the script from the directory it is in

```sh
./liquidator
```

You can also place this script somewhere in your $PATH to make it accessible
anywhere on your filesystem.

```sh
liquidator
```

Autocompletion files are available for `bash` and `zsh` in the autocomplete
folder.

## liquidator Configuration

At the top of the `liquidator` script are a number of constants. These are
documented in the script. These allow you to set the owner, admin and deployer
for the contract.

## Store

To store the contract it must first be compiled.

In the `pablo-vault` repo there is a script in the base of the project to
compile optimized binaries for production deployment.

```sh
build_release.sh
```

This generates binaries to `./artifacts` where the liquidator contract is
available.

```sh
ls ./artifacts | grep liquidator
liquidator.wasm
```

Assuming the liquidator script is in your path you may now store the contract.

```sh
liquidator store ./artifacts/liquidator.wasm
gas estimate: 1842942
3836
```

The number returned is the `code_id` which will be used in the next step.

## Instantiate

With code_id of `3836` from the previous step we can now instantiate the
contract.

The arguments are as follows

1. code_id - the code_id of the stored code
2. owner - the address of the owner of the contract
3. ibc_channel_id - the IBC channel id
4. ibc_to_address - the to address on the remote chain
5. liquidation_target - the denom to liquidate to

```sh
liquidator instantiate \
    3835 \ # code_id
    osmo1sut9jhaxsvc3lnps7jzu5pmh4n0kpzfsq0depx \ #owner
    channel-123 \ #ibc_channel_id
    noble1yrg6daqkxyeqye4aac09stzvvwppqwls6kwegl \ #ibc_to_address
    uion #liquidation_target
```

## Execute

### Set Routes

In order to liquidate assets that it holds, the contract needs to have a route
to use when selling one asset for another. Sometimes a pool will exist for a
trading pair. Other times the trade will use multiple hops to get the desired
trade.

A single pool trade selling uosmo for uion.

```sh
liquidator execute route  \
    uosmo \
    uion \
    '[{ "pool_id": "1", "token_out_denom": "uion" }]'
```

We can verify the route was set correctly

```sh
liquidator query route uosmo uion
```

```json
{
  "data": {
    "pool_route": [
      {
        "pool_id": "1",
        "token_out_denom": "uion"
      }
    ]
  }
```

A multi-hop trade selling
`ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4` to
`uion`.

```sh
liquidator execute route  \
    ibc/B3504E092456BA618CC28AC671A71FB08C6CA0FD0BE7C8A5B5A3E2DD933CC9E4 \
    uion \
    '[{ "pool_id": "6", "token_out_denom": "uosmo" }, { "pool_id": "1", "token_out_denom": "uion" }]'
```

<!-- prettier-ignore -->
> [!NOTE]
> The `ACCOUNT` constant in the script must be the owner account for this operation.

### Set Owner

The contract owner is set at instantiation but can be updated by _the existing
owner_.

The arguments are as follows

1. new_owner - The address of the new owner

```sh
liquidator execute owner \
    osmo1sut9jhaxsvc3lnps7jzu5pmh4n0kpzfsq0depx #new_owner
```

<!-- prettier-ignore -->
> [!NOTE]
> The `ACCOUNT` constant in the script must be the owner account for this operation.

### Set Config

Contract configuration may be updated by the contract owner.

The arguments are as follows

1. ibc_channel_id - the IBC channel id
2. ibc_to_address - the to address on the remote chain
3. liquidation_target - the denom to liquidate to

```sh
liquidator execute config \
    channel-123 \ #ibc_channel_id
    noble1yrg6daqkxyeqye4aac09stzvvwppqwls6kwegl  \ #ibc_to_address
    uion \ #liquidation target
```

<!-- prettier-ignore -->
> [!NOTE]
> The `ACCOUNT` constant in the script must be the owner account for this operation.

### Liquidate

Anyone may call this function. This will loop through the routes on the contract
and liquidate any first assets in the pair via the routes it has.

```sh
liquidator execute liquidate
```

### IBC Transfer

Anyone may call this function. This transfers any of the `liquidation_target`
asset held on the contract to the destination address on the remote chain via
IBC.

```sh
liquidator execute ibc_transfer
```

## Query

A number of queries are available to see state on the contract.

### Config

Returns the configuration for the contract

```sh
liquidator query config
```

```json
{
  "data": {
    "ibc_channel_id": "channel-123",
    "ibc_to_address": "noble1yrg6daqkxyeqye4aac09stzvvwppqwls6kwegl",
    "liquidation_target": "uosmo"
  }
}
```

### Owner

Returns the owner of the contract

```sh
liquidator query owner
```

```json
{
  "data": {
    "owner": "osmo1sut9jhaxsvc3lnps7jzu5pmh4n0kpzfsq0depx"
  }
}
```

### Route

Returns the liquidation route for a trading pair

```sh
liquidator query route uosmo uion
```

```json
{
  "data": {
    "pool_route": [
      {
        "pool_id": "1",
        "token_out_denom": "uion"
      }
    ]
  }
}
```

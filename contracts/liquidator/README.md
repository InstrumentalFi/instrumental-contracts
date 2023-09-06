# Liquidator Contract

The liquidator contract receives protocol fees, liquidates fees and forwards
proceeds via IBC.

## General flow

1. Contract receives protocol fees from another account.
2. These protocol fees are liquidated to a specfic token.
3. The token is sent via IBC to another chain.

The owner can set parameters at Instatiation.

- `ibc_channel_id` - The channel id to use for the IBC transfer.
- `ibc_to_address` - The account address on the receiving chain.
- `liquidation_target` - The denomination of the liquidation target.

In order for contract to know which routes to use to liqiuidate assets routes
must be set for pairs. These can be set by `set_route`.

When the liquidate function is call the contract will loop through routes and
liquidate any balances on the contract that it has routes for.

## InstantiateMsg

The instantiation message defines ibc information and the liquidation target

```json
{
  "ibc_channel_id": "osmo...",
  "ibc_channel_id": "channel-169",
  "ibc_to_address": "juno1yrg6daqkxyeqye4aac09stzvvwppqwls6kwegl",
  "liquidation_target": "56D7C03B8F6A07AD322EEE1BEF3AE996E09D1C1E34C27CF37E0D4A0AC5972516"
}
```

## ExecuteMsg

### `update_owner`

Updates the contract owner.

```json
{
  "update_owner": {
    "owner": "osmo..."
  }
}
```

### `update_config`

Enables the owner to update and edit the distribution address and proportions.

```json
{
  "update_config": {
    "ibc_channel_id": "channel-169",
    "ibc_to_address": "juno1yrg6daqkxyeqye4aac09stzvvwppqwls6kwegl",
    "liquidation_target": "56D7C03B8F6A07AD322EEE1BEF3AE996E09D1C1E34C27CF37E0D4A0AC5972516"
  }
}
```

### `set_route`

Specifies a route to liquidate an asset via the gamm.

```json
{
  "set_route": {
    "input_denom": "uosmo",
    "output_denom": "uion",
    "pool_route": [{ "pool_id": 1, "token_out_denom": "uion" }]
  }
}
```

### `remove_route`

Removes a route for trading pair

```json
{
  "set_route": {
    "input_denom": "uosmo",
    "output_denom": "uion"
  }
}
```

### `liquidate`

Permissionless method that liquidates assets.

```json
{
  "liquidate": {}
}
```

### `ibc_transfer`

Permissionless method that transfers liquidation target tokens via IBC to the
`ibc_channel_id` and `ibc_to_address`.

```json
{
  "ibc_transfer": {}
}
```

## QueryMsg

### `get_owner`

Returns contract owner.

```json
{
  "get_owner": {}
}
```

### `get_config`

Returns contract parameters.

```json
{
  "get_config": {}
}
```

### `get_route`

Returns contract parameters.

```json
{
  "get_route": {
    "input_demon": "uosmo",
    "output_denom": "uion"
  }
}
```

### `get_all_routes`

Returns contract parameters.

```json
{
  "get_all_routes": {}
}
```

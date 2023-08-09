# Staking

The fee Staking is where token holders stake their tokens in order to earn rewards generated. Rewards are distributed using an emission of tokens per interval. When users stake they are issued with a receipt token that they can use then to unstake their tokens.

## General flow

Contract has defined a deposit and reward token.

Users are able to stake the deposit token, receiving a receipt token (CW20) in return, and then claim rewards emitted over time.

The contract owner is able to define and update the rewards emitted per token interval.

---

## InstantiateMsg

The instantiation message contains the address of the fee collector contract and the relevant deposit and reward denoms. Further it defines the emission rate and the code id and name of the CW20 that is minted as receipt token.

```json
{
    "fee_collector": "centauri...",
    "deposit_denom": "udeposit",
    "reward_denom": "ureward",
    "deposit_decimals": 6,
    "reward_decimals": 6,
    "tokens_per_interval": 100000,
    "token_code_id": 1,
    "token_name": "stakedDeposit",
}
```

## ExecuteMsg

### `update_config`

Enables the owner to alter the reward token emission.

```json
{
   "update_config": {
        "tokens_per_interval": 150000
   } 
}
```

### `update_rewards`

Updates the internal accounting of rewards for users.

```json
{
   "update_rewards": {} 
}
```

### `stake`

Enables a user to stake deposit tokens, requires tokens to be sent in the transaction payload.

```json
{
   "stake": {} 
}
```

### `claim`

Allows a user to claim the rewards that have accrued to their staked position. User can optionally define a recipient address for the rewards

```json
{
   "claim": {
        "recipient": "Some(centauri...)",
   } 
}
```

### `pause`

Contract owner may pause staking.

```json
{
   "pause": {} 
}
```

### `unpause`

Contract owner may unpause staking.

```json
{
   "unpause": {} 
}
```

## QueryMsg

### `config`

Returns contract parameters.

```json
{
    "config": {}
}
```

### `state`

Returns contract state parameters, such as total amounts staked.

```json
{
    "state": {}
}
```

### `get_claimable`

Returns the amounts claimable for a specific user.

```json
{
    "get_claimable": {
        "user": "centauri..."
    }
}
```

### `get_user_staked_amount`

Returns the amount staked by a specific user.

```json
{
    "get_user_staked_amount": {
        "user": "centauri..."
    }
}
```

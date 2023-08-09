# Distributor Contract

The fee distributor receives fee tokens and then distributes them to the defined distribution address on a pro-rata basis.

## General flow

Contract receives tokens of a specific denomination.

Owner must define this upon contract instantiation. This token can then be distributed to a number of addresses proportionally also held by the contract.

The owner can define and update these addresses.

---

## InstantiateMsg

The instantiation message defines the token that is to be distributed and the distributors with their pro-rata proportion of tokens.

```json
{
    "token": "native_denom",
    "distribution": [
        ("centauri...", "500_000_000"),
        ("centauri...", "500_000_000"),
    ]
}
```

## ExecuteMsg

### `update_owner`

Updates the contract owner.

```json
{
   "update_owner": {
        "owner": "centauri..."
   } 
}
```

### `update_config`

Enables the owner to update and edit the distribution address and proportions.

```json
{
   "update_config": {
        "distribution": [
            ("centauri...", "500_000_000"),
            ("centauri...", "500_000_000"),
        ]
   } 
}
```

### `distribute`

Permissionless method that distributes held funds on a pro-rata basis to the defined distribution addresses.

```json
{
   "distribute": {} 
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

### `get_token`

Returns contract token.

```json
{
    "get_token": {}
}
```

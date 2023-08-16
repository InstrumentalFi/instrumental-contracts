use cosmwasm_std::{Addr, Deps, StdResult, Timestamp, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");

pub const STAKERS: Item<u64> = Item::new("stakers_counter");
pub const REWARDS_PER_TOKEN: Item<Uint128> = Item::new("rewards_per_token");
pub const USER_STAKE: Map<Addr, UserStake> = Map::new("staked_amounts");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub fee_collector: Addr,
    pub deposit_denom: String,
    pub deposit_decimals: u32,
    pub staked_denom: String,
    pub reward_denom: String,
    pub reward_decimals: u32,
    pub tokens_per_interval: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub is_open: bool,
    pub last_distribution: Timestamp,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct UserStake {
    pub staked_amounts: Uint128,
    pub claimable_rewards: Uint128,
    pub previous_cumulative_rewards_per_token: Uint128,
    pub cumulative_rewards: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Pool {
    pub id: u64,
    pub quote_denom: String,
}

/// Queries contract configuration
pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config = CONFIG.load(deps.storage).unwrap();

    Ok(config)
}

/// Queries contract state
pub fn query_state(deps: Deps) -> StdResult<State> {
    let state = STATE.load(deps.storage).unwrap();

    Ok(state)
}

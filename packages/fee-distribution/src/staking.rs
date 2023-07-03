use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub fee_collector: String,
    pub deposit_denom: String,
    pub reward_denom: String,
    pub deposit_decimals: u32,
    pub reward_decimals: u32,
    pub tokens_per_interval: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        tokens_per_interval: Option<Uint128>,
    },
    UpdateRewards {},
    Stake {},
    Unstake {},
    Claim {
        recipient: Option<String>,
    },
    Pause {},
    Unpause {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {},
    GetClaimable { user: String },
    GetUserStakedAmount { user: String },
}

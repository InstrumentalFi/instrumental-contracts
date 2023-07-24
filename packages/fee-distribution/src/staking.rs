use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw20::{Cw20Coin, Cw20ReceiveMsg, Logo, MinterResponse};
// use cw20_base::InstantiateMsg as Cw20InstantiateMsg;
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
    pub token_code_id: u64,
    pub token_name: String,
}

#[cw_serde]
pub struct InstantiateMarketingInfo {
    pub project: Option<String>,
    pub description: Option<String>,
    pub marketing: Option<String>,
    pub logo: Option<Logo>,
}

#[cw_serde]
#[cfg_attr(test, derive(Default))]
pub struct Cw20TokenInstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<MinterResponse>,
    pub marketing: Option<InstantiateMarketingInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateConfig {
        tokens_per_interval: Option<Uint128>,
    },
    UpdateRewards {},
    Stake {},
    Receive(Cw20ReceiveMsg),
    Claim {
        recipient: Option<String>,
    },
    Pause {},
    Unpause {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    Unstake {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {},
    GetClaimable {
        user: String,
    },
    GetUserStakedAmount {
        user: String,
    },
}

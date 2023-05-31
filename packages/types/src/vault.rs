use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, BalanceResponse, Timestamp};

#[cw_serde]
pub struct Config {
    pub token_a: Addr,
    pub token_b: Addr,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub token_a: Addr,
    pub token_b: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Deposit two tokens into the vault
    Deposit {},

    // Withdraw and redeem underlying from vault
    Withdraw {},

    // Harvest rewards from Pablo, receiving PICA
    Harvest {},

    // Sell PICA for underlying and put proceeds back into lp_pool
    Compound {},

    // Distribute rewards to veToken holders
    DistributeRewards {},

    // Sets the Harvest Wait Period,
    // Harvest can only be called if this period has expired
    SetHarvestWaitPeriod {},

    // Sets the Compound Wait Period,
    // Compound can only be called if this period has expired
    SetCompoundWaitPeriod {},
}

#[derive(QueryResponses)]
#[cw_serde(rename_all = "snake_case")]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},

    #[returns(LastHarvestResponse)]
    LastHarvest {},

    #[returns(LastCompoundResponse)]
    LastCompound {},

    #[returns(HarvestWaitPeriodResponse)]
    HarvestWaitPeriod {},

    #[returns(CompoundWaitPeriodResponse)]
    CompoundWaitPeriod {},

    #[returns(TokensBalancesResponse)]
    TokenBalances {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub token_a: Addr,
    pub token_b: Addr,
}

#[cw_serde]
pub struct LastHarvestResponse {
    pub timestamp: Timestamp,
}

#[cw_serde]
pub struct LastCompoundResponse {
    pub timestamp: Timestamp,
}

#[cw_serde]
pub struct HarvestWaitPeriodResponse {
    pub timestamp: Timestamp,
}

#[cw_serde]
pub struct CompoundWaitPeriodResponse {
    pub timestamp: Timestamp,
}

#[cw_serde]
pub struct TokensBalancesResponse {
    pub token_a: BalanceResponse,
    pub token_b: BalanceResponse,
}

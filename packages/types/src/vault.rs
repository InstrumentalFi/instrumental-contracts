use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, BalanceResponse, StdError, StdResult, Timestamp};

#[cw_serde]
pub struct Config {
    pub token_a: Addr,
    pub token_b: Addr,
    pub owner: Addr,
    pub harvest_wait_period: u64,  // Harvest wait period in seconds
    pub compound_wait_period: u64, // Compound wait period in seconds
}

#[cw_serde]
pub struct State {
    pub last_harvest: Timestamp,
    pub last_compound: Timestamp,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub token_a: Addr,
    pub token_b: Addr,
}

impl InstantiateMsg {
    pub fn validate(&self) -> StdResult<()> {
        // Check token_a and token_b are different
        if !self.has_valid_tokens() {
            return Err(StdError::generic_err("token_a and token_b cannot be the same"));
        }
        Ok(())
    }

    fn has_valid_tokens(&self) -> bool {
        if self.token_a == self.token_b {
            return false;
        }
        true
    }
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

    UpdateConfig {
        compound_wait_period: Option<String>,
        harvest_wait_period: Option<String>,
    },
}

#[derive(QueryResponses)]
#[cw_serde(rename_all = "snake_case")]
pub enum QueryMsg {
    #[returns(Config)]
    Config {},

    #[returns(State)]
    State {},

    #[returns(TokensBalancesResponse)]
    TokenBalances {},
}

#[cw_serde]
pub struct TokensBalancesResponse {
    pub token_a: BalanceResponse,
    pub token_b: BalanceResponse,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_instantiatemsg_tokens() {
        // Tokens are the same - invalid
        let mut msg = InstantiateMsg {
            token_a: Addr::unchecked("tokena"),
            token_b: Addr::unchecked("tokena"),
        };
        assert!(!msg.has_valid_tokens());

        // Tokens are not the same valid
        msg.token_b = Addr::unchecked("tokenb");
        assert!(msg.has_valid_tokens());
    }
}

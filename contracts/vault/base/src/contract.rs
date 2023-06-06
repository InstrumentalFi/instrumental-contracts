use crate::error::ContractResult;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw_storage_plus::Item;
use pablo_vault_types::vault::{Config, ExecuteMsg, InstantiateMsg, QueryMsg, State};

pub struct VaultBase<'a> {
    pub config: Item<'a, Config>,
}

impl<'a> Default for VaultBase<'a> {
    fn default() -> Self {
        Self {
            config: Item::new("config"),
        }
    }
}

impl<'a> VaultBase<'a> {
    pub fn instantiate(
        &self,
        deps: DepsMut,
        _info: MessageInfo,
        msg: InstantiateMsg,
    ) -> ContractResult<Response> {
        self.config.save(
            deps.storage,
            &Config {
                base_token: msg.base_token,
            },
        )?;
        Ok(Response::new().add_attribute("action", "instantiate"))
    }

    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> ContractResult<Response> {
        match msg {
            ExecuteMsg::Deposit {} => self.execute_deposit(deps, env, info, msg),
            ExecuteMsg::Withdraw {} => self.execute_withdraw(deps, env, info, msg),
            ExecuteMsg::Harvest {} => self.execute_harvest(deps, env, info, msg),
            ExecuteMsg::Compound {} => self.execute_compound(deps, env, info, msg),
            ExecuteMsg::DistributeRewards {} => {
                self.execute_distribute_rewards(deps, env, info, msg)
            }
            ExecuteMsg::UpdateConfig {} => self.execute_update_config(deps, env, info, msg),
        }
    }

    pub fn query(&self, deps: Deps, _env: Env, msg: QueryMsg) -> ContractResult<Binary> {
        let res = match msg {
            QueryMsg::Config {} => to_binary(&self.query_config(deps)?),
            QueryMsg::State {} => to_binary(&self.query_state(deps)?),
        };
        res.map_err(Into::into)
    }

    /// Deposits an equal amount of two tokens into the vault, returning a new token representing
    /// ownership of a deposit.
    fn execute_deposit(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: ExecuteMsg,
    ) -> ContractResult<Response> {
        unimplemented!();
    }

    /// Withdraws a position from the vault by sending a token representing ownership of a deposit
    /// ownership over a deposit. This burns the ownership token and returns the underlying tokens to
    /// the caller
    fn execute_withdraw(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: ExecuteMsg,
    ) -> ContractResult<Response> {
        unimplemented!();
    }

    /// Harvests rewards from the rewards contract and holds rewards on the vault contract
    fn execute_harvest(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: ExecuteMsg,
    ) -> ContractResult<Response> {
        unimplemented!();
    }

    /// Compounds rewards by
    /// * Selling the rewards token
    /// * Buying equal amounts of the underlying for the LP  (e.g. DOT/sDOT)
    /// * Investing underlying in the LP
    /// * Staking the LP Token in the rewards contract
    fn execute_compound(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: ExecuteMsg,
    ) -> ContractResult<Response> {
        unimplemented!();
    }

    /// Distribute rewards to the rewards contract
    fn execute_distribute_rewards(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: ExecuteMsg,
    ) -> ContractResult<Response> {
        unimplemented!();
    }

    /// Sets the harvest wait period. If the `execute_harvest` function is called
    /// before the wait period has expired an error will be returned
    fn execute_update_config(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: ExecuteMsg,
    ) -> ContractResult<Response> {
        unimplemented!();
    }

    /// Returns the configuration set during contract instnatiation
    fn query_config(&self, deps: Deps) -> StdResult<Config> {
        let config = self.config.load(deps.storage)?;
        Ok(config)
    }

    /// Returns the configuration set during contract instnatiation
    fn query_state(&self, _deps: Deps) -> StdResult<State> {
        unimplemented!();
    }
}

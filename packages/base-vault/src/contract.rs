use apollo_cw_asset::AssetInfo;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw_storage_plus::Item;
use cw_vault_token::{CwTokenError, VaultToken};
use pablo_vault_types::vault::{Config, ExecuteMsg, QueryMsg, State};
use serde::{de::DeserializeOwned, Serialize};

use crate::error::ContractResult;

pub const DEFAULT_VAULT_TOKENS_PER_STAKED_BASE_TOKEN: Uint128 = Uint128::new(1_000_000);

pub struct BaseVault<'a, V> {
    pub vault_token: Item<'a, V>,
    pub base_token: Item<'a, AssetInfo>,
    pub total_staked_base_tokens: Item<'a, Uint128>,
}

impl<V> Default for BaseVault<'_, V> {
    fn default() -> Self {
        Self {
            vault_token: Item::new("vault_token"),
            base_token: Item::new("base_token"),
            total_staked_base_tokens: Item::new("total_staked_base_tokens"),
        }
    }
}

impl<'a, V> BaseVault<'a, V>
where
    V: Serialize + DeserializeOwned + VaultToken,
{
    pub fn init(
        &self,
        deps: DepsMut,
        base_token: AssetInfo,
        vault_token: V,
        init_info: Option<Binary>,
    ) -> Result<Response, CwTokenError> {
        self.vault_token.save(deps.storage, &vault_token)?;
        self.base_token.save(deps.storage, &base_token)?;
        self.total_staked_base_tokens.save(deps.storage, &Uint128::zero())?;

        vault_token.instantiate(deps, init_info)
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

    /// Deposits an LP token to the vault and issues a share token
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
    fn query_config(&self, _deps: Deps) -> StdResult<Config> {
        unimplemented!();
    }

    /// Returns the configuration set during contract instnatiation
    fn query_state(&self, _deps: Deps) -> StdResult<State> {
        unimplemented!();
    }
}

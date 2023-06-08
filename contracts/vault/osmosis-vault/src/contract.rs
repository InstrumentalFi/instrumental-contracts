//use cosmwasm_std::Empty;
use base_vault::BaseVault;
// use base_vault::ContractResult;
use cosmwasm_std::{entry_point, DepsMut, Env, MessageInfo, Response, StdError};
use cw_dex::{osmosis::OsmosisPool, traits::Pool};
use cw_vault_token::{osmosis::OsmosisDenom, CwTokenError};
// use pablo_vault_types::vault::{ExecuteMsg, QueryMsg};
use thiserror::Error;

use crate::msg::InstantiateMsg;

pub type OsmosisVault<'a, V> = BaseVault<'a, V>;

pub const CONTRACT_NAME: &str = "crates.io:osmosis-vault";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Generic(String),
}

impl From<String> for ContractError {
    fn from(val: String) -> Self {
        ContractError::Generic(val)
    }
}

impl From<&str> for ContractError {
    fn from(val: &str) -> Self {
        ContractError::Generic(val.into())
    }
}

// use crate::migrations;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, CwTokenError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let contract = OsmosisVault::default();

    // TODO validation logic for 100 osmo being sent to contract
    let pool = OsmosisPool::new(msg.pool_id, deps.as_ref())?;
    let vault_token = OsmosisDenom::new(env.contract.address.to_string(), msg.vault_token_subdenom);
    contract.init(deps, pool.lp_token(), vault_token, None)
}

// #[entry_point]
// pub fn execute(
//     deps: DepsMut,
//     env: Env,
//     info: MessageInfo,
//     msg: ExecuteMsg,
// ) -> ContractResult<Response> {
//     OsmosisVault::default().execute(deps, env, info, msg)
// }

// #[entry_point]
// pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<Binary> {
//     OsmosisVault::default().query(deps, env, msg)
// }

// #[entry_point]
// pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> ContractResult<Response> {
//     migrations::v1_0_0::migrate(deps)
// }

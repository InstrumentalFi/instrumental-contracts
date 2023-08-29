use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use cw_controllers::Admin;

use crate::{
    error::ContractError,
    handle::{ibc_transfer, liquidate, update_config, update_owner},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query::{query_config, query_owner},
    state::{Config, CONFIG},
};

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Owner admin
pub const OWNER: Admin = Admin::new("owner");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, format!("crates.io:{CONTRACT_NAME}"), CONTRACT_VERSION)?;

    CONFIG.save(
        deps.storage,
        &Config {
            ibc_channel_id: msg.ibc_channel_id,
            ibc_to_address: msg.ibc_to_address,
            liquidation_target: msg.liquidation_target,
        },
    )?;

    OWNER.set(deps, Some(info.sender))?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::UpdateOwner {
            owner,
        } => update_owner(deps, info, owner),
        ExecuteMsg::UpdateConfig {
            ibc_to_address,
            ibc_channel_id,
            liquidation_target,
        } => update_config(deps, info, ibc_to_address, ibc_channel_id, liquidation_target),
        ExecuteMsg::Liquidate {} => liquidate(deps.as_ref(), env, info),
        ExecuteMsg::IbcTransfer {} => ibc_transfer(deps.as_ref(), env, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
    }
}

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::{
    error::ContractError,
    handle::{ibc_transfer, liquidate, set_route, update_config, update_owner},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query::{query_config, query_owner, query_route},
    state::{Config, CONFIG, OWNER},
};

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

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

    let owner_address = deps.api.addr_validate(&msg.owner)?;

    OWNER.save(deps.storage, &owner_address)?;

    Ok(Response::new().add_attribute("method", "instantiate").add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateOwner {
            owner,
        } => update_owner(deps, info, owner),
        ExecuteMsg::UpdateConfig {
            ibc_to_address,
            ibc_channel_id,
            liquidation_target,
        } => update_config(deps, info, ibc_to_address, ibc_channel_id, liquidation_target),
        ExecuteMsg::SetRoute {
            input_denom,
            output_denom,
            pool_route,
        } => set_route(deps, info, input_denom, output_denom, pool_route),
        ExecuteMsg::Liquidate {} => liquidate(deps.as_ref(), env, info),
        ExecuteMsg::IbcTransfer {} => ibc_transfer(deps.as_ref(), env, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::GetRoute {
            input_denom,
            output_denom,
        } => to_binary(&query_route(deps, input_denom, output_denom)?),
    }
}
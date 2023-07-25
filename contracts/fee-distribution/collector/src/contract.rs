use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use cw_controllers::Admin;
use fee_distribution::collector::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::{
    error::ContractError,
    handle::{add_token, remove_token, send_token, update_owner, update_whitelist},
    query::{
        query_all_token, query_is_token, query_owner, query_token_list_length, query_whitelist,
    },
    state::WHITELIST_ADDRESS,
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
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, format!("crates.io:{CONTRACT_NAME}"), CONTRACT_VERSION)?;

    WHITELIST_ADDRESS.save(deps.storage, &info.sender)?;

    OWNER.set(deps, Some(info.sender))?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::UpdateOwner {
            owner,
        } => update_owner(deps, info, owner),
        ExecuteMsg::AddToken {
            token,
        } => add_token(deps, info, token),
        ExecuteMsg::RemoveToken {
            token,
        } => remove_token(deps, info, token),
        ExecuteMsg::UpdateWhitelist {
            address,
        } => update_whitelist(deps, info, address),
        ExecuteMsg::SendToken {
            token,
            amount,
            recipient,
        } => send_token(deps.as_ref(), env, info, token, amount, recipient),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
        QueryMsg::GetWhitelist {} => to_binary(&query_whitelist(deps)?),
        QueryMsg::IsToken {
            token,
        } => to_binary(&query_is_token(deps, token)?),
        QueryMsg::GetTokenList {
            limit,
        } => to_binary(&query_all_token(deps, limit)?),
        QueryMsg::GetTokenLength {} => to_binary(&query_token_list_length(deps)?),
    }
}

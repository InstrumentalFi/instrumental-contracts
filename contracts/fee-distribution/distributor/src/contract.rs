use cosmwasm_std::{
    entry_point, to_json_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128,
};
use cw2::set_contract_version;
use cw_controllers::Admin;
use fee_distribution::distributor::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::{
    error::ContractError,
    handle::{distribute, update_config, update_owner},
    helpers::validate_distribution,
    query::{query_config, query_owner, query_token},
    state::{Config, CONFIG, TOKEN},
};

pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const DECIMALS: u128 = 1_000_000u128;
pub const DECIMAL_PLACES: u8 = 6u8;

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

    let mut distribution: Vec<(Addr, Uint128)> = vec![];

    for (recipient, share) in msg.distribution.iter() {
        distribution.push((deps.api.addr_validate(recipient)?, *share));
    }
    validate_distribution(distribution.clone())?;

    CONFIG.save(
        deps.storage,
        &Config {
            distribution,
        },
    )?;

    TOKEN.save(deps.storage, &msg.token)?;

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
            distribution,
        } => update_config(deps, info, distribution),
        ExecuteMsg::Distribute {} => distribute(deps.as_ref(), env, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_json_binary(&query_owner(deps)?),
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
        QueryMsg::GetToken {} => to_json_binary(&query_token(deps)?),
    }
}

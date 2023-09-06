use std::collections::HashMap;

use cosmwasm_std::{Deps, Order, StdError, StdResult};
use cw_storage_plus::Bound;

const MAX_LIMIT: u32 = 30;
const DEFAULT_LIMIT: u32 = 10;

use crate::{
    error::ContractError,
    msg::{GetAllRoutesResponse, GetOwnerResponse, GetRouteResponse},
    state::{Config, CONFIG, OWNER, ROUTING_TABLE},
};

/// Queries contract owner from the admin
pub fn query_owner(deps: Deps) -> Result<GetOwnerResponse, ContractError> {
    if let Some(owner) = OWNER.get(deps)? {
        Ok(GetOwnerResponse {
            owner: owner.to_string(),
        })
    } else {
        Err(ContractError::NoOwner {})
    }
}

/// Queries config
pub fn query_config(deps: Deps) -> StdResult<Config> {
    match CONFIG.may_load(deps.storage) {
        Ok(Some(config)) => Ok(config),
        Ok(None) => Err(StdError::generic_err("No config set")),
        Err(_) => Err(StdError::generic_err("No config set")),
    }
}

pub fn query_route(
    deps: Deps,
    input_denom: String,
    output_denom: String,
) -> StdResult<GetRouteResponse> {
    let route = ROUTING_TABLE
        .load(deps.storage, (&input_denom, &output_denom))
        .map_err(|_| StdError::not_found("Route"))?;
    Ok(GetRouteResponse {
        pool_route: route,
    })
}

pub fn query_all_routes(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<GetAllRoutesResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.map(|s| Bound::ExclusiveRaw(s.into()));

    let routes: StdResult<HashMap<_, _>> = ROUTING_TABLE
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| item.map(|((k1, k2), v)| (format!("{}:{}", k1, k2), v)))
        .collect();

    Ok(GetAllRoutesResponse {
        routes: routes?,
    })
}

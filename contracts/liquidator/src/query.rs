use cosmwasm_std::{Deps, StdError, StdResult};

use crate::{
    msg::{GetOwnerResponse, GetRouteResponse},
    state::{Config, CONFIG, OWNER, ROUTING_TABLE},
};

/// Queries contract owner from the admin
pub fn query_owner(deps: Deps) -> StdResult<GetOwnerResponse> {
    let owner = OWNER.load(deps.storage)?;
    Ok(GetOwnerResponse {
        owner: owner.into_string(),
    })
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
    let route = ROUTING_TABLE.load(deps.storage, (&input_denom, &output_denom))?;
    Ok(GetRouteResponse {
        pool_route: route,
    })
}

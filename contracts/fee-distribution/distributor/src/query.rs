use cosmwasm_std::{Deps, StdError, StdResult};
use fee_distribution::distributor::OwnerResponse;

use crate::{
    contract::OWNER,
    state::{Config, CONFIG, TOKEN},
};

/// Queries contract owner from the admin
pub fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    if let Some(owner) = OWNER.get(deps)? {
        Ok(OwnerResponse {
            owner,
        })
    } else {
        Err(StdError::generic_err("No owner set"))
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
/// Queries if the token with given address is already stored
pub fn query_token(deps: Deps) -> StdResult<String> {
    TOKEN.load(deps.storage)
}

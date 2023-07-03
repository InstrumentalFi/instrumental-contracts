use cosmwasm_std::{Deps, StdError, StdResult};
use fee_distribution::collector::{
    AllTokenResponse, ConfigResponse, OwnerResponse, TokenLengthResponse, TokenResponse,
};

use crate::{
    contract::OWNER,
    state::{is_token, read_token_list, TOKEN_LIMIT},
};

const DEFAULT_PAGINATION_LIMIT: u32 = 10u32;
const MAX_PAGINATION_LIMIT: u32 = TOKEN_LIMIT as u32;

/// Queries contract config
pub fn query_config(_: Deps) -> StdResult<ConfigResponse> {
    Ok(ConfigResponse {})
}

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

/// Queries if the token with given address is already stored
pub fn query_is_token(deps: Deps, token: String) -> StdResult<TokenResponse> {
    // read the current storage and pull the vamm status corresponding to the given addr
    let token_bool = is_token(deps.storage, token);

    Ok(TokenResponse {
        is_token: token_bool,
    })
}

/// Queries the list of tokens currently stored
pub fn query_all_token(deps: Deps, limit: Option<u32>) -> StdResult<AllTokenResponse> {
    // set the limit for pagination
    let limit = limit.unwrap_or(DEFAULT_PAGINATION_LIMIT).min(MAX_PAGINATION_LIMIT) as usize;

    let list = read_token_list(deps, limit)?;
    Ok(AllTokenResponse {
        token_list: list,
    })
}

/// Queries the length of the list of tokens currently stored
pub fn query_token_list_length(deps: Deps) -> StdResult<TokenLengthResponse> {
    // set the limit for pagination
    let limit = TOKEN_LIMIT;

    let list_length = read_token_list(deps, limit)?.len();
    Ok(TokenLengthResponse {
        length: list_length,
    })
}

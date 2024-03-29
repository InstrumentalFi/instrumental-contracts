use cosmwasm_std::{Addr, StdError, StdResult, Uint128};

use crate::{contract::DECIMALS, state::RECIPIENT_LIMIT};

pub fn validate_distribution(distribution: Vec<(Addr, Uint128)>) -> StdResult<()> {
    // validate the distribution
    let mut total_weight = Uint128::zero();
    for (_, weight) in distribution.iter() {
        if weight.is_zero() {
            return Err(StdError::generic_err("distribution weight cannot be zero"));
        }
        total_weight += *weight;
    }

    if distribution.len() > RECIPIENT_LIMIT || distribution.is_empty() {
        return Err(StdError::generic_err(format!(
            "Invalid number of recipients: {}",
            distribution.len()
        )));
    }

    if total_weight != Uint128::new(DECIMALS) {
        return Err(StdError::generic_err("total weight must equal to 1_000_000"));
    }

    Ok(())
}

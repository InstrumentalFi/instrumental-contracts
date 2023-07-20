use crate::{
    contract::{DECIMALS, OWNER},
    helpers::validate_distribution,
    state::{Config, CONFIG, TOKEN},
};

use cosmwasm_std::{
    Addr, BalanceResponse, BankMsg, BankQuery, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Response, StdError, StdResult, Uint128,
};

pub fn update_owner(deps: DepsMut, info: MessageInfo, owner: String) -> StdResult<Response> {
    // validate the address
    let valid_owner = deps.api.addr_validate(&owner)?;

    OWNER
        .execute_update_admin(deps, info, Some(valid_owner))
        .map_err(|error| StdError::generic_err(format!("{}", error)))
}

pub fn update_config(
    deps: DepsMut,
    _info: MessageInfo,
    distribution: Vec<(String, Uint128)>,
) -> StdResult<Response> {
    let mut updated_distribution: Vec<(Addr, Uint128)> = vec![];

    for (recipient, share) in distribution.iter() {
        updated_distribution.push((deps.api.addr_validate(recipient)?, *share));
    }
    validate_distribution(updated_distribution.clone())?;

    CONFIG.save(
        deps.storage,
        &Config {
            distribution: updated_distribution,
        },
    )?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn distribute(deps: Deps, env: Env, _info: MessageInfo) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    let token = TOKEN.load(deps.storage)?;

    let mut response = Response::new();
    let res: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: env.contract.address.to_string(),
        denom: token.clone(),
    }))?;

    let total_amount = res.amount.amount;

    for (recipient, share) in config.distribution.iter() {
        let amount =
            total_amount.checked_mul(*share).unwrap().checked_div(DECIMALS.into()).unwrap();

        if !amount.is_zero() {
            let msg = BankMsg::Send {
                to_address: recipient.to_string(),
                amount: vec![Coin {
                    denom: token.clone(),
                    amount,
                }],
            };

            response = response.add_message(CosmosMsg::Bank(msg));
        }
    }

    Ok(response.add_attribute("action", "distribute"))
}

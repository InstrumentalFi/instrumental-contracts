use cosmwasm_std::{
    BalanceResponse, BankMsg, BankQuery, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    QueryRequest, Response, StdError, StdResult, Uint128,
};

use crate::{
    contract::OWNER,
    state::{is_token, remove_token as remove_token_from_list, save_token, WHITELIST_ADDRESS},
};

pub fn update_owner(deps: DepsMut, info: MessageInfo, owner: String) -> StdResult<Response> {
    // validate the address
    let valid_owner = deps.api.addr_validate(&owner)?;

    OWNER
        .execute_update_admin(deps, info, Some(valid_owner))
        .map_err(|error| StdError::generic_err(format!("{}", error)))
}

pub fn add_token(deps: DepsMut, info: MessageInfo, token: String) -> StdResult<Response> {
    // check permission
    if !OWNER.is_admin(deps.as_ref(), &info.sender)? {
        return Err(StdError::generic_err("unauthorized"));
    }

    // add the token
    save_token(deps, token.clone())?;

    Ok(Response::default().add_attributes([("action", "add_token"), ("denom", token.as_str())]))
}

pub fn remove_token(deps: DepsMut, info: MessageInfo, token: String) -> StdResult<Response> {
    // check permission
    if !OWNER.is_admin(deps.as_ref(), &info.sender)? {
        return Err(StdError::generic_err("unauthorized"));
    }

    // remove token here
    remove_token_from_list(deps, token.clone())?;

    Ok(Response::default().add_attributes([("action", "remove_token"), ("denom", token.as_str())]))
}

pub fn update_whitelist(deps: DepsMut, info: MessageInfo, address: String) -> StdResult<Response> {
    // check permission
    if !OWNER.is_admin(deps.as_ref(), &info.sender)? {
        return Err(StdError::generic_err("unauthorized"));
    }

    let address = deps.api.addr_validate(&address)?;

    // add the address to whitelist
    WHITELIST_ADDRESS.save(deps.storage, &address)?;

    Ok(Response::default()
        .add_attributes([("action", "update_whitelist"), ("address", address.as_str())]))
}
pub fn send_token(
    deps: Deps,
    env: Env,
    info: MessageInfo,
    token: String,
    amount: Uint128,
    recipient: String,
) -> StdResult<Response> {
    // check amount is not zero
    if amount.is_zero() {
        return Err(StdError::generic_err("Cannot transfer zero tokens"));
    }

    let whitelist = WHITELIST_ADDRESS.load(deps.storage)?;

    // check permissions to send the message
    if !OWNER.is_admin(deps, &info.sender)? && whitelist != info.sender {
        return Err(StdError::generic_err("unauthorized"));
    }

    // validate the recipient address
    let valid_recipient = deps.api.addr_validate(&recipient)?;

    // check that the token is in the token list
    if !is_token(deps.storage, token.clone()) {
        return Err(StdError::generic_err("This token is not supported"));
    };

    // query the balance of the given token that this contract holds
    let res: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: env.contract.address.to_string(),
        denom: token.clone(),
    }))?;
    let balance = res.amount.amount;

    // check that the balance is sufficient to pay the amount
    if balance < amount {
        return Err(StdError::generic_err("Insufficient funds"));
    }

    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: valid_recipient.to_string(),
        amount: vec![Coin {
            denom: token,
            amount,
        }],
    });

    Ok(Response::default().add_message(msg))
}

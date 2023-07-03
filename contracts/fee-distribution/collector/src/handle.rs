use cosmwasm_std::{
    BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
    Uint128,
};
use osmosis_std::types::cosmos::bank::v1beta1::BankQuerier;
use std::str::FromStr;

use crate::{
    contract::OWNER,
    state::{is_token, remove_token as remove_token_from_list, save_token},
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
    save_token(deps, token)?;

    Ok(Response::default())
}

pub fn remove_token(deps: DepsMut, info: MessageInfo, token: String) -> StdResult<Response> {
    // check permission
    if !OWNER.is_admin(deps.as_ref(), &info.sender)? {
        return Err(StdError::generic_err("unauthorized"));
    }

    // remove token here
    remove_token_from_list(deps, token)?;

    Ok(Response::default())
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

    // check permissions to send the message
    if !OWNER.is_admin(deps, &info.sender)? {
        return Err(StdError::generic_err("unauthorized"));
    }

    // validate the recipient address
    let valid_recipient = deps.api.addr_validate(&recipient)?;

    // check that the token is in the token list
    if !is_token(deps.storage, token.clone()) {
        return Err(StdError::generic_err("This token is not supported"));
    };

    // query the balance of the given token that this contract holds
    let bank = BankQuerier::new(&deps.querier);

    let balance = match bank
        .balance(env.contract.address.to_string(), token.clone())
        .unwrap()
        .balance
    {
        Some(balance) => Uint128::from_str(balance.amount.as_str()).unwrap(),
        None => Uint128::zero(),
    };

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

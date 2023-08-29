use cosmwasm_std::{
    BalanceResponse, BankQuery, Coin, Deps, DepsMut, Env, IbcMsg, MessageInfo, QueryRequest,
    Response, StdError, StdResult, Uint128,
};

pub const PACKET_LIFETIME: u64 = 60 * 60; // One hour

use crate::{
    contract::OWNER,
    state::{Config, CONFIG},
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
    info: MessageInfo,
    ibc_to_address: String,
    ibc_channel_id: String,
    liquidation_target: String,
) -> StdResult<Response> {
    if !OWNER.is_admin(deps.as_ref(), &info.sender)? {
        return Err(StdError::generic_err("unauthorized"));
    }
    CONFIG.save(
        deps.storage,
        &Config {
            ibc_to_address,
            ibc_channel_id,
            liquidation_target,
        },
    )?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn ibc_transfer(deps: Deps, env: Env, _info: MessageInfo) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    let liquidation_target = config.liquidation_target.clone();

    let res: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: env.contract.address.to_string(),
        denom: liquidation_target,
    }))?;

    let balance = res.amount.amount;

    if balance.is_zero() {
        return Err(StdError::generic_err("Balance is zero"));
    }

    let msg = IbcMsg::Transfer {
        channel_id: config.ibc_channel_id,
        to_address: config.ibc_to_address,
        amount: Coin {
            amount: Uint128::from(50u128),
            denom: config.liquidation_target,
        },
        timeout: env.block.time.plus_seconds(PACKET_LIFETIME).into(),
    };

    let res = Response::new().add_message(msg).add_attribute("action", "handle_send_funds");

    Ok(res)
}

pub fn liquidate(_deps: Deps, _env: Env, _info: MessageInfo) -> StdResult<Response> {
    let res = Response::new();
    Ok(res)
}

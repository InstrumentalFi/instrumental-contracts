use cosmwasm_std::{
    BalanceResponse, BankQuery, Coin, Deps, DepsMut, Env, IbcMsg, MessageInfo, QueryRequest,
    Response,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

pub const PACKET_LIFETIME: u64 = 60 * 60; // One hour

use crate::{
    error::ContractError,
    helpers::{validate_is_owner, validate_pool_route},
    state::{Config, CONFIG, OWNER, ROUTING_TABLE},
};

pub fn update_owner(
    deps: DepsMut,
    info: MessageInfo,
    owner: String,
) -> Result<Response, ContractError> {
    validate_is_owner(deps.as_ref(), info.sender)?;
    let new_owner = deps.api.addr_validate(&owner)?;
    OWNER.save(deps.storage, &new_owner)?;
    Ok(Response::new()
        .add_attribute("action", "change_contract_owner")
        .add_attribute("new_owner", new_owner))
}

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    ibc_to_address: String,
    ibc_channel_id: String,
    liquidation_target: String,
) -> Result<Response, ContractError> {
    validate_is_owner(deps.as_ref(), info.sender)?;
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

pub fn ibc_transfer(deps: Deps, env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let liquidation_target = config.liquidation_target.clone();

    let res: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: env.contract.address.to_string(),
        denom: liquidation_target,
    }))?;

    let balance = res.amount.amount;

    if balance.is_zero() {
        return Err(ContractError::ZeroAmount {});
    }

    let msg = IbcMsg::Transfer {
        channel_id: config.ibc_channel_id,
        to_address: config.ibc_to_address,
        amount: Coin {
            amount: balance,
            denom: config.liquidation_target,
        },
        timeout: env.block.time.plus_seconds(PACKET_LIFETIME).into(),
    };

    let res = Response::new().add_message(msg).add_attribute("action", "handle_send_funds");

    Ok(res)
}

pub fn liquidate(_deps: Deps, _env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    // for item in ROUTING_TABLE.range(deps.storage, None, None, Order::Ascending) {
    //     let ((key1, key2), values) = item?;
    // }
    let res = Response::new();
    Ok(res)
}

pub fn set_route(
    deps: DepsMut,
    info: MessageInfo,
    input_denom: String,
    output_denom: String,
    pool_route: Vec<SwapAmountInRoute>,
) -> Result<Response, ContractError> {
    validate_is_owner(deps.as_ref(), info.sender)?;

    validate_pool_route(
        deps.as_ref(),
        input_denom.clone(),
        output_denom.clone(),
        pool_route.clone(),
    )?;

    ROUTING_TABLE.save(deps.storage, (&input_denom, &output_denom), &pool_route)?;

    Ok(Response::new().add_attribute("action", "set_route"))
}

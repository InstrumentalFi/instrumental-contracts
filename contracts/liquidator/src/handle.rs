use cosmwasm_std::{
    ensure, Addr, BalanceResponse, BankQuery, Coin, Deps, DepsMut, Env, Event, IbcMsg, MessageInfo,
    Order, QueryRequest, Response, Uint128,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::{MsgSwapExactAmountIn, SwapAmountInRoute};

pub const PACKET_LIFETIME: u64 = 60 * 60; // One hour

use crate::{
    error::ContractError,
    helpers::{generate_swap_msg, validate_pool_route},
    state::{CONFIG, OWNER, ROUTING_TABLE},
};

pub fn update_owner(
    deps: DepsMut,
    info: MessageInfo,
    owner: String,
) -> Result<Response, ContractError> {
    let valid_owner = deps.api.addr_validate(&owner)?;
    match OWNER.execute_update_admin(deps, info, Some(valid_owner)) {
        Ok(response) => Ok(response),
        Err(_e) => Err(ContractError::OwnerUpdateError {}),
    }
}

pub fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    ibc_to_address: Option<String>,
    ibc_channel_id: Option<String>,
    liquidation_target: Option<String>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    ensure!(OWNER.is_admin(deps.as_ref(), &info.sender)?, ContractError::Unauthorized {});
    let mut event = Event::new("update_config");

    if let Some(ibc_to_address) = ibc_to_address {
        config.ibc_to_address = deps.api.addr_validate(&ibc_to_address)?;
        event = event.add_attribute("ibc_to_address", ibc_to_address);
    }

    if let Some(ibc_channel_id) = ibc_channel_id {
        config.ibc_channel_id = ibc_channel_id.clone();
        event = event.add_attribute("ibc_channel_id", ibc_channel_id);
    }

    if let Some(liquidation_target) = liquidation_target {
        config.liquidation_target = liquidation_target.clone();
        event = event.add_attribute("liquidation_target", liquidation_target);
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default().add_event(event))
}

pub fn ibc_transfer(deps: Deps, env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let liquidation_target = config.liquidation_target.clone();

    let res: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: env.contract.address.to_string(),
        denom: liquidation_target,
    }))?;

    let balance = res.amount.amount;

    let msg = IbcMsg::Transfer {
        channel_id: config.ibc_channel_id,
        to_address: config.ibc_to_address.to_string(),
        amount: Coin {
            amount: balance,
            denom: config.liquidation_target,
        },
        timeout: env.block.time.plus_seconds(PACKET_LIFETIME).into(),
    };

    let res = Response::new().add_message(msg).add_attribute("action", "handle_send_funds");

    Ok(res)
}

pub fn liquidate(deps: Deps, env: Env, _info: MessageInfo) -> Result<Response, ContractError> {
    let contract_address = env.contract.address.to_string();
    let mut swap_msgs: Vec<MsgSwapExactAmountIn> = Vec::new();

    // Loop through pairs in ROUTING_TABLE
    // If the contract has balance liquidate to target via the route
    for item in ROUTING_TABLE.range(deps.storage, None, None, Order::Ascending) {
        let ((token1, token2), _routes) = item?;

        // Check if the contract has any balance of the token_in denom
        let res: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
            address: contract_address.clone(),
            denom: token1.clone(),
        }))?;

        let balance = res.amount.amount;

        // If there is some balance liquidate via the route
        if !balance.is_zero() {
            let token_in = Coin {
                amount: res.amount.amount,
                denom: token1,
            };

            let token_out = Coin {
                amount: Uint128::from(1u128),
                denom: token2,
            };
            let address = Addr::unchecked(contract_address.clone());
            let msg = generate_swap_msg(deps, address, token_in, token_out);
            swap_msgs.push(msg.unwrap());
        }
    }
    let res = Response::new().add_messages(swap_msgs).add_attribute("action", "liquidate");
    Ok(res)
}

pub fn set_route(
    deps: DepsMut,
    info: MessageInfo,
    input_denom: String,
    output_denom: String,
    pool_route: Vec<SwapAmountInRoute>,
) -> Result<Response, ContractError> {
    ensure!(OWNER.is_admin(deps.as_ref(), &info.sender)?, ContractError::Unauthorized {});

    validate_pool_route(
        deps.as_ref(),
        input_denom.clone(),
        output_denom.clone(),
        pool_route.clone(),
    )?;

    ROUTING_TABLE.save(deps.storage, (&input_denom, &output_denom), &pool_route)?;

    Ok(Response::new().add_attribute("action", "set_route"))
}

pub fn remove_route(
    deps: DepsMut,
    info: MessageInfo,
    input_denom: &str,
    output_denom: &str,
) -> Result<Response, ContractError> {
    ensure!(OWNER.is_admin(deps.as_ref(), &info.sender)?, ContractError::Unauthorized {});
    ROUTING_TABLE.remove(deps.storage, (input_denom, output_denom));
    Ok(Response::new().add_attribute("action", "delete_route"))
}

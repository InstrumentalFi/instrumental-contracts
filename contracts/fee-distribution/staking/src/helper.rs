use crate::state::CONFIG;

use cosmwasm_std::{
    to_binary, Coin, CosmosMsg, Deps, QueryRequest, StdError, StdResult, Uint128, WasmMsg,
    WasmQuery,
};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg, MinterResponse, TokenInfoResponse};
use fee_distribution::collector::ExecuteMsg as FeeExecuteMsg;
use osmosis_std::types::cosmos::bank::v1beta1::BankQuerier;
use std::str::FromStr;

pub fn get_bank_balance(deps: Deps, address: String, denom: String) -> Uint128 {
    let bank = BankQuerier::new(&deps.querier);

    match bank.balance(address, denom).unwrap().balance {
        Some(balance) => Uint128::from_str(balance.amount.as_str()).unwrap(),
        None => Uint128::zero(),
    }
}

pub fn get_token_total_supply(deps: Deps) -> Uint128 {
    let config = CONFIG.load(deps.storage).unwrap();

    let res: TokenInfoResponse = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: config.staked_denom,
            msg: to_binary(&Cw20QueryMsg::TokenInfo {}).unwrap(),
        }))
        .unwrap();

    res.total_supply
}

pub fn create_distribute_message(
    fee_collector: String,
    token: String,
    amount: Uint128,
    recipient: String,
) -> CosmosMsg {
    CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: fee_collector,
        msg: to_binary(&FeeExecuteMsg::SendToken {
            token,
            amount,
            recipient,
        })
        .unwrap(),
        funds: vec![],
    })
}

pub fn parse_funds(funds: Vec<Coin>, expected_denom: String) -> StdResult<Uint128> {
    if funds.is_empty() {
        return Ok(Uint128::zero());
    };

    if funds.len() != 1 || funds[0].denom != expected_denom {
        return Err(StdError::generic_err("Invalid Funds"));
    }

    Ok(funds[0].amount)
}

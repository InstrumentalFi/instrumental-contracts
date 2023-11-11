use cosmwasm_std::{
    from_json, wasm_execute, wasm_instantiate, CosmosMsg, DepsMut, Env, MessageInfo, Response,
    SubMsg, Uint128,
};
use cw20::{Cw20ExecuteMsg, Cw20ReceiveMsg, MinterResponse};
use fee_distribution::staking::{Cw20HookMsg, Cw20TokenInstantiateMsg};

use crate::{contract::INSTANTIATE_REPLY_ID, error::ContractError, handle::handle_unstake};

pub fn receive_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    match from_json(&cw20_msg.msg)? {
        Cw20HookMsg::Unstake {} => handle_unstake(deps, env, info, cw20_msg),
    }
}

pub fn create_instantiate_token_msg(
    code_id: u64,
    token_name: String,
    token_symbol: String,
    decimals: u8,
    contract_address: String,
) -> SubMsg {
    SubMsg::reply_on_success(
        wasm_instantiate(
            code_id,
            &Cw20TokenInstantiateMsg {
                name: token_name,
                symbol: format!("ve{}", token_symbol),
                decimals,
                initial_balances: vec![],
                mint: Some(MinterResponse {
                    minter: contract_address,
                    cap: None,
                }),
                marketing: None,
            },
            vec![],
            String::from("Instrumental voting escrow token"),
        )
        .unwrap(),
        INSTANTIATE_REPLY_ID,
    )
}

pub fn create_mint_token_msg(
    amount: Uint128,
    recipient: String,
    contract_address: String,
) -> CosmosMsg {
    wasm_execute(
        contract_address,
        &Cw20ExecuteMsg::Mint {
            amount,
            recipient,
        },
        vec![],
    )
    .unwrap()
    .into()
}

pub fn create_burn_token_msg(amount: Uint128, contract_address: String) -> CosmosMsg {
    wasm_execute(
        contract_address,
        &Cw20ExecuteMsg::Burn {
            amount,
        },
        vec![],
    )
    .unwrap()
    .into()
}

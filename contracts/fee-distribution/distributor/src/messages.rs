use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, Uint128};

pub fn execute_transfer(denom: String, receiver: &Addr, amount: Uint128) -> CosmosMsg {
    CosmosMsg::Bank(BankMsg::Send {
        to_address: receiver.to_string(),
        amount: vec![Coin { denom, amount }],
    })
}

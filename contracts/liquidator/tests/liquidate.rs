#[path = "./common/mod.rs"]
mod common;
use common::TestEnv;
use liquidator::msg::{ExecuteMsg, GetRouteResponse, QueryMsg};
use osmosis_std::types::{
    cosmos::{
        bank::v1beta1::{MsgSend, QueryBalanceRequest},
        base::v1beta1::Coin,
    },
    osmosis::poolmanager::v1beta1::SwapAmountInRoute,
};
use osmosis_test_tube::{Account, Bank, Module, Wasm};

#[test]
fn test_liquidate() {
    let TestEnv {
        app,
        contract_address,
        owner,
    } = TestEnv::new();
    let wasm = Wasm::new(&app);
    let bank = Bank::new(&app);

    // Set a route for the test uosmo/uion
    let set_route_msg = ExecuteMsg::SetRoute {
        input_denom: "uosmo".to_string(),
        output_denom: "uion".to_string(),
        pool_route: vec![SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: "uion".to_string(),
        }],
    };

    wasm.execute(&contract_address, &set_route_msg, &[], &owner).unwrap();

    // Verify this was set correctly
    let resp: GetRouteResponse = wasm
        .query(
            &contract_address,
            &QueryMsg::GetRoute {
                input_denom: "uosmo".to_string(),
                output_denom: "uion".to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        resp.pool_route,
        vec![SwapAmountInRoute {
            pool_id: 1, // uion/uosmo
            token_out_denom: "uion".to_string(),
        },],
    );

    // Send the contract some uosmo
    bank.send(
        MsgSend {
            from_address: owner.address(),
            to_address: contract_address.clone(),
            amount: vec![Coin {
                amount: (5_00 * 10u128.pow(6)).to_string(),
                denom: "uosmo".to_string(),
            }],
        },
        &owner,
    )
    .unwrap();

    // Verify the contract has the balance
    let balance = bank
        .query_balance(&QueryBalanceRequest {
            address: contract_address.clone(),
            denom: "uosmo".to_string(),
        })
        .unwrap()
        .balance
        .unwrap_or_default()
        .amount;

    assert_eq!(balance, "500000000".to_string());

    // Call liquidate
    let msg = ExecuteMsg::Liquidate {};
    wasm.execute(&contract_address, &msg, &[], &owner).unwrap();

    // Verify that the uosmo balance is zero
    let balance = bank
        .query_balance(&QueryBalanceRequest {
            address: contract_address.clone(),
            denom: "uosmo".to_string(),
        })
        .unwrap()
        .balance
        .unwrap_or_default()
        .amount;

    assert_eq!(balance, "0".to_string());

    // Verify that the uion balance is greater than zero
    let balance = bank
        .query_balance(&QueryBalanceRequest {
            address: contract_address.clone(),
            denom: "uion".to_string(),
        })
        .unwrap()
        .balance
        .unwrap_or_default()
        .amount;
    assert_ne!(balance, "0".to_string());
}

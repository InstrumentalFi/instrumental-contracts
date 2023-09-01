mod common;
use cosmwasm_std::Coin;
use liquidator::msg::{ExecuteMsg, GetRouteResponse, QueryMsg};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use osmosis_test_tube::{Module, Wasm};

const INITIAL_AMOUNT: u128 = 1_000_000_000_000;

use common::TestEnv;

#[test]
fn test_adding_single_route_as_owner() {
    let TestEnv {
        app,
        contract_address,
        owner,
    } = TestEnv::new();
    let wasm = Wasm::new(&app);

    // setup route
    // uosmo/uion = pool(2): uosmo/stake -> pool(3): stake/uion
    let set_route_msg = ExecuteMsg::SetRoute {
        input_denom: "uosmo".to_string(),
        output_denom: "uion".to_string(),
        pool_route: vec![SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: "uion".to_string(),
        }],
    };

    wasm.execute(&contract_address, &set_route_msg, &[], &owner).unwrap();

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
            pool_id: 1, // stake/uosmo
            token_out_denom: "uion".to_string(),
        },],
    )
}

#[test]
fn test_adding_route_as_not_owner() {
    let TestEnv {
        app,
        contract_address,
        owner: _,
    } = TestEnv::new();
    let wasm = Wasm::new(&app);

    let initial_balance = [
        Coin::new(INITIAL_AMOUNT, "uosmo"),
        Coin::new(INITIAL_AMOUNT, "uion"),
        Coin::new(INITIAL_AMOUNT, "stake"),
    ];

    let alice = app.init_account(&initial_balance).unwrap();

    let set_route_msg = ExecuteMsg::SetRoute {
        input_denom: "uosmo".to_string(),
        output_denom: "uion".to_string(),
        pool_route: vec![SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: "uion".to_string(),
        }],
    };

    let res = wasm.execute(&contract_address, &set_route_msg, &[], &alice).unwrap_err();

    assert_eq!(res.to_string(), "execute error: failed to execute message; message index: 0: Unauthorized: execute wasm contract failed");
}

#[test]
fn test_adding_multihop_route_as_owner() {
    let TestEnv {
        app,
        contract_address,
        owner,
    } = TestEnv::new();
    let wasm = Wasm::new(&app);

    // setup route
    // uosmo/uion = pool(2): uosmo/stake -> pool(3): stake/uion
    let set_route_msg = ExecuteMsg::SetRoute {
        input_denom: "uosmo".to_string(),
        output_denom: "uion".to_string(),
        pool_route: vec![
            SwapAmountInRoute {
                pool_id: 2,
                token_out_denom: "stake".to_string(),
            },
            SwapAmountInRoute {
                pool_id: 3,
                token_out_denom: "uion".to_string(),
            },
        ],
    };

    wasm.execute(&contract_address, &set_route_msg, &[], &owner).unwrap();

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
        vec![
            SwapAmountInRoute {
                pool_id: 2, // stake/uosmo
                token_out_denom: "stake".to_string(),
            },
            SwapAmountInRoute {
                pool_id: 3, // stake/uion
                token_out_denom: "uion".to_string(),
            },
        ],
    )
}

#[test]
fn test_output_denom_not_match() {
    let TestEnv {
        app,
        contract_address,
        owner,
    } = TestEnv::new();
    let wasm = Wasm::new(&app);

    // setup route
    // uosmo/uion = pool(2): uosmo/stake -> pool(3): stake/uion
    let set_route_msg = ExecuteMsg::SetRoute {
        input_denom: "uosmo".to_string(),
        output_denom: "uion".to_string(),
        pool_route: vec![SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: "uosmo".to_string(),
        }],
    };

    let res = wasm.execute(&contract_address, &set_route_msg, &[], &owner).unwrap_err();

    assert_eq!(res.to_string(), "execute error: failed to execute message; message index: 0: Invalid Pool Route: \"last denom doesn't match\": execute wasm contract failed");
}

#[test]
fn test_output_denom_not_in_pool() {
    let TestEnv {
        app,
        contract_address,
        owner,
    } = TestEnv::new();
    let wasm = Wasm::new(&app);

    let set_route_msg = ExecuteMsg::SetRoute {
        input_denom: "stake".to_string(),
        output_denom: "uosmo".to_string(),
        pool_route: vec![SwapAmountInRoute {
            pool_id: 1, // uosmo/uion
            token_out_denom: "stake".to_string(),
        }],
    };

    let res = wasm.execute(&contract_address, &set_route_msg, &[], &owner).unwrap_err();

    assert_eq!(res.to_string(), "execute error: failed to execute message; message index: 0: Invalid Pool Route: \"denom stake is not in pool id 1\": execute wasm contract failed");
}

#[test]
fn test_pool_does_not_have_output_asset() {
    let TestEnv {
        app,
        contract_address,
        owner,
    } = TestEnv::new();
    let wasm = Wasm::new(&app);

    // setup route
    // uosmo/uion = pool(2): uosmo/stake -> pool(3): stake/uion
    let set_route_msg = ExecuteMsg::SetRoute {
        input_denom: "uosmo".to_string(),
        output_denom: "uion".to_string(),
        pool_route: vec![SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: "uosmo".to_string(),
        }],
    };

    let res = wasm.execute(&contract_address, &set_route_msg, &[], &owner).unwrap_err();

    assert_eq!(res.to_string(), "execute error: failed to execute message; message index: 0: Invalid Pool Route: \"last denom doesn't match\": execute wasm contract failed");
}

mod common;
use common::TestEnv;
use liquidator::msg::{ExecuteMsg, GetAllRoutesResponse, GetRouteResponse, QueryMsg};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use osmosis_test_tube::{Module, RunnerResult, Wasm};

#[test]
fn test_query_route() {
    let TestEnv {
        app,
        contract_address,
        owner,
    } = TestEnv::new();
    let wasm = Wasm::new(&app);

    // setup route
    // uosmo/uion = pool(1)
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
fn test_query_route_no_route() {
    let TestEnv {
        app,
        contract_address,
        owner: _,
    } = TestEnv::new();
    let wasm = Wasm::new(&app);

    let resp: RunnerResult<GetRouteResponse> = wasm.query(
        &contract_address,
        &QueryMsg::GetRoute {
            input_denom: "uosmo".to_string(),
            output_denom: "uion".to_string(),
        },
    );
    let err = resp.unwrap_err();
    assert_eq!(err.to_string(), "query error: Route not found: query wasm contract failed");
}

#[test]
fn test_query_all_routes_one_route() {
    let TestEnv {
        app,
        contract_address,
        owner,
    } = TestEnv::new();
    let wasm = Wasm::new(&app);

    // setup route
    // uosmo/uion = pool(1)
    let set_route_msg = ExecuteMsg::SetRoute {
        input_denom: "uosmo".to_string(),
        output_denom: "uion".to_string(),
        pool_route: vec![SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: "uion".to_string(),
        }],
    };

    wasm.execute(&contract_address, &set_route_msg, &[], &owner).unwrap();

    let resp: GetAllRoutesResponse = wasm
        .query(
            &contract_address,
            &QueryMsg::GetAllRoutes {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();

    let expected_route = vec![SwapAmountInRoute {
        pool_id: 1, // stake/uosmo
        token_out_denom: "uion".to_string(),
    }];

    if let Some(route) = resp.routes.get("uosmo:uion") {
        assert_eq!(*route, expected_route);
    } else {
        panic!("Route uosmo:uion not found!");
    }
}

#[test]
fn test_query_all_routes_two_routes() {
    let TestEnv {
        app,
        contract_address,
        owner,
    } = TestEnv::new();
    let wasm = Wasm::new(&app);

    // setup route
    // uosmo/uion = pool(1)
    let set_route_msg_1 = ExecuteMsg::SetRoute {
        input_denom: "uosmo".to_string(),
        output_denom: "uion".to_string(),
        pool_route: vec![SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: "uion".to_string(),
        }],
    };

    wasm.execute(&contract_address, &set_route_msg_1, &[], &owner).unwrap();

    let set_route_msg_2 = ExecuteMsg::SetRoute {
        input_denom: "uosmo".to_string(),
        output_denom: "stake".to_string(),
        pool_route: vec![SwapAmountInRoute {
            pool_id: 2,
            token_out_denom: "stake".to_string(),
        }],
    };

    wasm.execute(&contract_address, &set_route_msg_2, &[], &owner).unwrap();

    let resp: GetAllRoutesResponse = wasm
        .query(
            &contract_address,
            &QueryMsg::GetAllRoutes {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();

    assert_eq!(resp.routes.len(), 2, "Expected exactly 2 routes in the hashmap");
}

#[test]
fn test_query_all_routes_no_routes() {
    let TestEnv {
        app,
        contract_address,
        owner: _,
    } = TestEnv::new();
    let wasm = Wasm::new(&app);

    let resp: GetAllRoutesResponse = wasm
        .query(
            &contract_address,
            &QueryMsg::GetAllRoutes {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();

    assert_eq!(resp.routes.len(), 0, "Expected exactly 0 routes in the hashmap");
}

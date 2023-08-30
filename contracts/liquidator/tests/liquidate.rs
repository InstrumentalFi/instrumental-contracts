mod common;
use common::TestEnv;
use liquidator::msg::ExecuteMsg;
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use osmosis_test_tube::{Module, Wasm};

#[test]
fn test_output() {
    let TestEnv {
        app,
        contract_address,
        owner,
    } = TestEnv::new();
    let wasm = Wasm::new(&app);

    let set_route_msg = ExecuteMsg::SetRoute {
        input_denom: "uosmo".to_string(),
        output_denom: "uion".to_string(),
        pool_route: vec![SwapAmountInRoute {
            pool_id: 1,
            token_out_denom: "uion".to_string(),
        }],
    };

    wasm.execute(&contract_address, &set_route_msg, &[], &owner).unwrap();

    let msg = ExecuteMsg::Liquidate {};

    wasm.execute(&contract_address, &msg, &[], &owner).unwrap();
    assert_eq!(1, 1);
}

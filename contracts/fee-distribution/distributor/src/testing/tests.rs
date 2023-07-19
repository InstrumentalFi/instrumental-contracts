use crate::{
    contract::{execute, instantiate, query},
    state::Config,
};
use cosmrs::proto::cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, Addr, Uint128};
use fee_distribution::distributor::{ExecuteMsg, InstantiateMsg, OwnerResponse, QueryMsg};
use osmosis_test_tube::{Account, Bank, Module, Wasm};
use testing::staking_env::StakingEnv;

#[test]
fn test_instantiation() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        token: "uusd".to_string(),
        distribution: vec![
            ("addr0000".to_string(), Uint128::from(500_000u128)),
            ("addr0001".to_string(), Uint128::from(500_000u128)),
        ],
    };

    let info = mock_info("addr0000", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
    let resp: OwnerResponse = from_binary(&res).unwrap();
    let owner = resp.owner;

    assert_eq!(owner, Addr::unchecked("addr0000".to_string()));

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetToken {}).unwrap();
    let token: String = from_binary(&res).unwrap();
    assert_eq!(owner, "uusd".to_string());

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
    let resp: Config = from_binary(&res).unwrap();
    let distribution = resp.distribution;

    let expected_address = Addr::unchecked("addr0000".to_string());
    let expected_value = Uint128::from(500_000u128);

    assert_eq!(
        distribution.iter().find(|(address, _)| *address == expected_address),
        Some(&(expected_address, expected_value))
    );

    let expected_address = Addr::unchecked("addr0001".to_string());

    assert_eq!(
        distribution.iter().find(|(address, _)| *address == expected_address),
        Some(&(expected_address, expected_value))
    );
}

#[test]
fn test_update_owner() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        token: "uusd".to_string(),
        distribution: vec![
            ("addr0000".to_string(), Uint128::from(500_000u128)),
            ("addr0001".to_string(), Uint128::from(500_000u128)),
        ],
    };
    let info = mock_info("addr0000", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Update the owner
    let msg = ExecuteMsg::UpdateOwner {
        owner: "addr0001".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
    let resp: OwnerResponse = from_binary(&res).unwrap();
    let owner = resp.owner;

    assert_eq!(owner, Addr::unchecked("addr0001".to_string()));
}

#[test]
fn test_update_config() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        token: "uusd".to_string(),
        distribution: vec![
            ("addr0000".to_string(), Uint128::from(500_000u128)),
            ("addr0001".to_string(), Uint128::from(500_000u128)),
        ],
    };
    let info = mock_info("addr0000", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Update the owner
    let msg = ExecuteMsg::UpdateConfig {
        distribution: vec![
            ("addr0000".to_string(), Uint128::from(200_000u128)),
            ("addr0001".to_string(), Uint128::from(300_000u128)),
            ("addr0002".to_string(), Uint128::from(500_000u128)),
        ],
    };

    let info = mock_info("addr0000", &[]);
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
    let resp: Config = from_binary(&res).unwrap();
    let distribution = resp.distribution;

    let expected_address = Addr::unchecked("addr0000".to_string());
    let expected_value = Uint128::from(200_000u128);

    assert_eq!(
        distribution.iter().find(|(address, _)| *address == expected_address),
        Some(&(expected_address, expected_value))
    );

    let expected_address = Addr::unchecked("addr0001".to_string());
    let expected_value = Uint128::from(300_000u128);

    assert_eq!(
        distribution.iter().find(|(address, _)| *address == expected_address),
        Some(&(expected_address, expected_value))
    );

    let expected_address = Addr::unchecked("addr0002".to_string());
    let expected_value = Uint128::from(500_000u128);

    assert_eq!(
        distribution.iter().find(|(address, _)| *address == expected_address),
        Some(&(expected_address, expected_value))
    );
}

#[test]
fn test_query_token() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        distribution: vec![
            ("addr0000".to_string(), Uint128::from(500_000u128)),
            ("addr0001".to_string(), Uint128::from(500_000u128)),
        ],
    };
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // add token to tokenlist here
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::AddToken {
        token: "token1".to_string(),
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // query if the token has been added
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::IsToken {
            token: "token1".to_string(),
        },
    )
    .unwrap();

    let res: TokenResponse = from_binary(&res).unwrap();
    let is_token = res.is_token;

    assert!(is_token);
}

#[test]
fn test_query_token() {
    // instantiate contract here
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        token: "uusd".to_string(),
        distribution: vec![
            ("addr0000".to_string(), Uint128::from(500_000u128)),
            ("addr0001".to_string(), Uint128::from(500_000u128)),
        ],
    };
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // check for the added tokens
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetToken {}).unwrap();

    let token: String = from_binary(&res).unwrap();

    assert_eq!(token, "uusd".to_string());
}

#[test]
fn test_distribute_token() {
    // Using the native token, we only work to 6dp

    let env = StakingEnv::new();

    let wasm = Wasm::new(&env.app);
    let bank = Bank::new(&env.app);

    let distribution = vec![
        (env.traders[0].address(), Uint128::from(500_000u128)),
        (env.traders[1].address(), Uint128::from(300_000u128)),
        (env.traders[2].address(), Uint128::from(200_000u128)),
    ];

    let distributor =
        env.deploy_distributor_contract(&wasm, "distributor".to_string(), distribution);

    // give funds to the fee pool contract
    bank.send(
        MsgSend {
            from_address: env.signer.address(),
            to_address: distributor.clone(),
            amount: vec![Coin {
                amount: (5_000 * 10u128.pow(6)).to_string(),
                denom: "ubase".to_string(),
            }],
        },
        &env.signer,
    )
    .unwrap();

    // query balance of bob
    let balance = env.get_balance(env.empty.address(), "ubase".to_string());
    assert_eq!(balance, Uint128::zero());

    // query balance of contract
    let balance = env.get_balance(distributor.clone(), "ubase".to_string());
    assert_eq!(balance, Uint128::from(5_000u128 * 10u128.pow(6)));

    // send token
    wasm.execute(&distributor, &ExecuteMsg::Distribute {}, &[], &env.signer).unwrap();

    // query new balance of intended recipient
    let balance = env.get_balance(env.empty.address(), "ubase".to_string());
    assert_eq!(balance, Uint128::from(1_000u128 * 10u128.pow(6)));

    // Query new contract balance
    let balance = env.get_balance(distributor, "ubase".to_string());
    assert_eq!(balance, Uint128::from(4000u128 * 10u128.pow(6)));
}

// #[test]
// fn test_send_native_token_unsupported_token() {
//     let env = StakingEnv::new();

//     let wasm = Wasm::new(&env.app);
//     let bank = Bank::new(&env.app);

//     let distributor = env.deploy_distributor_contract(&wasm, "collector".to_string());

//     // give funds to the fee pool contract
//     bank.send(
//         MsgSend {
//             from_address: env.signer.address(),
//             to_address: distributor.clone(),
//             amount: vec![Coin {
//                 amount: (5_000u128 * 10u128.pow(6)).to_string(),
//                 denom: "ubase".to_string(),
//             }],
//         },
//         &env.signer,
//     )
//     .unwrap();

//     // try to send token - note this fails because we have not added the token to the token list, so it is not accepted/supported yet
//     let res = wasm
//         .execute(
//             &distributor,
//             &ExecuteMsg::SendToken {
//                 token: "ubase".to_string(),
//                 amount: Uint128::from(1000u128 * 10u128.pow(6)),
//                 recipient: env.empty.address(),
//             },
//             &[],
//             &env.signer,
//         )
//         .unwrap_err();
//     assert_eq!(
//         "execute error: failed to execute message; message index: 0: Generic error: This token is not supported: execute wasm contract failed",
//         res.to_string()
//     );
// }

// #[test]
// fn test_send_native_token_insufficient_balance() {
//     let env = StakingEnv::new();

//     let wasm = Wasm::new(&env.app);
//     let bank = Bank::new(&env.app);

//     let distributor = env.deploy_distributor_contract(&wasm, "collector".to_string());

//     // give funds to the fee pool contract
//     bank.send(
//         MsgSend {
//             from_address: env.signer.address(),
//             to_address: distributor.clone(),
//             amount: vec![Coin {
//                 amount: (1_000u128 * 10u128.pow(6)).to_string(),
//                 denom: "ubase".to_string(),
//             }],
//         },
//         &env.signer,
//     )
//     .unwrap();

//     // add the token so we can send funds with it
//     wasm.execute(
//         &distributor,
//         &ExecuteMsg::AddToken {
//             token: "ubase".to_string(),
//         },
//         &[],
//         &env.signer,
//     )
//     .unwrap();

//     // query balance of bob
//     let balance = env.get_balance(env.empty.address(), "ubase".to_string());
//     assert_eq!(balance, Uint128::zero());

//     // query balance of contract
//     let balance = env.get_balance(distributor.clone(), "ubase".to_string());
//     assert_eq!(balance, Uint128::from(1000u128 * 10u128.pow(6)));

//     // send token
//     let res = wasm
//         .execute(
//             &distributor,
//             &ExecuteMsg::SendToken {
//                 token: "ubase".to_string(),
//                 amount: Uint128::from(2000u128 * 10u128.pow(6)),
//                 recipient: env.empty.address(),
//             },
//             &[],
//             &env.signer,
//         )
//         .unwrap_err();
//     assert_eq!(
//         "execute error: failed to execute message; message index: 0: Generic error: Insufficient funds: execute wasm contract failed".to_string(),
//         res.to_string()
//     );
//     // query new balance of intended recipient
//     let balance = env.get_balance(env.empty.address(), "ubase".to_string());
//     assert_eq!(balance, Uint128::zero());

//     // Query new contract balance
//     let balance = env.get_balance(distributor, "ubase".to_string());
//     assert_eq!(balance, Uint128::from(1000u128 * 10u128.pow(6)));
// }

// #[test]
// fn test_not_owner() {
//     let env = StakingEnv::new();

//     let wasm = Wasm::new(&env.app);

//     let distributor = env.deploy_distributor_contract(&wasm, "collector".to_string());

//     // instantiate contract here
//     let mut deps = mock_dependencies();
//     let msg = InstantiateMsg {
//         distribution: vec![
//             ("addr0000".to_string(), Uint128::from(5_000_000u128)),
//             ("addr0001".to_string(), Uint128::from(5_000_000u128)),
//         ],
//     };
//     let info = mock_info("owner", &[]);

//     instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//     // try to update the config
//     let res = wasm
//         .execute(
//             &distributor,
//             &ExecuteMsg::UpdateOwner {
//                 owner: env.traders[0].address(),
//             },
//             &[],
//             &env.traders[1],
//         )
//         .unwrap_err();
//     assert_eq!(res.to_string(), "execute error: failed to execute message; message index: 0: Generic error: Caller is not admin: execute wasm contract failed");

//     // try to add a token
//     let res = wasm
//         .execute(
//             &distributor,
//             &ExecuteMsg::AddToken {
//                 token: "ubase".to_string(),
//             },
//             &[],
//             &env.traders[0],
//         )
//         .unwrap_err();
//     assert_eq!(res.to_string(), "execute error: failed to execute message; message index: 0: Generic error: unauthorized: execute wasm contract failed");

//     // try to remove a token
//     let res = wasm
//         .execute(
//             &distributor,
//             &ExecuteMsg::RemoveToken {
//                 token: "token1".to_string(),
//             },
//             &[],
//             &env.traders[0],
//         )
//         .unwrap_err();
//     assert_eq!(res.to_string(), "execute error: failed to execute message; message index: 0: Generic error: unauthorized: execute wasm contract failed");

//     // try to send money
//     let res = wasm
//         .execute(
//             &distributor,
//             &ExecuteMsg::SendToken {
//                 token: "ubase".to_string(),
//                 amount: Uint128::from(2000u128 * 10u128.pow(6)),
//                 recipient: env.traders[0].address(),
//             },
//             &[],
//             &env.traders[0],
//         )
//         .unwrap_err();
//     assert_eq!("execute error: failed to execute message; message index: 0: Generic error: unauthorized: execute wasm contract failed".to_string(), res.to_string());
// }

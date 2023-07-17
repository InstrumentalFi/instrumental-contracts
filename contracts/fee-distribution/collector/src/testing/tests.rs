use crate::contract::{execute, instantiate, query};
use cosmrs::proto::cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{from_binary, Addr, Uint128};
use fee_distribution::collector::{
    AllTokenResponse, ExecuteMsg, InstantiateMsg, OwnerResponse, QueryMsg, TokenLengthResponse,
    TokenResponse, WhitelistResponse,
};
use osmosis_test_tube::{Account, Bank, Module, Wasm};
use testing::staking_env::StakingEnv;

#[test]
fn test_instantiation() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};

    let info = mock_info("addr0000", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
    let resp: OwnerResponse = from_binary(&res).unwrap();
    let owner = resp.owner;

    assert_eq!(owner, Addr::unchecked("addr0000".to_string()));
}

#[test]
fn test_update_owner() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
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
fn test_update_whitelist() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("addr0000", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Update the owner
    let msg = ExecuteMsg::UpdateWhitelist {
        address: "addr0001".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetWhitelist {}).unwrap();
    let resp: WhitelistResponse = from_binary(&res).unwrap();
    let owner = resp.address;

    assert_eq!(owner, Addr::unchecked("addr0001".to_string()));
}

#[test]
fn test_query_token() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
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
fn test_query_all_token() {
    // instantiate contract here
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // check to see that there are no tokens listed
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetTokenList {
            limit: None,
        },
    )
    .unwrap_err();

    assert_eq!(res.to_string(), "Generic error: No tokens are stored");

    // add a token
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::AddToken {
        token: "token1".to_string(),
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // add another token
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::AddToken {
        token: "ubase".to_string(),
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // check for the added tokens
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetTokenList {
            limit: None,
        },
    )
    .unwrap();

    let res: AllTokenResponse = from_binary(&res).unwrap();
    let list = res.token_list;

    assert_eq!(list, vec!["token1".to_string(), "ubase".to_string(),]);
}

#[test]
fn test_add_token() {
    // instantiate contract here
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // query the token we want to add
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

    assert!(!is_token);

    // add a token
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::AddToken {
        token: "token1".to_string(),
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // check for the added token
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
fn test_add_token_twice() {
    // instantiate contract here
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // add a token
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::AddToken {
        token: "token1".to_string(),
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // try to add the same token here
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::AddToken {
        token: "token1".to_string(),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

    assert_eq!(res.to_string(), "Generic error: This token is already added");
}

#[test]
fn test_add_second_token() {
    // instantiate contract here
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // add first token to tokenlist here
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::AddToken {
        token: "token1".to_string(),
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // add second token to tokenlist here
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::AddToken {
        token: "token2".to_string(),
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // check for the second added token
    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::IsToken {
            token: "token2".to_string(),
        },
    )
    .unwrap();

    let res: TokenResponse = from_binary(&res).unwrap();
    let is_token = res.is_token;

    assert!(is_token);
}

#[test]
fn test_remove_token() {
    // instantiate contract here
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // add first token to tokenlist here
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::AddToken {
        token: "token1".to_string(),
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // check to see that there is one token
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

    // remove the first token
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::RemoveToken {
        token: "token1".to_string(),
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // check that the first token is not there
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

    assert!(!is_token);
}

#[test]
fn test_remove_when_no_tokens() {
    // instantiate contract here
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // check to see that there is no token
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

    assert!(!is_token);

    // try to remove the first token
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::RemoveToken {
        token: "token1".to_string(),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

    assert_eq!(res.to_string(), "Generic error: No tokens are stored")
}

#[test]
fn test_remove_non_existed_token() {
    // instantiate contract here
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // add a token
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::AddToken {
        token: "token1".to_string(),
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // check to see that there is one token
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

    // remove a token which isn't stored
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::RemoveToken {
        token: "token2".to_string(),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

    assert_eq!(res.to_string(), "Generic error: This token has not been added")
}

#[test]
fn test_token_capacity() {
    // for the purpose of this test, TOKEN_LIMIT is set to 3 (so four exceeds it!)
    // instantiate contract here
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let tokens: Vec<String> = vec![
        "token1".to_string(),
        "token2".to_string(),
        "token3".to_string(),
        "token4".to_string(),
    ];

    // add three tokens
    for n in 1..4 {
        let info = mock_info("owner", &[]);
        let msg = ExecuteMsg::AddToken {
            token: tokens[n - 1].clone(),
        };

        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    }

    // try to add a fourth token
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::AddToken {
        token: "token4".to_string(),
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

    assert_eq!(res.to_string(), "Generic error: The token capacity is already reached");

    // instantiate contract here
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let tokens: Vec<String> = vec![
        "token1".to_string(),
        "token2".to_string(),
        "token3".to_string(),
        "token4".to_string(),
    ];

    // add four vamms
    for n in 1..5 {
        let info = mock_info("owner", &[]);
        let msg = ExecuteMsg::AddToken {
            token: tokens[n - 1].clone(),
        };

        if n == 4 {
            let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

            assert_eq!(res.to_string(), "Generic error: The token capacity is already reached");
            break;
        }
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    }
}

#[test]
fn test_token_length() {
    // instantiate contract here
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // add first token to tokenlist here
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::AddToken {
        token: "token1".to_string(),
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // add second token to tokenlist here
    let info = mock_info("owner", &[]);
    let msg = ExecuteMsg::AddToken {
        token: "token2".to_string(),
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // check for the second added token
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetTokenLength {}).unwrap();

    let res: TokenLengthResponse = from_binary(&res).unwrap();
    let length = res.length;

    assert_eq!(length, 2usize);
}

#[test]
fn test_send_native_token() {
    // Using the native token, we only work to 6dp

    let env = StakingEnv::new();

    let wasm = Wasm::new(&env.app);
    let bank = Bank::new(&env.app);

    let fee_collector = env.deploy_fee_collector_contract(&wasm, "collector".to_string());

    // give funds to the fee pool contract
    bank.send(
        MsgSend {
            from_address: env.signer.address(),
            to_address: fee_collector.clone(),
            amount: vec![Coin {
                amount: (5_000 * 10u128.pow(6)).to_string(),
                denom: "ubase".to_string(),
            }],
        },
        &env.signer,
    )
    .unwrap();

    // add the token so we can send funds with it
    wasm.execute(
        &fee_collector,
        &ExecuteMsg::AddToken {
            token: "ubase".to_string(),
        },
        &[],
        &env.signer,
    )
    .unwrap();

    // query balance of bob
    let balance = env.get_balance(env.empty.address(), "ubase".to_string());
    assert_eq!(balance, Uint128::zero());

    // query balance of contract
    let balance = env.get_balance(fee_collector.clone(), "ubase".to_string());
    assert_eq!(balance, Uint128::from(5_000u128 * 10u128.pow(6)));

    // send token
    wasm.execute(
        &fee_collector,
        &ExecuteMsg::SendToken {
            token: "ubase".to_string(),
            amount: Uint128::from(1000u128 * 10u128.pow(6)),
            recipient: env.empty.address(),
        },
        &[],
        &env.signer,
    )
    .unwrap();

    // query new balance of intended recipient
    let balance = env.get_balance(env.empty.address(), "ubase".to_string());
    assert_eq!(balance, Uint128::from(1_000u128 * 10u128.pow(6)));

    // Query new contract balance
    let balance = env.get_balance(fee_collector, "ubase".to_string());
    assert_eq!(balance, Uint128::from(4000u128 * 10u128.pow(6)));
}

#[test]
fn test_send_native_token_unsupported_token() {
    let env = StakingEnv::new();

    let wasm = Wasm::new(&env.app);
    let bank = Bank::new(&env.app);

    let fee_collector = env.deploy_fee_collector_contract(&wasm, "collector".to_string());

    // give funds to the fee pool contract
    bank.send(
        MsgSend {
            from_address: env.signer.address(),
            to_address: fee_collector.clone(),
            amount: vec![Coin {
                amount: (5_000u128 * 10u128.pow(6)).to_string(),
                denom: "ubase".to_string(),
            }],
        },
        &env.signer,
    )
    .unwrap();

    // try to send token - note this fails because we have not added the token to the token list, so it is not accepted/supported yet
    let res = wasm
        .execute(
            &fee_collector,
            &ExecuteMsg::SendToken {
                token: "ubase".to_string(),
                amount: Uint128::from(1000u128 * 10u128.pow(6)),
                recipient: env.empty.address(),
            },
            &[],
            &env.signer,
        )
        .unwrap_err();
    assert_eq!(
        "execute error: failed to execute message; message index: 0: Generic error: This token is not supported: execute wasm contract failed",
        res.to_string()
    );
}

#[test]
fn test_send_native_token_insufficient_balance() {
    let env = StakingEnv::new();

    let wasm = Wasm::new(&env.app);
    let bank = Bank::new(&env.app);

    let fee_collector = env.deploy_fee_collector_contract(&wasm, "collector".to_string());

    // give funds to the fee pool contract
    bank.send(
        MsgSend {
            from_address: env.signer.address(),
            to_address: fee_collector.clone(),
            amount: vec![Coin {
                amount: (1_000u128 * 10u128.pow(6)).to_string(),
                denom: "ubase".to_string(),
            }],
        },
        &env.signer,
    )
    .unwrap();

    // add the token so we can send funds with it
    wasm.execute(
        &fee_collector,
        &ExecuteMsg::AddToken {
            token: "ubase".to_string(),
        },
        &[],
        &env.signer,
    )
    .unwrap();

    // query balance of bob
    let balance = env.get_balance(env.empty.address(), "ubase".to_string());
    assert_eq!(balance, Uint128::zero());

    // query balance of contract
    let balance = env.get_balance(fee_collector.clone(), "ubase".to_string());
    assert_eq!(balance, Uint128::from(1000u128 * 10u128.pow(6)));

    // send token
    let res = wasm
        .execute(
            &fee_collector,
            &ExecuteMsg::SendToken {
                token: "ubase".to_string(),
                amount: Uint128::from(2000u128 * 10u128.pow(6)),
                recipient: env.empty.address(),
            },
            &[],
            &env.signer,
        )
        .unwrap_err();
    assert_eq!(
        "execute error: failed to execute message; message index: 0: Generic error: Insufficient funds: execute wasm contract failed".to_string(),
        res.to_string()
    );
    // query new balance of intended recipient
    let balance = env.get_balance(env.empty.address(), "ubase".to_string());
    assert_eq!(balance, Uint128::zero());

    // Query new contract balance
    let balance = env.get_balance(fee_collector, "ubase".to_string());
    assert_eq!(balance, Uint128::from(1000u128 * 10u128.pow(6)));
}

#[test]
fn test_not_owner() {
    let env = StakingEnv::new();

    let wasm = Wasm::new(&env.app);

    let fee_collector = env.deploy_fee_collector_contract(&wasm, "collector".to_string());

    // instantiate contract here
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {};
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // try to update the config
    let res = wasm
        .execute(
            &fee_collector,
            &ExecuteMsg::UpdateOwner {
                owner: env.traders[0].address(),
            },
            &[],
            &env.traders[1],
        )
        .unwrap_err();
    assert_eq!(res.to_string(), "execute error: failed to execute message; message index: 0: Generic error: Caller is not admin: execute wasm contract failed");

    // try to add a token
    let res = wasm
        .execute(
            &fee_collector,
            &ExecuteMsg::AddToken {
                token: "ubase".to_string(),
            },
            &[],
            &env.traders[0],
        )
        .unwrap_err();
    assert_eq!(res.to_string(), "execute error: failed to execute message; message index: 0: Generic error: unauthorized: execute wasm contract failed");

    // try to remove a token
    let res = wasm
        .execute(
            &fee_collector,
            &ExecuteMsg::RemoveToken {
                token: "token1".to_string(),
            },
            &[],
            &env.traders[0],
        )
        .unwrap_err();
    assert_eq!(res.to_string(), "execute error: failed to execute message; message index: 0: Generic error: unauthorized: execute wasm contract failed");

    // try to send money
    let res = wasm
        .execute(
            &fee_collector,
            &ExecuteMsg::SendToken {
                token: "ubase".to_string(),
                amount: Uint128::from(2000u128 * 10u128.pow(6)),
                recipient: env.traders[0].address(),
            },
            &[],
            &env.traders[0],
        )
        .unwrap_err();
    assert_eq!("execute error: failed to execute message; message index: 0: Generic error: unauthorized: execute wasm contract failed".to_string(), res.to_string());
}

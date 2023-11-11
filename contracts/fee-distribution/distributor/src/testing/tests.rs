use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env, mock_info},
    Addr, Uint128,
};
use fee_distribution::distributor::{ExecuteMsg, InstantiateMsg, OwnerResponse, QueryMsg};
use instrumental_testing::staking_env::StakingEnv;
use osmosis_std::types::cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin};
use osmosis_test_tube::{Account, Bank, Module, Wasm};

use crate::{
    contract::{execute, instantiate, query},
    state::Config,
};

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
    let resp: OwnerResponse = from_json(&res).unwrap();
    let owner = resp.owner;

    assert_eq!(owner, Addr::unchecked("addr0000".to_string()));

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetToken {}).unwrap();
    let token: String = from_json(&res).unwrap();
    assert_eq!(token, "uusd".to_string());

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
    let resp: Config = from_json(&res).unwrap();
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
fn test_fail_instantiation() {
    let mut deps = mock_dependencies();

    // total weight must equal to 1_000_000
    let msg = InstantiateMsg {
        token: "uusd".to_string(),
        distribution: vec![
            ("addr0000".to_string(), Uint128::from(500_000u128)),
            ("addr0001".to_string(), Uint128::from(500_000u128)),
            ("addr0002".to_string(), Uint128::from(500_000u128)),
            ("addr0003".to_string(), Uint128::from(500_000u128)),
            ("addr0004".to_string(), Uint128::from(500_000u128)),
        ],
    };

    let info = mock_info("addr0000", &[]);
    let err = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap_err();

    assert_eq!(err.to_string(), "Generic error: total weight must equal to 1_000_000".to_string());

    // too many distribution addresses
    let msg = InstantiateMsg {
        token: "uusd".to_string(),
        distribution: vec![
            ("addr0000".to_string(), Uint128::from(100_000u128)),
            ("addr0001".to_string(), Uint128::from(100_000u128)),
            ("addr0002".to_string(), Uint128::from(100_000u128)),
            ("addr0003".to_string(), Uint128::from(100_000u128)),
            ("addr0004".to_string(), Uint128::from(300_000u128)),
            ("addr0005".to_string(), Uint128::from(300_000u128)),
        ],
    };

    let info = mock_info("addr0000", &[]);
    let err = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap_err();

    assert_eq!(err.to_string(), "Generic error: Invalid number of recipients: 6".to_string());

    // distribution cannot be empty
    let msg = InstantiateMsg {
        token: "uusd".to_string(),
        distribution: vec![],
    };

    let info = mock_info("addr0000", &[]);
    let err = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap_err();

    assert_eq!(err.to_string(), "Generic error: Invalid number of recipients: 0".to_string());
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
    let resp: OwnerResponse = from_json(&res).unwrap();
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
    let resp: Config = from_json(&res).unwrap();
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
        token: "uusd".to_string(),
        distribution: vec![
            ("addr0000".to_string(), Uint128::from(500_000u128)),
            ("addr0001".to_string(), Uint128::from(500_000u128)),
        ],
    };
    let info = mock_info("owner", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // query if the token has been added
    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetToken {}).unwrap();

    let token: String = from_json(&res).unwrap();

    assert_eq!(token, "uusd".to_string());
}

#[test]
fn test_distribute_token() {
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
                amount: (100 * 10u128.pow(6)).to_string(),
                denom: env.denoms["reward"].to_string(),
            }],
        },
        &env.signer,
    )
    .unwrap();

    // get balance trader 0
    let balance = env.get_balance(env.traders[0].address(), env.denoms["reward"].to_string());
    assert_eq!(balance, Uint128::zero());

    // get balance trader 1
    let balance = env.get_balance(env.traders[1].address(), env.denoms["reward"].to_string());
    assert_eq!(balance, Uint128::zero());

    // get balance trader 2
    let balance = env.get_balance(env.traders[2].address(), env.denoms["reward"].to_string());
    assert_eq!(balance, Uint128::zero());

    // distribute the tokens
    wasm.execute(distributor.as_str(), &ExecuteMsg::Distribute {}, &[], &env.signer).unwrap();

    // get balance trader 0 after distribution
    let balance = env.get_balance(env.traders[0].address(), env.denoms["reward"].to_string());
    assert_eq!(balance, Uint128::from(50_000_000u128));

    // get balance trader 1 after distribution
    let balance = env.get_balance(env.traders[1].address(), env.denoms["reward"].to_string());
    assert_eq!(balance, Uint128::from(30_000_000u128));

    // get balance trader 2 after distribution
    let balance = env.get_balance(env.traders[2].address(), env.denoms["reward"].to_string());
    assert_eq!(balance, Uint128::from(20_000_000u128));

    // get balance contract
    let balance = env.get_balance(distributor, env.denoms["reward"].to_string());
    assert_eq!(balance, Uint128::zero());
}

#[test]
fn test_fail_distribute_token_zero_funds() {
    let env = StakingEnv::new();

    let wasm = Wasm::new(&env.app);

    let distribution = vec![
        (env.traders[0].address(), Uint128::from(500_000u128)),
        (env.traders[1].address(), Uint128::from(500_000u128)),
    ];

    let distributor =
        env.deploy_distributor_contract(&wasm, "distributor".to_string(), distribution);

    // distribute the tokens
    wasm.execute(distributor.as_str(), &ExecuteMsg::Distribute {}, &[], &env.signer).unwrap();

    // get balance trader 0 after distribution
    let balance = env.get_balance(env.traders[0].address(), env.denoms["reward"].to_string());
    assert_eq!(balance, Uint128::zero());

    // get balance trader 1 after distribution
    let balance = env.get_balance(env.traders[1].address(), env.denoms["reward"].to_string());
    assert_eq!(balance, Uint128::zero());
}

use cosmwasm_std::{coin, Addr, Timestamp, Uint128};
use fee_distribution::staking::{ExecuteMsg, QueryMsg};
use instrumental_testing::staking_env::StakingEnv;
use osmosis_std::types::cosmos::{bank::v1beta1::MsgSend, base::v1beta1::Coin};
use osmosis_test_tube::{Account, Bank, Module, Wasm};

use crate::state::{Config, State, UserStake};

#[test]
fn test_query_config() {
    let env = StakingEnv::new();

    let wasm = Wasm::new(&env.app);

    let staking_address =
        env.deploy_staking_contract(&wasm, "staking".to_string(), env.signer.address());

    let config: Config = wasm.query(&staking_address, &QueryMsg::Config {}).unwrap();
    assert_eq!(
        config,
        Config {
            owner: Addr::unchecked(env.signer.address()),
            fee_collector: Addr::unchecked(env.signer.address()),
            deposit_denom: env.denoms["deposit"].to_string(),
            staked_denom: config.staked_denom.to_string(),
            deposit_decimals: 6u32,
            reward_denom: env.denoms["reward"].to_string(),
            reward_decimals: 6u32,
            tokens_per_interval: 1_000_000u128.into(),
        }
    );
}

#[test]
fn test_query_state() {
    let env = StakingEnv::new();

    let wasm = Wasm::new(&env.app);

    let staking_address =
        env.deploy_staking_contract(&wasm, "staking".to_string(), env.signer.address());

    let state: State = wasm.query(&staking_address, &QueryMsg::State {}).unwrap();
    assert_eq!(
        state,
        State {
            is_open: false,
            last_distribution: Timestamp::from_nanos(env.app.get_block_time_nanos() as u64),
        }
    );
}

#[test]
fn test_query_get_claimable() {
    let env = StakingEnv::new();

    let bank = Bank::new(&env.app);
    let wasm = Wasm::new(&env.app);

    let (staking_address, _) = env.deploy_staking_contracts(&wasm);

    bank.send(
        MsgSend {
            from_address: env.signer.address(),
            to_address: staking_address.clone(),
            amount: [Coin {
                amount: 10u128.to_string(),
                denom: env.denoms["reward"].to_string(),
            }]
            .to_vec(),
        },
        &env.signer,
    )
    .unwrap();

    let amount: Uint128 = wasm
        .query(
            &staking_address,
            &QueryMsg::GetClaimable {
                user: env.traders[0].address(),
            },
        )
        .unwrap();
    assert_eq!(amount, Uint128::zero());

    wasm.execute(&staking_address, &ExecuteMsg::Unpause {}, &[], &env.signer).unwrap();

    let amount_to_stake = 1_000_000u128;
    wasm.execute(
        &staking_address,
        &ExecuteMsg::Stake {},
        &[coin(amount_to_stake, env.denoms["deposit"].to_string())],
        &env.traders[0],
    )
    .unwrap();

    env.app.increase_time(5u64);

    let amount: Uint128 = wasm
        .query(
            &staking_address,
            &QueryMsg::GetClaimable {
                user: env.traders[0].address(),
            },
        )
        .unwrap();
    assert_eq!(amount, Uint128::from(5_000_000u128));
}

#[test]
fn test_query_get_user_staked_amount() {
    let env = StakingEnv::new();

    let bank = Bank::new(&env.app);
    let wasm = Wasm::new(&env.app);

    let (staking_address, collector_address) = env.deploy_staking_contracts(&wasm);

    bank.send(
        MsgSend {
            from_address: env.signer.address(),
            to_address: collector_address,
            amount: [Coin {
                amount: 1_000_000_000u128.to_string(),
                denom: env.denoms["reward"].to_string(),
            }]
            .to_vec(),
        },
        &env.signer,
    )
    .unwrap();

    let amount: UserStake = wasm
        .query(
            &staking_address,
            &QueryMsg::GetUserStakedAmount {
                user: env.traders[0].address(),
            },
        )
        .unwrap();
    assert_eq!(amount, UserStake::default());

    wasm.execute(&staking_address, &ExecuteMsg::Unpause {}, &[], &env.signer).unwrap();

    let amount_to_stake = 1_000_000u128;
    wasm.execute(
        &staking_address,
        &ExecuteMsg::Stake {},
        &[coin(amount_to_stake, env.denoms["deposit"].to_string())],
        &env.traders[0],
    )
    .unwrap();

    env.app.increase_time(5u64);

    let amount: UserStake = wasm
        .query(
            &staking_address,
            &QueryMsg::GetUserStakedAmount {
                user: env.traders[0].address(),
            },
        )
        .unwrap();
    assert_eq!(
        amount,
        UserStake {
            staked_amounts: amount_to_stake.into(),
            previous_cumulative_rewards_per_token: Uint128::zero(),
            claimable_rewards: Uint128::zero(),
            cumulative_rewards: Uint128::zero(),
            average_staked_amounts: Uint128::zero(),
        }
    );

    wasm.execute(
        &staking_address,
        &ExecuteMsg::Claim {
            recipient: None,
        },
        &[],
        &env.traders[0],
    )
    .unwrap();

    let amount: UserStake = wasm
        .query(
            &staking_address,
            &QueryMsg::GetUserStakedAmount {
                user: env.traders[0].address(),
            },
        )
        .unwrap();
    assert_eq!(
        amount,
        UserStake {
            staked_amounts: amount_to_stake.into(),
            previous_cumulative_rewards_per_token: Uint128::from(10_000_000u128),
            claimable_rewards: Uint128::zero(),
            cumulative_rewards: Uint128::from(10_000_000u128),
            average_staked_amounts: Uint128::from(1_000_000u128),
        }
    );
}

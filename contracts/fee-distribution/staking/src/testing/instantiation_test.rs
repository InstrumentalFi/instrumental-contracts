use cosmwasm_std::{Addr, Timestamp};
use fee_distribution::staking::QueryMsg;
use osmosis_test_tube::{Account, Module, Wasm};
use testing::staking_env::StakingEnv;

use crate::state::{Config, State};

#[test]
fn test_instantiation() {
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

    let state: State = wasm.query(&staking_address, &QueryMsg::State {}).unwrap();
    assert_eq!(
        state,
        State {
            is_open: false,
            last_distribution: Timestamp::from_nanos(env.app.get_block_time_nanos() as u64),
        }
    );
}

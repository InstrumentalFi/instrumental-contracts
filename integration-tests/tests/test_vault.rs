mod helpers;
use cosmwasm_std::{Addr, Coin};
use osmosis_test_tube::{Account, Module, Wasm};
use pablo_vault_types::vault::{Config, ExecuteMsg, InstantiateMsg, QueryMsg, State};
use vault::contract::{DAY_IN_SECONDS, TWO_DAYS_IN_SECONDS};

use crate::helpers::osmosis::{assert_err, instantiate_contract, Setup};

const OSMOSIS_VAULT_CONTRACT_NAME: &str = "vault";

#[test]
fn instantiation() {
    let Setup {
        app,
        signer,
    } = Setup::new();

    let wasm = Wasm::new(&app);

    let contract_addr = instantiate_contract(
        &wasm,
        &signer,
        OSMOSIS_VAULT_CONTRACT_NAME,
        &InstantiateMsg {
            token_a: Addr::unchecked("tokena"),
            token_b: Addr::unchecked("tokenb"),
        },
    );

    let config: Config = wasm.query(&contract_addr, &QueryMsg::Config {}).unwrap();

    assert_eq!(
        config,
        Config {
            token_a: Addr::unchecked("tokena"),
            token_b: Addr::unchecked("tokenb"),
            owner: Addr::unchecked(&signer.address()),
            compound_wait_period: DAY_IN_SECONDS,
            harvest_wait_period: DAY_IN_SECONDS,
        }
    );
}

#[test]
fn default_state() {
    let Setup {
        app,
        signer,
    } = Setup::new();

    let wasm = Wasm::new(&app);

    let contract_addr = instantiate_contract(
        &wasm,
        &signer,
        OSMOSIS_VAULT_CONTRACT_NAME,
        &InstantiateMsg {
            token_a: Addr::unchecked("tokena"),
            token_b: Addr::unchecked("tokenb"),
        },
    );

    let state: State = wasm.query(&contract_addr, &QueryMsg::State {}).unwrap();

    assert_eq!(state.last_harvest, state.last_compound)
}

#[test]
fn update_config_not_owner() {
    let Setup {
        app,
        signer,
    } = Setup::new();

    let wasm = Wasm::new(&app);

    let alice = app.init_account(&[Coin::new(10_000_000_000_000, "uosmo")]).unwrap();

    let contract_addr = instantiate_contract(
        &wasm,
        &signer,
        OSMOSIS_VAULT_CONTRACT_NAME,
        &InstantiateMsg {
            token_a: Addr::unchecked("tokena"),
            token_b: Addr::unchecked("tokenb"),
        },
    );

    let res = wasm
        .execute(
            &contract_addr,
            &ExecuteMsg::UpdateConfig {
                compound_wait_period: None,
                harvest_wait_period: None,
            },
            &[],
            &alice,
        )
        .unwrap_err();
    assert_err(res, "Unauthorized");
}

#[test]
fn update_config_with_owner() {
    let Setup {
        app,
        signer,
    } = Setup::new();

    let wasm = Wasm::new(&app);

    let contract_addr = instantiate_contract(
        &wasm,
        &signer,
        OSMOSIS_VAULT_CONTRACT_NAME,
        &InstantiateMsg {
            token_a: Addr::unchecked("tokena"),
            token_b: Addr::unchecked("tokenb"),
        },
    );

    let config_before: Config = wasm.query(&contract_addr, &QueryMsg::Config {}).unwrap();

    assert_eq!(config_before.compound_wait_period, DAY_IN_SECONDS);
    assert_eq!(config_before.harvest_wait_period, DAY_IN_SECONDS);

    wasm.execute(
        &contract_addr,
        &ExecuteMsg::UpdateConfig {
            compound_wait_period: Some(TWO_DAYS_IN_SECONDS.to_string()),
            harvest_wait_period: Some(TWO_DAYS_IN_SECONDS.to_string()),
        },
        &[],
        &signer,
    )
    .unwrap();

    let config_after: Config = wasm.query(&contract_addr, &QueryMsg::Config {}).unwrap();

    assert_eq!(config_after.compound_wait_period, TWO_DAYS_IN_SECONDS);
    assert_eq!(config_after.harvest_wait_period, TWO_DAYS_IN_SECONDS);
}

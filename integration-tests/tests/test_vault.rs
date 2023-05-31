mod helpers;
use cosmwasm_std::{Addr, Coin};
use osmosis_test_tube::{Account, Module, OsmosisTestApp, Wasm};
use pablo_vault_types::vault::{Config, InstantiateMsg, QueryMsg, State};
use vault::contract::DAY_IN_SECONDS;

use crate::helpers::osmosis::instantiate_contract;

const OSMOSIS_VAULT_CONTRACT_NAME: &str = "vault";

#[test]
fn instantiation() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let signer = app.init_account(&[Coin::new(10_000_000_000_000, "uosmo")]).unwrap();

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
            owner: Addr::unchecked(signer.address()),
            compound_wait_period: DAY_IN_SECONDS,
            harvest_wait_period: DAY_IN_SECONDS,
        }
    );
}

#[test]
fn default_state() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let signer = app.init_account(&[Coin::new(10_000_000_000_000, "uosmo")]).unwrap();

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

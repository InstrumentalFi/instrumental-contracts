mod helpers;
use cosmwasm_std::{coin, Addr};
use osmosis_test_tube::{Module, OsmosisTestApp, Wasm};
use pablo_vault_types::vault::{Config, InstantiateMsg, QueryMsg};

use crate::helpers::osmosis::instantiate_contract;

const OSMOSIS_VAULT_CONTRACT_NAME: &str = "vault";

#[test]
fn instantiation() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);

    let signer = app
        .init_account(&[
            coin(1_000_000_000_000, "uosmo"),
            coin(1_000_000_000_000, "umars"),
            coin(1_000_000_000_000, "uatom"),
        ])
        .unwrap();

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
        }
    );
}

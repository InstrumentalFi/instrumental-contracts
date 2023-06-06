mod helpers;
use cosmwasm_std::Addr;
use osmosis_test_tube::{Module, Wasm};
use pablo_vault_types::vault::{Config, InstantiateMsg, QueryMsg};

use crate::helpers::osmosis::{instantiate_contract, Setup};

const OSMOSIS_VAULT_CONTRACT_NAME: &str = "vault-osmosis";

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
            base_token: Addr::unchecked("blah"),
        },
    );

    let config: Config = wasm.query(&contract_addr, &QueryMsg::Config {}).unwrap();

    assert_eq!(
        config,
        Config {
            base_token: Addr::unchecked("blah"),
        }
    );
}

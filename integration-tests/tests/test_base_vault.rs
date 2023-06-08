mod helpers;
use cosmrs::proto::cosmos::bank::v1beta1::QueryBalanceRequest;
use cosmwasm_std::{Addr, Coin};
use osmosis_test_tube::{Account, Bank, Gamm, Module, Wasm};
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

#[test]
fn test_pool_setup() {
    let Setup {
        app,
        signer: _,
    } = Setup::new();

    let gamm = Gamm::new(&app);
    let bank = Bank::new(&app);

    let alice = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();

    let pool_liquidity = vec![Coin::new(1_000, "uatom"), Coin::new(1_000, "uosmo")];
    let pool_id = gamm.create_basic_pool(&pool_liquidity, &alice).unwrap().data.pool_id;

    let pool = gamm.query_pool(pool_id).unwrap();

    let lp_token = pool.total_shares.unwrap().denom;
    assert_eq!(lp_token, "gamm/pool/1".to_string());

    let alice_balance = bank
        .query_balance(&QueryBalanceRequest {
            address: alice.address(),
            denom: lp_token,
        })
        .unwrap()
        .balance
        .unwrap()
        .amount;

    assert_eq!(alice_balance, "100000000000000000000".to_string());
}

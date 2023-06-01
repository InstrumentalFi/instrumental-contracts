mod helpers;
use cosmrs::proto::cosmos::bank::v1beta1::QueryBalanceRequest;
use cosmwasm_std::Coin;
use osmosis_test_tube::{Account, Bank, Gamm, Module, Wasm};
use osmosis_vault::msg::InstantiateMsg;

use crate::helpers::osmosis::{instantiate_contract, Setup};

const OSMOSIS_VAULT_CONTRACT_NAME: &str = "osmosis_vault";

#[test]
fn instantiation() {
    let Setup {
        app,
        signer,
    } = Setup::new();

    let wasm = Wasm::new(&app);
    let gamm = Gamm::new(&app);

    let pool_liquidity = vec![Coin::new(1_000, "uatom"), Coin::new(1_000, "uosmo")];
    let gamm_pool_id = gamm.create_basic_pool(&pool_liquidity, &signer).unwrap().data.pool_id;

    let _contract_addr = instantiate_contract(
        &wasm,
        &signer,
        OSMOSIS_VAULT_CONTRACT_NAME,
        &InstantiateMsg {
            pool_id: gamm_pool_id,
            vault_token_subdenom: "osmosis-vault".to_string(),
        },
    );

    // let config: Config = wasm.query(&contract_addr, &QueryMsg::Config {}).unwrap();

    // assert_eq!(
    //     config,
    //     Config {
    //         base_token: Addr::unchecked("/gamm/pool/1"),
    //     }
    // );
}

#[test]
fn test_pool_setup() {
    let Setup {
        app,
        signer,
    } = Setup::new();

    let gamm = Gamm::new(&app);
    let bank = Bank::new(&app);

    let pool_liquidity = vec![Coin::new(1_000, "uatom"), Coin::new(1_000, "uosmo")];
    let pool_id = gamm.create_basic_pool(&pool_liquidity, &signer).unwrap().data.pool_id;

    let pool = gamm.query_pool(pool_id).unwrap();

    let lp_token = pool.total_shares.unwrap().denom;
    assert_eq!(lp_token, "gamm/pool/1".to_string());

    let signer_balance = bank
        .query_balance(&QueryBalanceRequest {
            address: signer.address(),
            denom: lp_token,
        })
        .unwrap()
        .balance
        .unwrap()
        .amount;

    assert_eq!(signer_balance, "100000000000000000000".to_string());
}

#[test]
fn test_deposit() {
    let Setup {
        app,
        signer,
    } = Setup::new();

    let gamm = Gamm::new(&app);
    let bank = Bank::new(&app);

    let pool_liquidity = vec![Coin::new(1_000, "uatom"), Coin::new(1_000, "uosmo")];
    let pool_id = gamm.create_basic_pool(&pool_liquidity, &signer).unwrap().data.pool_id;

    let pool = gamm.query_pool(pool_id).unwrap();

    let lp_token = pool.total_shares.unwrap().denom;
    assert_eq!(lp_token, "gamm/pool/1".to_string());

    let signer_balance = bank
        .query_balance(&QueryBalanceRequest {
            address: signer.address(),
            denom: lp_token,
        })
        .unwrap()
        .balance
        .unwrap()
        .amount;

    assert_eq!(signer_balance, "100000000000000000000".to_string());
}

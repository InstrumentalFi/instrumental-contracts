mod helpers;
use apollo_cw_asset::{AssetInfo, AssetInfoUnchecked};
use cosmrs::proto::cosmos::bank::v1beta1::QueryBalanceRequest;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Decimal};
use cw_dex::osmosis::OsmosisPool;
use cw_dex_router::{
    msg::ExecuteMsg,
    operations::{SwapOperation, SwapOperationsList},
};
use osmosis_test_tube::{Account, Bank, Gamm, Module, OsmosisTestApp, Wasm};
use osmosis_vault::msg::InstantiateMsg;
use simple_vault::state::ConfigUnchecked;

use crate::helpers::osmosis::{instantiate_contract, Setup};

// Needed as liquidity_helper doesn't expose InstantiateMsg type
#[cw_serde]
pub struct BlankInstantiateMsg {}

const OSMOSIS_VAULT_CONTRACT_NAME: &str = "osmosis_vault";

#[test]
fn instantiation() {
    let app = OsmosisTestApp::new();
    let wasm = Wasm::new(&app);
    let gamm = Gamm::new(&app);

    let signer = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
            Coin::new(1_000_000_000_000, "pica"),
        ])
        .unwrap();

    let admin = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();

    let force_withdraw_admin = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();

    let treasury = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();

    // Set performance fee to 0.125
    let performance_fee = Decimal::permille(125);

    // Base pool uatom / uosmo
    let pool_liquidity = vec![Coin::new(1_000, "uatom"), Coin::new(1_000, "uosmo")];
    let base_pool_id = gamm.create_basic_pool(&pool_liquidity, &signer).unwrap().data.pool_id;

    // Setup reward token as Pica and liquidity pool
    let reward_token_denoms = vec!["pica".to_string()];
    let reward_liquidation_target = "uatom".to_string();
    let reward1_pool_liquidity = vec![Coin::new(1_000, "pica"), Coin::new(1_000, "uatom")];
    let reward1_pool_id =
        gamm.create_basic_pool(&reward1_pool_liquidity, &signer).unwrap().data.pool_id;
    let reward1_pool = OsmosisPool::unchecked(reward1_pool_id);
    let reward1_token = reward1_pool_liquidity
        .iter()
        .find(|x| x.denom != reward_liquidation_target)
        .unwrap()
        .denom
        .clone();

    let reward_assets = reward_token_denoms
        .iter()
        .map(|x| AssetInfoUnchecked::Native(x.clone()))
        .collect::<Vec<_>>();

    let olh_wasm_byte_code =
        std::fs::read("../integration-tests/tests/test-artifacts/osmosis_liquidity_helper.wasm")
            .unwrap();
    let olh_code_id = wasm.store_code(&olh_wasm_byte_code, None, &admin).unwrap().data.code_id;

    let osmosis_liquidity_helper = wasm
        .instantiate(olh_code_id, &BlankInstantiateMsg {}, None, None, &[], &admin)
        .unwrap()
        .data
        .address;

    let cw_dex_wasm_byte_code =
        std::fs::read("../integration-tests/tests/test-artifacts/cw_dex_router_osmosis.wasm")
            .unwrap();
    let cw_dex_code_id =
        wasm.store_code(&cw_dex_wasm_byte_code, None, &admin).unwrap().data.code_id;

    let cw_dex_router = wasm
        .instantiate(cw_dex_code_id, &BlankInstantiateMsg {}, None, None, &[], &admin)
        .unwrap()
        .data
        .address;

    let lh = liquidity_helper::helper::LiquidityHelperBase(osmosis_liquidity_helper);

    let config = ConfigUnchecked {
        force_withdraw_whitelist: vec![force_withdraw_admin.address()],
        performance_fee,
        reward_assets,
        reward_liquidation_target: AssetInfoUnchecked::Native(reward_liquidation_target.clone()),
        treasury: treasury.address(),
        liquidity_helper: lh,
        router: cw_dex_router.clone().into(),
    };

    // Update path on the router
    wasm.execute(
        &cw_dex_router,
        &ExecuteMsg::SetPath {
            offer_asset: AssetInfo::Native(reward1_token.clone()).into(),
            ask_asset: AssetInfo::Native(reward_liquidation_target.clone()).into(),
            path: SwapOperationsList::new(vec![SwapOperation {
                offer_asset_info: AssetInfo::Native(reward1_token),
                ask_asset_info: AssetInfo::Native(reward_liquidation_target),
                pool: cw_dex::Pool::Osmosis(reward1_pool),
            }])
            .into(),
            bidirectional: false,
        },
        &[],
        &admin,
    )
    .unwrap();

    let _contract_addr = instantiate_contract(
        &wasm,
        &signer,
        OSMOSIS_VAULT_CONTRACT_NAME,
        &InstantiateMsg {
            admin: admin.address(),
            pool_id: base_pool_id,
            lockup_duration: 86400u64,
            config,
            vault_token_subdenom: "osmosis-vault".to_string(),
        },
    );
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

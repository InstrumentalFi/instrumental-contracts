#![allow(dead_code)]
//use anyhow::Result as AnyResult;
use cosmwasm_std::Coin;
//use cw_multi_test::AppResponse;
use osmosis_std::types::osmosis::{
    gamm::v1beta1::{MsgSwapExactAmountIn, MsgSwapExactAmountInResponse},
    poolmanager::v1beta1::SwapAmountInRoute,
};
use osmosis_test_tube::{Account, ExecuteResponse, OsmosisTestApp, Runner, SigningAccount};

pub mod osmosis {
    use std::fmt::Display;

    use apollo_cw_asset::AssetInfoBase;
    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::{Addr, Decimal};
    use osmosis_vault::msg::InstantiateMsg;
    use simple_vault::state::ConfigUnchecked;
    const OSMOSIS_VAULT_CONTRACT_NAME: &str = "osmosis_vault";

    // Needed as liquidity_helper doesn't expose InstantiateMsg type
    #[cw_serde]
    pub struct BlankInstantiateMsg {}
    use apollo_cw_asset::{AssetInfo, AssetInfoUnchecked};
    use cosmwasm_std::Coin;
    use cw_dex::{osmosis::OsmosisPool, traits::Pool as PoolTrait};
    use cw_dex_router::{
        msg::ExecuteMsg,
        operations::{SwapOperation, SwapOperationsList},
    };
    use osmosis_test_tube::{
        Account, Gamm, Module, OsmosisTestApp, RunnerError, SigningAccount, Wasm,
    };
    use serde::Serialize;

    pub struct Setup {
        pub app: OsmosisTestApp,
        pub signer: SigningAccount,
        pub admin: SigningAccount,
        pub force_withdraw_admin: SigningAccount,
        pub treasury: SigningAccount,
        pub vault_address: String,
        pub base_token: AssetInfoBase<Addr>,
    }

    impl Setup {
        pub fn new() -> Self {
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
                    Coin::new(1_000_000_000_000, "pica"),
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
            let base_pool_id =
                gamm.create_basic_pool(&pool_liquidity, &signer).unwrap().data.pool_id;

            let base_pool = OsmosisPool::unchecked(base_pool_id);
            let base_token = base_pool.lp_token();

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

            let olh_wasm_byte_code = std::fs::read(
                "../integration-tests/tests/test-artifacts/osmosis_liquidity_helper.wasm",
            )
            .unwrap();
            let olh_code_id =
                wasm.store_code(&olh_wasm_byte_code, None, &admin).unwrap().data.code_id;

            let osmosis_liquidity_helper = wasm
                .instantiate(olh_code_id, &BlankInstantiateMsg {}, None, None, &[], &admin)
                .unwrap()
                .data
                .address;

            let cw_dex_wasm_byte_code = std::fs::read(
                "../integration-tests/tests/test-artifacts/cw_dex_router_osmosis.wasm",
            )
            .unwrap();
            let cw_dex_code_id =
                wasm.store_code(&cw_dex_wasm_byte_code, None, &admin).unwrap().data.code_id;

            let router_address = wasm
                .instantiate(cw_dex_code_id, &BlankInstantiateMsg {}, None, None, &[], &admin)
                .unwrap()
                .data
                .address;

            let lh = liquidity_helper::helper::LiquidityHelperBase(osmosis_liquidity_helper);

            let config = ConfigUnchecked {
                force_withdraw_whitelist: vec![force_withdraw_admin.address()],
                performance_fee,
                reward_assets,
                reward_liquidation_target: AssetInfoUnchecked::Native(
                    reward_liquidation_target.clone(),
                ),
                treasury: treasury.address(),
                liquidity_helper: lh,
                router: router_address.clone().into(),
            };

            // Update path on the router
            wasm.execute(
                &router_address,
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

            let vault_address = instantiate_contract(
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

            Self {
                app,
                admin,
                signer,
                force_withdraw_admin,
                treasury,
                base_token,
                vault_address,
            }
        }
    }

    impl Default for Setup {
        fn default() -> Self {
            Self::new()
        }
    }

    pub fn wasm_file(contract_name: &str) -> String {
        let artifacts_dir =
            std::env::var("ARTIFACTS_DIR_PATH").unwrap_or_else(|_| "artifacts".to_string());
        let snaked_name = contract_name.replace('-', "_");
        format!("../{artifacts_dir}/{snaked_name}.wasm")
    }

    pub fn instantiate_contract<M>(
        wasm: &Wasm<OsmosisTestApp>,
        owner: &SigningAccount,
        contract_name: &str,
        msg: &M,
    ) -> String
    where
        M: ?Sized + Serialize,
    {
        let wasm_byte_code = std::fs::read(wasm_file(contract_name)).unwrap();
        let code_id = wasm.store_code(&wasm_byte_code, None, owner).unwrap().data.code_id;

        wasm.instantiate(
            code_id,
            msg,
            None,
            Some(contract_name),
            &[Coin::new(10_000_000, "uosmo")],
            owner,
        )
        .unwrap()
        .data
        .address
    }

    pub fn assert_err(actual: RunnerError, expected: impl Display) {
        match actual {
            RunnerError::ExecuteError {
                msg,
            } => assert!(msg.contains(&expected.to_string())),
            RunnerError::QueryError {
                msg,
            } => assert!(msg.contains(&expected.to_string())),
            _ => panic!("Unhandled error"),
        }
    }
}

/// Every execution creates new block and block timestamp will +5 secs from last block
/// (see https://github.com/osmosis-labs/osmosis-rust/issues/53#issuecomment-1311451418).
///
/// We need to swap n times to pass twap window size. Every swap moves block 5 sec so
/// n = window_size / 5 sec.
pub fn swap_to_create_twap_records(
    app: &OsmosisTestApp,
    signer: &SigningAccount,
    pool_id: u64,
    coin_in: Coin,
    denom_out: &str,
    window_size: u64,
) {
    let n = window_size / 5u64;
    swap_n_times(app, signer, pool_id, coin_in, denom_out, n);
}

pub fn swap_n_times(
    app: &OsmosisTestApp,
    signer: &SigningAccount,
    pool_id: u64,
    coin_in: Coin,
    denom_out: &str,
    n: u64,
) {
    for _ in 0..n {
        swap(app, signer, pool_id, coin_in.clone(), denom_out);
    }
}

pub fn swap(
    app: &OsmosisTestApp,
    signer: &SigningAccount,
    pool_id: u64,
    coin_in: Coin,
    denom_out: &str,
) -> ExecuteResponse<MsgSwapExactAmountInResponse> {
    app.execute::<_, MsgSwapExactAmountInResponse>(
        MsgSwapExactAmountIn {
            sender: signer.address(),
            routes: vec![SwapAmountInRoute {
                pool_id,
                token_out_denom: denom_out.to_string(),
            }],
            token_in: Some(coin_in.into()),
            token_out_min_amount: "1".to_string(),
        },
        MsgSwapExactAmountIn::TYPE_URL,
        signer,
    )
    .unwrap()
}

// pub fn assert_err(res: AnyResult<AppResponse>, err: ContractError) {
//     match res {
//         Ok(_) => panic!("Result was not an error"),
//         Err(generic_err) => {
//             let contract_err: ContractError = generic_err.downcast().unwrap();
//             assert_eq!(contract_err, err);
//         }
//     }
// }

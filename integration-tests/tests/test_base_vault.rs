mod helpers;
use apollo_cw_asset::AssetInfoBase;
use cosmrs::proto::cosmos::bank::v1beta1::QueryBalanceRequest;
use cosmwasm_std::{Coin, Decimal, Uint128};
use cw_dex::{
    osmosis::{OsmosisPool, OsmosisStaking},
    traits::Pool as PoolTrait,
};
use cw_vault_token::osmosis::OsmosisDenom;
use osmosis_test_tube::{Account, Bank, Module, Wasm};
use osmosis_vault::msg::{ExecuteMsg, QueryMsg};
use simple_vault::msg::{ExtensionQueryMsg, SimpleExtensionQueryMsg, StateResponse};

use crate::helpers::osmosis::Setup;

#[test]
fn instantiation() {
    let Setup {
        app,
        signer: _,
        admin,
        force_withdraw_admin,
        treasury,
        vault_address,
        base_token,
    } = Setup::new();

    let wasm = Wasm::new(&app);

    let state: StateResponse<OsmosisStaking, OsmosisPool, OsmosisDenom> = wasm
        .query(
            &vault_address,
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Simple(SimpleExtensionQueryMsg::State {})),
        )
        .unwrap();

    let vault_token_denom = state.vault_token.to_string();
    let total_staked_base_tokens = state.total_staked_base_tokens;
    let vault_token_supply = state.vault_token_supply;
    let vault_admin = state.admin;
    let config = state.config;
    let pool = state.pool;

    // Check admin address is set correctly
    assert_eq!(vault_admin.unwrap(), admin.address());

    // Check the config of the vault is set correctly
    // Performance fee is 0.125
    assert_eq!(config.performance_fee, Decimal::permille(125));
    // Treasury address is correct
    assert_eq!(config.treasury, treasury.address());
    // Router address is correct
    // assert_eq!(config.router, TODO);
    // Reward asset is set correctly
    assert_eq!(config.reward_assets, vec![AssetInfoBase::Native("pica".to_string())]);
    // Reward liquidation target is set correctly
    assert_eq!(config.reward_liquidation_target, AssetInfoBase::Native("uatom".to_string()));
    // Whitelisted addresses that can call ForceWithdraw and
    // ForceWithdrawUnlocking are set correctly
    assert_eq!(config.force_withdraw_whitelist, vec![force_withdraw_admin.address()]);
    // Liquidity helper address is set correctly
    // assert_eq!(config.liquidity_helper, TODO);

    // Check staked tokens is zero
    assert_eq!(total_staked_base_tokens, Uint128::zero());

    // TODO Check the Staking struct is set correctly

    // Check the Pool struct is set correctly
    assert_eq!(pool.lp_token().to_string(), base_token);

    // Check the vault token is set correctly
    // TODO replace string with regex
    assert_eq!(
        vault_token_denom,
        "factory/osmo17p9rzwnnfxcjp32un9ug7yhhzgtkhvl9jfksztgw5uh69wac2pgs5yczr8/osmosis-vault"
            .to_string()
    );

    // Check vault token supply is zero
    assert_eq!(vault_token_supply, Uint128::zero());
}

#[test]
fn deposit() {
    let Setup {
        app,
        signer,
        admin: _,
        force_withdraw_admin: _,
        treasury: _,
        vault_address,
        base_token,
    } = Setup::new();

    let wasm = Wasm::new(&app);
    let bank = Bank::new(&app);

    let state: StateResponse<OsmosisStaking, OsmosisPool, OsmosisDenom> = wasm
        .query(
            &vault_address,
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Simple(SimpleExtensionQueryMsg::State {})),
        )
        .unwrap();

    let vault_token_denom = state.vault_token.to_string();

    let mut balance = bank
        .query_balance(&QueryBalanceRequest {
            address: signer.address(),
            denom: vault_token_denom.clone(),
        })
        .unwrap()
        .balance
        .unwrap();

    assert_eq!("0".to_string(), balance.amount);

    let deposit_amount = Uint128::new(2);

    let deposit_msg = ExecuteMsg::Deposit {
        amount: deposit_amount,
        recipient: None,
    };
    wasm.execute(
        &vault_address,
        &deposit_msg,
        &[Coin {
            amount: deposit_amount,
            denom: base_token,
        }],
        &signer,
    )
    .unwrap();

    balance = bank
        .query_balance(&QueryBalanceRequest {
            address: signer.address(),
            denom: vault_token_denom,
        })
        .unwrap()
        .balance
        .unwrap();

    assert_eq!("2000000".to_string(), balance.amount);
    assert_eq!(
        "factory/osmo17p9rzwnnfxcjp32un9ug7yhhzgtkhvl9jfksztgw5uh69wac2pgs5yczr8/osmosis-vault"
            .to_string(),
        balance.denom
    );
}

mod helpers;
use apollo_cw_asset::AssetInfoBase;
use cosmwasm_std::{Decimal, Uint128};
use cw_dex::{
    osmosis::{OsmosisPool, OsmosisStaking},
    traits::Pool as PoolTrait,
};
use cw_vault_token::osmosis::OsmosisDenom;
use osmosis_test_tube::{Account, Module, Wasm};
use osmosis_vault::msg::QueryMsg;
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

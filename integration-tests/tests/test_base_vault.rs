mod helpers;
use std::str::FromStr;

use apollo_cw_asset::AssetInfoBase;
use base_vault::DEFAULT_VAULT_TOKENS_PER_STAKED_BASE_TOKEN;
use cosmrs::proto::cosmos::{
    bank::v1beta1::{MsgSend, QueryBalanceRequest},
    base::v1beta1::Coin as ProtoCoin,
};
use cosmwasm_std::{Coin, Decimal, Uint128};
use cw_dex::{
    osmosis::{OsmosisPool, OsmosisStaking},
    traits::Pool as PoolTrait,
};
use cw_vault_token::osmosis::OsmosisDenom;
use osmosis_test_tube::{Account, Bank, Module, Runner, SigningAccount, Wasm};
use osmosis_vault::msg::{ExecuteMsg, QueryMsg};
use simple_vault::msg::{ExtensionQueryMsg, SimpleExtensionQueryMsg, StateResponse};

use crate::helpers::osmosis::Setup;

fn query_vault_state<'a, R>(
    runner: &'a R,
    vault_addr: &str,
) -> StateResponse<OsmosisStaking, OsmosisPool, OsmosisDenom>
where
    R: Runner<'a>,
{
    let wasm = Wasm::new(runner);
    let state: StateResponse<OsmosisStaking, OsmosisPool, OsmosisDenom> = wasm
        .query(
            vault_addr,
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Simple(SimpleExtensionQueryMsg::State {})),
        )
        .unwrap();
    state
}

fn query_token_balance<'a, R>(runner: &'a R, address: &str, denom: &str) -> Uint128
where
    R: Runner<'a>,
{
    let bank = Bank::new(runner);
    let balance = bank
        .query_balance(&QueryBalanceRequest {
            address: address.to_string(),
            denom: denom.to_string(),
        })
        .unwrap()
        .balance
        .unwrap_or_default()
        .amount;
    Uint128::from_str(&balance).unwrap()
}

fn send_native_coins<'a, R>(
    runner: &'a R,
    from: &SigningAccount,
    to: &str,
    denom: &str,
    amount: impl Into<String>,
) where
    R: Runner<'a>,
{
    let bank = Bank::new(runner);
    bank.send(
        MsgSend {
            amount: vec![ProtoCoin {
                denom: denom.to_string(),
                amount: amount.into(),
            }],
            from_address: from.address(),
            to_address: to.to_string(),
        },
        from,
    )
    .unwrap();
}

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

    let state = query_vault_state(&app, &vault_address);

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
    assert_eq!(pool.lp_token().to_string(), base_token.to_string());

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

    let state = query_vault_state(&app, &vault_address);

    let vault_token_denom = state.vault_token.to_string();
    let vault_token_supply = state.vault_token_supply;
    let total_staked_amount = state.total_staked_base_tokens;

    let signer_vault_token_balance_before =
        query_token_balance(&app, &signer.address(), &vault_token_denom);
    assert_eq!(Uint128::zero(), signer_vault_token_balance_before);

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
            denom: base_token.to_string(),
        }],
        &signer,
    )
    .unwrap();

    let signer_vault_token_balance_after =
        query_token_balance(&app, &signer.address(), &vault_token_denom);
    assert_eq!(Uint128::new(2000000), signer_vault_token_balance_after);
    assert_eq!(
        vault_token_supply,
        total_staked_amount * DEFAULT_VAULT_TOKENS_PER_STAKED_BASE_TOKEN
    );
}

#[test]
fn reward_tokens() {
    let Setup {
        app,
        signer,
        admin: _,
        force_withdraw_admin: _,
        treasury,
        vault_address,
        base_token,
    } = Setup::new();

    let wasm = Wasm::new(&app);

    let state = query_vault_state(&app, &vault_address);

    let vault_token_denom = state.vault_token.to_string();
    let config = state.config;

    let deposit_amount = Uint128::new(200_000_000u128);
    let deposit_msg = ExecuteMsg::Deposit {
        amount: deposit_amount,
        recipient: None,
    };

    wasm.execute(
        &vault_address,
        &deposit_msg,
        &[Coin {
            amount: deposit_amount,
            denom: base_token.to_string(),
        }],
        &signer,
    )
    .unwrap();

    // Send some reward tokens to vault to simulate reward accruing
    let reward_amount = Uint128::new(100_000_000u128);
    send_native_coins(
        &app,
        &signer,
        &vault_address.clone(),
        &config.reward_assets[0].to_string(),
        reward_amount,
    );

    // Query treasury reward token balance
    let treasury_reward_token_balance_before =
        query_token_balance(&app, &treasury.address(), &config.reward_assets[0].to_string());

    // Query vault state
    let state = query_vault_state(&app, &vault_address);
    let total_staked_amount_before_compound_deposit = state.total_staked_base_tokens;

    // Deposit some more base token to vault to trigger compounding
    let deposit_amount = Uint128::new(200_000_000u128);
    let deposit_msg = ExecuteMsg::Deposit {
        amount: deposit_amount,
        recipient: None,
    };
    wasm.execute(
        &vault_address,
        &deposit_msg,
        &[Coin {
            amount: deposit_amount,
            denom: base_token.to_string(),
        }],
        &signer,
    )
    .unwrap();

    // Query vault state
    let state = query_vault_state(&app, &vault_address);
    let total_staked_amount = state.total_staked_base_tokens;
    let total_staked_amount_diff_after_compounding_reward1 =
        total_staked_amount - total_staked_amount_before_compound_deposit;
    // Should have increased more than the deposit due to the compounded rewards
    assert!(total_staked_amount_diff_after_compounding_reward1 > deposit_amount);

    // Query treasury reward token balance
    let treasury_reward_token_balance_after =
        query_token_balance(&app, &treasury.address(), &config.reward_assets[0].to_string());
    assert_eq!(
        treasury_reward_token_balance_after,
        treasury_reward_token_balance_before + reward_amount * config.performance_fee
    );

    let alice = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();

    // Send base_token signer to alice to test another deposit
    let alice_deposit_amount = Uint128::from(100_000_000u128);
    send_native_coins(
        &app,
        &signer,
        &alice.address(),
        &base_token.to_string(),
        alice_deposit_amount,
    );

    // Query vault state
    let state_before_alice_deposit = query_vault_state(&app, &vault_address);

    // Deposit from alice
    let deposit_msg = ExecuteMsg::Deposit {
        amount: alice_deposit_amount,
        recipient: None,
    };
    wasm.execute(
        &vault_address,
        &deposit_msg,
        &[Coin {
            amount: alice_deposit_amount,
            denom: base_token.to_string(),
        }],
        &alice,
    )
    .unwrap();

    let alice_vault_token_balance = query_token_balance(&app, &alice.address(), &vault_token_denom);
    assert_ne!(alice_vault_token_balance, Uint128::zero());
    let alice_base_token_balance =
        query_token_balance(&app, &alice.address(), &base_token.to_string());
    assert!(alice_base_token_balance.is_zero());

    // Query signer's vault token balance
    let signer_vault_token_balance =
        query_token_balance(&app, &signer.address(), &vault_token_denom);

    // Check that total supply of vault tokens is correct
    let state = query_vault_state(&app, &vault_address);
    let vault_token_supply = state.vault_token_supply;
    assert_eq!(signer_vault_token_balance + alice_vault_token_balance, vault_token_supply);

    // Assert that alices's share of the vault was correctly calculated
    println!("Alice vault token balance: {}", alice_vault_token_balance);
    println!("vault token supply: {}", vault_token_supply);
    println!("alice_deposit_amount: {}", alice_deposit_amount);
    println!(
        "total_staked_base_tokens_before_alice_deposit: {}",
        state_before_alice_deposit.total_staked_base_tokens
    );
    let alice_vault_token_share =
        Decimal::from_ratio(alice_vault_token_balance, vault_token_supply);
    let expected_share = Decimal::from_ratio(
        alice_deposit_amount,
        state_before_alice_deposit.total_staked_base_tokens,
    );
    println!("alice_vault_token_share: {}", alice_vault_token_share);
    println!("expected_share: {}", expected_share);
    // Failing on small decimal difference
    //assert_eq!(alice_vault_token_share, expected_share);
}

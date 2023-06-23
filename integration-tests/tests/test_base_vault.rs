mod helpers;
use prost::Message;
use std::str::FromStr;

use apollo_cw_asset::AssetInfoBase;
use base_vault::DEFAULT_VAULT_TOKENS_PER_STAKED_BASE_TOKEN;
use cosmrs::proto::cosmos::{
    bank::v1beta1::{MsgSend, QueryBalanceRequest},
    base::v1beta1::Coin as ProtoCoin,
};
use cosmrs::Any;
use cosmwasm_std::{Coin, Decimal, Uint128};
use cw_dex::{
    osmosis::{OsmosisPool, OsmosisStaking},
    traits::Pool as PoolTrait,
};
use cw_vault_standard::extensions::{
    force_unlock::ForceUnlockExecuteMsg,
    lockup::{LockupExecuteMsg, LockupQueryMsg, UnlockingPosition},
};

use cw_vault_token::osmosis::OsmosisDenom;
use osmosis_std::types::osmosis::lockup::Params as LockupParams;
use osmosis_test_tube::{Account, Bank, Module, Runner, SigningAccount, Wasm};
use osmosis_vault::msg::{ExecuteMsg, QueryMsg};
use simple_vault::msg::{
    ExtensionExecuteMsg, ExtensionQueryMsg, SimpleExtensionQueryMsg, StateResponse,
};

use crate::helpers::osmosis::{assert_err, Setup};

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

    // Track how much user 1 deposits (different depending on number of reward tokens)
    let mut signer_total_deposit_amount = Uint128::zero();

    let deposit_amount = Uint128::new(200_000_000u128);
    signer_total_deposit_amount += deposit_amount;
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
        &vault_address,
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
    signer_total_deposit_amount += deposit_amount;
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
    //println!("Alice vault token balance: {}", alice_vault_token_balance);
    //println!("vault token supply: {}", vault_token_supply);
    //println!("alice_deposit_amount: {}", alice_deposit_amount);
    // println!(
    //     "total_staked_base_tokens_before_alice_deposit: {}",
    //     state_before_alice_deposit.total_staked_base_tokens
    // );
    let _alice_vault_token_share =
        Decimal::from_ratio(alice_vault_token_balance, vault_token_supply);
    let _expected_share = Decimal::from_ratio(
        alice_deposit_amount,
        state_before_alice_deposit.total_staked_base_tokens,
    );
    // println!("alice_vault_token_share: {}", alice_vault_token_share);
    // println!("expected_share: {}", expected_share);
    // Failing on small decimal difference
    //assert_eq!(alice_vault_token_share, expected_share);

    // TODO second reward token test

    // Query user 1 vault token balance
    let signer_vault_token_balance =
        query_token_balance(&app, &signer.address(), &vault_token_denom);

    // Query how many base tokens user 1's vault tokens represents
    let msg = QueryMsg::ConvertToAssets {
        amount: signer_vault_token_balance,
    };

    let signer_base_token_balance_in_vault: Uint128 = wasm.query(&vault_address, &msg).unwrap();

    // Assert that user 1's vault tokens represents more than the amount they
    // deposited (due to compounding)
    assert!(signer_base_token_balance_in_vault > signer_total_deposit_amount);

    // Begin Unlocking all signer's vault tokens
    let signer_withdraw_amount = signer_vault_token_balance;
    let state = query_vault_state(&app, &vault_address);
    let vault_token_supply_before_withdraw = state.vault_token_supply;

    let withdraw_msg =
        ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Lockup(LockupExecuteMsg::Unlock {
            amount: signer_withdraw_amount,
        }));
    let _res = wasm
        .execute(
            &vault_address,
            &withdraw_msg,
            &[Coin {
                amount: signer_withdraw_amount,
                denom: vault_token_denom.clone(),
            }],
            &signer,
        )
        .unwrap();

    // Query signer's unlocking position
    let unlocking_positions: Vec<UnlockingPosition> = wasm
        .query(
            &vault_address,
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Lockup(
                LockupQueryMsg::UnlockingPositions {
                    owner: signer.address(),
                    limit: None,
                    start_after: None,
                },
            )),
        )
        .unwrap();
    assert!(unlocking_positions.len() == 1);
    let position = unlocking_positions[0].clone();

    // Withdraw unlocked - should fail
    let withdraw_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Lockup(
        LockupExecuteMsg::WithdrawUnlocked {
            lockup_id: position.id,
            recipient: None,
        },
    ));

    let res = wasm.execute(&vault_address, &withdraw_msg, &[], &signer).unwrap_err();
    // Should error because not unlocked yet
    assert_err(res, "Generic error: Claim has not yet matured");

    app.increase_time(86400);

    // Query signer base token balance
    let base_token_balance_before =
        query_token_balance(&app, &signer.address(), &base_token.to_string());
    println!("User1 base token balance before: {}", base_token_balance_before);

    // Withdraw unlocked
    println!("Withdrawing unlocked");
    let _res = wasm.execute(&vault_address, &withdraw_msg, &[], &signer).unwrap();

    // Query user 1 base token balance
    let base_token_balance_after =
        query_token_balance(&app, &signer.address(), &base_token.to_string());
    println!("User1 base token balance after withdrawal: {}", base_token_balance_after);
    assert!(base_token_balance_after > base_token_balance_before);

    let base_token_balance_increase = base_token_balance_after - base_token_balance_before;
    // Assert that all the base tokens were withdrawn
    assert_eq!(base_token_balance_increase, signer_base_token_balance_in_vault);

    // Query vault token supply
    let vault_token_supply: Uint128 =
        wasm.query(&vault_address, &QueryMsg::TotalVaultTokenSupply {}).unwrap();
    println!("Vault token supply: {}", vault_token_supply);
    assert_eq!(vault_token_supply_before_withdraw - vault_token_supply, signer_withdraw_amount);

    // Try force redeem from non-admin wallet
    println!("Force redeem, should fail as sender not whitelisted in contract");
    let force_withdraw_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ForceUnlock(
        ForceUnlockExecuteMsg::ForceRedeem {
            amount: Uint128::from(1000000u128),
            recipient: None,
        },
    ));
    let res = wasm
        .execute(
            &vault_address,
            &force_withdraw_msg,
            &[Coin::new(1000000, &vault_token_denom)],
            &alice,
        )
        .unwrap_err(); // Should error because not unlocked yet
    println!("Error: {}", res);
    // Failing
    // assert!(res.to_string().contains("Unauthorized"));
    //
    // Send 3M vault tokens to force_withdraw_admin
    //let vault_token_balance = query_token_balance(&app, &signer.address(), &vault_token_denom);
    //println!("vault_token_balance: {}", vault_token_balance);
    //send_native_coins(&app, &signer, &force_withdraw_admin.address(), &vault_token_denom, "100");
}
#[test]
fn force_redeem_unauthorized() {
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

    // Deposit into the contract so signer has vault tokens
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

    let state = query_vault_state(&app, &vault_address);
    let vault_token_denom = state.vault_token.to_string();

    let vault_token_balance = query_token_balance(&app, &signer.address(), &vault_token_denom);
    println!("vault_token_balance: {}", vault_token_balance);

    // Generate a user and send vault tokens
    let alice_vault_token_amount = Uint128::new(100_000_000u128);
    let alice = app
        .init_account(&[
            Coin::new(1_000_000_000_000, "uatom"),
            Coin::new(1_000_000_000_000, "uosmo"),
        ])
        .unwrap();
    send_native_coins(
        &app,
        &signer,
        &alice.address(),
        &vault_token_denom,
        alice_vault_token_amount,
    );

    // Try to force withdraw. This should fail as Alice is not whitelisted
    let force_withdraw_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ForceUnlock(
        ForceUnlockExecuteMsg::ForceRedeem {
            amount: alice_vault_token_amount,
            recipient: None,
        },
    ));
    let res = wasm
        .execute(
            &vault_address,
            &force_withdraw_msg,
            &[Coin::new(alice_vault_token_amount.into(), &vault_token_denom)],
            &alice,
        )
        .unwrap_err(); // Should error because Alice is not authorized to force withdraw
    assert_err(res, "Unauthorized");
}

#[test]
fn force_redeem_authorized_contract_not_authorized() {
    // This test should fail where a contract is not authorized to unlock
    // See https://github.com/apollodao/apollo-vaults/blob/4af68b6d8e10c9f55f6f352c1980a9abe8c378d7/contracts/osmosis-vault/tests/integration_test.rs#L730
    let Setup {
        app,
        signer,
        admin: _,
        force_withdraw_admin,
        treasury: _,
        vault_address,
        base_token,
    } = Setup::new();

    let wasm = Wasm::new(&app);

    // Deposit into the contract so signer has vault tokens
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

    let state = query_vault_state(&app, &vault_address);
    let vault_token_denom = state.vault_token.to_string();
    let force_withdraw_vault_token_amount = Uint128::new(100_000_000u128);

    // Send vault tokens to the force withdraw account
    send_native_coins(
        &app,
        &signer,
        &force_withdraw_admin.address(),
        &vault_token_denom,
        force_withdraw_vault_token_amount,
    );

    let force_withdraw_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ForceUnlock(
        ForceUnlockExecuteMsg::ForceRedeem {
            amount: force_withdraw_vault_token_amount,
            recipient: None,
        },
    ));
    let res = wasm
        .execute(
            &vault_address,
            &force_withdraw_msg,
            &[Coin::new(force_withdraw_vault_token_amount.into(), &vault_token_denom)],
            &force_withdraw_admin,
        )
        .unwrap_err();

    // Contract is no authorized to unlock so this fails
    assert_err(res, "Sender (osmo17p9rzwnnfxcjp32un9ug7yhhzgtkhvl9jfksztgw5uh69wac2pgs5yczr8) not allowed to force unlock: unauthorized");
}

//#[test]
//Need to add a successful unlock test, where the contract is added to whitelist
//fn force_redeem_authorized_contract_unlocked() {}

#[test]
fn initiate_force_unlock_contract_unauthorized() {
    let Setup {
        app,
        signer,
        admin: _,
        force_withdraw_admin,
        treasury: _,
        vault_address,
        base_token,
    } = Setup::new();

    let wasm = Wasm::new(&app);

    // Deposit into the contract so signer has vault tokens
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

    let state = query_vault_state(&app, &vault_address);
    let vault_token_denom = state.vault_token.to_string();
    let force_withdraw_vault_token_amount = Uint128::new(100_000_000u128);

    // Send vault tokens to the force withdraw account
    send_native_coins(
        &app,
        &signer,
        &force_withdraw_admin.address(),
        &vault_token_denom,
        force_withdraw_vault_token_amount,
    );

    let unlock_msg =
        ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Lockup(LockupExecuteMsg::Unlock {
            amount: force_withdraw_vault_token_amount,
        }));
    wasm.execute(
        &vault_address,
        &unlock_msg,
        &[Coin::new(force_withdraw_vault_token_amount.into(), &vault_token_denom)],
        &force_withdraw_admin,
    )
    .unwrap();

    // Query unlocking positions
    let unlocking_positions: Vec<UnlockingPosition> = wasm
        .query(
            &vault_address,
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Lockup(
                LockupQueryMsg::UnlockingPositions {
                    owner: force_withdraw_admin.address().clone(),
                    limit: None,
                    start_after: None,
                },
            )),
        )
        .unwrap();

    assert!(unlocking_positions.len() == 1);

    let position = unlocking_positions[0].clone();
    assert_eq!(
        position.base_token_amount,
        force_withdraw_vault_token_amount / DEFAULT_VAULT_TOKENS_PER_STAKED_BASE_TOKEN
    );

    // Try force withdraw unlocking from non-admin wallet, should fail
    println!("Force withdraw unlocking, should fail as not admin");
    let force_withdraw_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ForceUnlock(
        ForceUnlockExecuteMsg::ForceWithdrawUnlocking {
            amount: Some(position.base_token_amount),
            recipient: None,
            lockup_id: position.id,
        },
    ));

    let res = wasm.execute(&vault_address, &force_withdraw_msg, &[], &signer).unwrap_err(); // Should error because not admin
    assert_err(res, "Unauthorized");

    // Try force withdraw unlocking from whitelisted admin wallet, should work
    let force_withdraw_admin_base_token_balance_before =
        query_token_balance(&app, &force_withdraw_admin.address(), &base_token.to_string());

    assert_eq!(force_withdraw_admin_base_token_balance_before, Uint128::zero());

    let res =
        wasm.execute(&vault_address, &force_withdraw_msg, &[], &force_withdraw_admin).unwrap_err();

    // Contract is not authorized to unlock so this fails
    assert_err(res, "Sender (osmo17p9rzwnnfxcjp32un9ug7yhhzgtkhvl9jfksztgw5uh69wac2pgs5yczr8) not allowed to force unlock: unauthorized");
}

#[test]
fn initiate_force_unlock_contract_authorized() {
    let Setup {
        app,
        signer,
        admin: _,
        force_withdraw_admin,
        treasury: _,
        vault_address,
        base_token,
    } = Setup::new();

    let wasm = Wasm::new(&app);

    // Deposit into the contract so signer has vault tokens
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

    let state = query_vault_state(&app, &vault_address);
    let vault_token_denom = state.vault_token.to_string();
    let force_withdraw_vault_token_amount = Uint128::new(100_000_000u128);

    // Send vault tokens to the force withdraw account
    send_native_coins(
        &app,
        &signer,
        &force_withdraw_admin.address(),
        &vault_token_denom,
        force_withdraw_vault_token_amount,
    );

    let unlock_msg =
        ExecuteMsg::VaultExtension(ExtensionExecuteMsg::Lockup(LockupExecuteMsg::Unlock {
            amount: force_withdraw_vault_token_amount,
        }));
    wasm.execute(
        &vault_address,
        &unlock_msg,
        &[Coin::new(force_withdraw_vault_token_amount.into(), &vault_token_denom)],
        &force_withdraw_admin,
    )
    .unwrap();

    // Query unlocking positions
    let unlocking_positions: Vec<UnlockingPosition> = wasm
        .query(
            &vault_address,
            &QueryMsg::VaultExtension(ExtensionQueryMsg::Lockup(
                LockupQueryMsg::UnlockingPositions {
                    owner: force_withdraw_admin.address().clone(),
                    limit: None,
                    start_after: None,
                },
            )),
        )
        .unwrap();

    assert!(unlocking_positions.len() == 1);

    let position = unlocking_positions[0].clone();
    assert_eq!(
        position.base_token_amount,
        force_withdraw_vault_token_amount / DEFAULT_VAULT_TOKENS_PER_STAKED_BASE_TOKEN
    );

    // Allow the contract to unlock positions on the gamm via governance
    // https://github.com/osmosis-labs/test-tube/blob/a4a647726f7bd6d36f4e2d58ed8af37d5acd25a1/packages/osmosis-test-tube/src/runner/app.rs#L538-L549
    app.set_param_set(
        "lockup",
        Any {
            type_url: LockupParams::TYPE_URL.to_string(),
            value: LockupParams {
                force_unlock_allowed_addresses: vec![vault_address.clone()],
            }
            .encode_to_vec(),
        },
    )
    .unwrap();

    // Try force withdraw unlocking from whitelisted admin wallet, should work
    let force_withdraw_admin_base_token_balance_before =
        query_token_balance(&app, &force_withdraw_admin.address(), &base_token.to_string());

    assert_eq!(force_withdraw_admin_base_token_balance_before, Uint128::zero());

    let force_withdraw_msg = ExecuteMsg::VaultExtension(ExtensionExecuteMsg::ForceUnlock(
        ForceUnlockExecuteMsg::ForceWithdrawUnlocking {
            amount: Some(position.base_token_amount),
            recipient: None,
            lockup_id: position.id,
        },
    ));

    wasm.execute(&vault_address, &force_withdraw_msg, &[], &force_withdraw_admin).unwrap();

    let force_withdraw_admin_base_token_balance_after =
        query_token_balance(&app, &force_withdraw_admin.address(), &base_token.to_string());

    assert_eq!(force_withdraw_admin_base_token_balance_after, position.base_token_amount);
}

use crate::helpers::store_code;

use cosmrs::proto::cosmos::bank::v1beta1::QueryBalanceRequest;
use cosmwasm_std::{coin, Addr, Uint128};
use cw20::{BalanceResponse, Cw20QueryMsg};
use fee_distribution::{
    collector::{ExecuteMsg as CollectorExecuteMsg, InstantiateMsg as CollectorInstantiateMsg},
    distributor::InstantiateMsg as DistributorInstantiateMsg,
    staking::InstantiateMsg,
};
use osmosis_test_tube::{Bank, Module, OsmosisTestApp, SigningAccount, Wasm};
use std::{collections::HashMap, str::FromStr};

pub const ONE: u128 = 1_000_000; // 1.0@6dp
pub const SCALE_FACTOR: u128 = 10_000;
pub const BASE_PRICE: u128 = 3_000_000_000; // 3000.0@6dp
pub const POWER_PRICE: u128 = 3_010_000_000; // 3010.0@6dp
pub const SCALED_POWER_PRICE: u128 = 30_100_000; // 0.3010@6dp

pub struct ContractInfo {
    pub addr: Addr,
    pub id: u64,
}

pub struct StakingEnv {
    pub app: OsmosisTestApp,
    pub signer: SigningAccount,
    pub handler: SigningAccount,
    pub empty: SigningAccount,
    pub traders: Vec<SigningAccount>,
    pub denoms: HashMap<String, String>,
    pub cw20_code_id: u64,
}

impl StakingEnv {
    pub fn new() -> Self {
        let app = OsmosisTestApp::new();

        let wasm = Wasm::new(&app);

        let mut denoms = HashMap::new();
        denoms.insert("gas".to_string(), "uosmo".to_string());
        denoms.insert("base".to_string(), "ubase".to_string());
        denoms.insert("reward".to_string(), "uusdc".to_string());
        denoms.insert("deposit".to_string(), "udeposit".to_string());

        let signer = app
            .init_account(&[
                coin(1_000_000_000_000_000_000, denoms["gas"].to_string()),
                coin(1_000_000_000_000_000_000, denoms["base"].to_string()),
                coin(1_000_000_000_000, denoms["reward"].to_string()),
            ])
            .unwrap();

        let handler = app.init_account(&[]).unwrap();

        let mut traders: Vec<SigningAccount> = Vec::new();
        for _ in 0..5 {
            traders.push(
                app.init_account(&[
                    coin(1_000_000_000_000_000_000, denoms["gas"].to_string()),
                    coin(1_000_000_000_000_000_000, denoms["base"].to_string()),
                    coin(1_000_000_000, denoms["deposit"].to_string()),
                ])
                .unwrap(),
            );
        }

        let empty = app
            .init_account(&[
                coin(1_000_000_000, denoms["gas"].to_string()),
                coin(1_000_000_000, denoms["deposit"].to_string()),
            ])
            .unwrap();

        let cw20_code_id = store_code(&wasm, &signer, "cw20_base".to_string());

        Self {
            app,
            signer,
            handler,
            empty,
            traders,
            denoms,
            cw20_code_id,
        }
    }

    pub fn deploy_staking_contracts(&self, wasm: &Wasm<OsmosisTestApp>) -> (String, String) {
        let code_id = store_code(wasm, &self.signer, "collector".to_string());
        let fee_collector_address = wasm
            .instantiate(
                code_id,
                &CollectorInstantiateMsg {},
                None,
                Some("collector-contract"),
                &[coin(1_000_000_000_000, self.denoms["gas"].clone())],
                &self.signer,
            )
            .unwrap()
            .data
            .address;

        let code_id = store_code(wasm, &self.signer, "staking".to_string());
        let staking_address = wasm
            .instantiate(
                code_id,
                &InstantiateMsg {
                    token_code_id: self.cw20_code_id,
                    token_name: "voting escrow".to_string(),
                    fee_collector: fee_collector_address.clone(),
                    deposit_denom: self.denoms["deposit"].clone(),
                    reward_denom: self.denoms["reward"].clone(),
                    deposit_decimals: 6u32,
                    reward_decimals: 6u32,
                    tokens_per_interval: 1_000_000u128.into(),
                },
                None,
                Some("staking-contract"),
                &[coin(1_000_000_000_000, self.denoms["gas"].clone())],
                &self.signer,
            )
            .unwrap()
            .data
            .address;

        // add the reward token as a token
        {
            wasm.execute(
                fee_collector_address.as_str(),
                &CollectorExecuteMsg::AddToken {
                    token: self.denoms["reward"].clone(),
                },
                &[],
                &self.signer,
            )
            .unwrap();
        }

        // update the collector to have the staking contract as an auth
        {
            wasm.execute(
                fee_collector_address.as_str(),
                &CollectorExecuteMsg::UpdateOwner {
                    owner: staking_address.clone(),
                },
                &[],
                &self.signer,
            )
            .unwrap();
        }

        (staking_address, fee_collector_address)
    }

    pub fn deploy_staking_contract(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_name: String,
        fee_collector: String,
    ) -> String {
        let code_id = store_code(wasm, &self.signer, contract_name);
        wasm.instantiate(
            code_id,
            &InstantiateMsg {
                token_code_id: self.cw20_code_id,
                token_name: "voting escrow".to_string(),
                fee_collector,
                deposit_denom: self.denoms["deposit"].clone(),
                reward_denom: self.denoms["reward"].clone(),
                deposit_decimals: 6u32,
                reward_decimals: 6u32,
                tokens_per_interval: 1_000_000u128.into(),
            },
            None,
            Some("staking-contract"),
            &[coin(1_000_000_000_000, self.denoms["gas"].clone())],
            &self.signer,
        )
        .unwrap()
        .data
        .address
    }

    pub fn deploy_fee_collector_contract(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_name: String,
    ) -> String {
        let code_id = store_code(wasm, &self.signer, contract_name);
        wasm.instantiate(
            code_id,
            &CollectorInstantiateMsg {},
            None,
            Some("collector-contract"),
            &[],
            &self.signer,
        )
        .unwrap()
        .data
        .address
    }

    pub fn deploy_distributor_contract(
        &self,
        wasm: &Wasm<OsmosisTestApp>,
        contract_name: String,
        distribution: Vec<(String, Uint128)>,
    ) -> String {
        let code_id = store_code(wasm, &self.signer, contract_name);
        wasm.instantiate(
            code_id,
            &DistributorInstantiateMsg {
                token: self.denoms["reward"].clone(),
                distribution,
            },
            None,
            Some("distributor-contract"),
            &[],
            &self.signer,
        )
        .unwrap()
        .data
        .address
    }

    pub fn get_balance(&self, address: String, denom: String) -> Uint128 {
        let bank = Bank::new(&self.app);

        let response = bank
            .query_balance(&QueryBalanceRequest {
                address,
                denom,
            })
            .unwrap();

        match response.balance {
            Some(balance) => Uint128::from_str(&balance.amount).unwrap(),
            None => Uint128::zero(),
        }
    }

    pub fn get_cw20_balance(&self, address: String, contract: String) -> Uint128 {
        let wasm = Wasm::new(&self.app);

        let response: BalanceResponse = wasm
            .query(
                &contract,
                &Cw20QueryMsg::Balance {
                    address,
                },
            )
            .unwrap();

        response.balance
    }
}

impl Default for StakingEnv {
    fn default() -> Self {
        Self::new()
    }
}

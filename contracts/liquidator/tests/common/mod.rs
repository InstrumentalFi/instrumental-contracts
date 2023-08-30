#![allow(dead_code)]
use std::path::PathBuf;

use cosmwasm_std::Coin;
use liquidator::msg::InstantiateMsg;
use osmosis_test_tube::{Account, Gamm, Module, OsmosisTestApp, SigningAccount, Wasm};

pub struct TestEnv {
    pub app: OsmosisTestApp,
    pub contract_address: String,
    pub owner: SigningAccount,
}

impl TestEnv {
    pub fn new() -> Self {
        let app = OsmosisTestApp::new();
        let gamm = Gamm::new(&app);
        let wasm = Wasm::new(&app);

        // setup owner account
        let initial_balance = [
            Coin::new(1_000_000_000_000, "uosmo"),
            Coin::new(1_000_000_000_000, "uion"),
            Coin::new(1_000_000_000_000, "stake"),
        ];
        let owner = app.init_account(&initial_balance).unwrap();

        // create pools
        gamm.create_basic_pool(
            &[Coin::new(100_000_000, "uion"), Coin::new(100_000_000, "uosmo")],
            &owner,
        )
        .unwrap();
        gamm.create_basic_pool(
            &[Coin::new(100_000_000, "stake"), Coin::new(100_000_000, "uosmo")],
            &owner,
        )
        .unwrap();
        gamm.create_basic_pool(
            &[Coin::new(100_000_000, "stake"), Coin::new(100_000_000, "uion")],
            &owner,
        )
        .unwrap();

        let code_id = wasm.store_code(&get_wasm(), None, &owner).unwrap().data.code_id;

        let contract_address = wasm
            .instantiate(
                code_id,
                &InstantiateMsg {
                    ibc_channel_id: "channel-10".to_string(),
                    ibc_to_address: "neutron1yrg6daqkxyeqye4aac09stzvvwppqwlsk2jn2k".to_string(),
                    liquidation_target: "uosmo".to_string(),
                    owner: owner.address(),
                },
                Some(&owner.address()),
                None,
                &[],
                &owner,
            )
            .unwrap()
            .data
            .address;

        TestEnv {
            app,
            contract_address,
            owner,
        }
    }
}

pub fn get_wasm() -> Vec<u8> {
    let wasm_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("target")
        .join("wasm32-unknown-unknown")
        .join("release")
        .join("liquidator.wasm");
    std::fs::read(wasm_path).unwrap()
}

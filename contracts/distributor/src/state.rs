use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const CONFIG: Item<Config> = Item::new("config");
pub const TOKEN: Item<String> = Item::new("token");
pub const RECIPIENT_LIMIT: usize = 5usize;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub distribution: Vec<(Addr, Uint128)>,
}

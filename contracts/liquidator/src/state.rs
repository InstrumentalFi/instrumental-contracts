use cosmwasm_std::Addr;
use cw_controllers::Admin;
use cw_storage_plus::{Item, Map};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const CONFIG: Item<Config> = Item::new("config");
pub const OWNER: Admin = Admin::new("owner");
pub const ROUTING_TABLE: Map<(&str, &str), Vec<SwapAmountInRoute>> = Map::new("routing_table");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub ibc_channel_id: String,
    pub ibc_to_address: Addr,
    pub liquidation_target: String,
}

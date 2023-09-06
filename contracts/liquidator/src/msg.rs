use std::collections::HashMap;

use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: String,
    pub ibc_channel_id: String,
    pub ibc_to_address: String,
    pub liquidation_target: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateOwner {
        owner: String,
    },
    UpdateConfig {
        ibc_channel_id: String,
        ibc_to_address: String,
        liquidation_target: String,
    },
    SetRoute {
        input_denom: String,
        output_denom: String,
        pool_route: Vec<SwapAmountInRoute>,
    },
    RemoveRoute {
        input_denom: String,
        output_denom: String,
    },
    Liquidate {},
    IbcTransfer {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetOwner {},
    GetConfig {},
    GetRoute {
        input_denom: String,
        output_denom: String,
    },
    GetAllRoutes {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetOwnerResponse {
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct GetRouteResponse {
    pub pool_route: Vec<SwapAmountInRoute>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct GetAllRoutesResponse {
    pub routes: HashMap<String, Vec<SwapAmountInRoute>>,
}

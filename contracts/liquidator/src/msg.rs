use cosmwasm_schema::{cw_serde, QueryResponses};
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub ibc_channel_id: String,
    pub ibc_to_address: String,
    pub liquidation_target: String,
}

#[cw_serde]
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
    Liquidate {},
    IbcTransfer {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetOwnerResponse)]
    GetOwner {},
    #[returns(Config)]
    GetConfig {},
    #[returns(GetRouteResponse)]
    GetRoute {
        input_denom: String,
        output_denom: String,
    },
}

#[cw_serde]
pub struct GetOwnerResponse {
    pub owner: String,
}

#[cw_serde]
pub struct GetRouteResponse {
    pub pool_route: Vec<SwapAmountInRoute>,
}

// use cosmwasm_schema::{cw_serde, QueryResponses};
// use cosmwasm_std::Addr;

// #[cw_serde]
// pub struct Config {
//     pub token_a: Addr,
//     pub token_b: Addr,
// }

// #[cw_serde]
// pub struct InstantiateMsg {
//     pub token_a: Addr,
//     pub token_b: Addr,
// }

// #[cw_serde]
// pub enum ExecuteMsg {}

// #[derive(QueryResponses)]
// #[cw_serde(rename_all = "snake_case")]
// pub enum QueryMsg {
//     #[returns(ConfigResponse)]
//     Config {},
// }

// #[cw_serde]
// pub struct ConfigResponse {
//     pub token_a: Addr,
//     pub token_b: Addr,
// }

use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Addr,
};
use pablo_vault_types::vault::{ConfigResponse, InstantiateMsg, QueryMsg};
use vault::contract::instantiate;

use crate::helpers::th_query;

mod helpers;

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies();

    instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("deployer", &[]),
        InstantiateMsg {
            token_a: Addr::unchecked("tokena"),
            token_b: Addr::unchecked("tokenb"),
        },
    )
    .unwrap();

    let config: ConfigResponse = th_query(deps.as_ref(), QueryMsg::Config {});
    assert_eq!(config.token_a, Addr::unchecked("tokena"));
    assert_eq!(config.token_b, Addr::unchecked("tokenb"));
}

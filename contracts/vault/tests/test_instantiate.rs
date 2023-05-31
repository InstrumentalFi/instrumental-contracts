use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info},
    Addr,
};
use pablo_vault_types::vault::{ConfigResponse, InstantiateMsg, QueryMsg};
use vault::{contract::instantiate, error::ContractError};

use crate::helpers::th_query;

mod helpers;

#[test]
fn invalid_token_pair() {
    let mut deps = mock_dependencies();

    let err = instantiate(
        deps.as_mut(),
        mock_env(),
        mock_info("deployer", &[]),
        InstantiateMsg {
            token_a: Addr::unchecked("tokena"),
            token_b: Addr::unchecked("tokena"),
        },
    )
    .unwrap_err();
    assert!(
        matches!(err, ContractError::Std { .. }),
        "Expected ContractError::Std, received {}",
        err
    );
}

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
    let one_day: u64 = 86400;
    assert_eq!(config.token_a, Addr::unchecked("tokena"));
    assert_eq!(config.token_b, Addr::unchecked("tokenb"));
    assert_eq!(config.owner, Addr::unchecked("deployer"));
    assert_eq!(config.harvest_wait_period, one_day);
    assert_eq!(config.compound_wait_period, one_day);
}

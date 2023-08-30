mod common;
use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env, mock_info},
    Addr,
};
use liquidator::{
    contract::{execute, instantiate, query},
    msg::{ExecuteMsg, GetOwnerResponse, InstantiateMsg, QueryMsg},
    state::Config,
};

#[test]
fn test_instantiation() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        ibc_channel_id: "channel-10".to_string(),
        ibc_to_address: "neutron1yrg6daqkxyeqye4aac09stzvvwppqwlsk2jn2k".to_string(),
        liquidation_target: "uosmo".to_string(),
        owner: "addr0000".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
    let resp: GetOwnerResponse = from_binary(&res).unwrap();
    let owner = resp.owner;

    assert_eq!(owner, Addr::unchecked("addr0000".to_string()));

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
    let resp: Config = from_binary(&res).unwrap();
    let ibc_channel_id = resp.ibc_channel_id;
    let ibc_to_address = resp.ibc_to_address;
    let liquidation_target = resp.liquidation_target;

    assert_eq!("channel-10".to_string(), ibc_channel_id);
    assert_eq!("neutron1yrg6daqkxyeqye4aac09stzvvwppqwlsk2jn2k".to_string(), ibc_to_address);
    assert_eq!("uosmo".to_string(), liquidation_target);
}

#[test]
fn test_update_owner() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        ibc_channel_id: "channel-10".to_string(),
        ibc_to_address: "neutron1yrg6daqkxyeqye4aac09stzvvwppqwlsk2jn2k".to_string(),
        liquidation_target: "uosmo".to_string(),
        owner: "addr0000".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Update the owner
    let msg = ExecuteMsg::UpdateOwner {
        owner: "addr0001".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
    let resp: GetOwnerResponse = from_binary(&res).unwrap();
    let owner = resp.owner;

    assert_eq!(owner, Addr::unchecked("addr0001".to_string()));
}

#[test]
fn test_update_config() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        ibc_channel_id: "channel-10".to_string(),
        ibc_to_address: "neutron1yrg6daqkxyeqye4aac09stzvvwppqwlsk2jn2k".to_string(),
        liquidation_target: "uosmo".to_string(),
        owner: "addr0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Update the owner
    let msg = ExecuteMsg::UpdateConfig {
        ibc_channel_id: "channel-10".to_string(),
        ibc_to_address: "centauri1yrg6daqkxyeqye4aac09stzvvwppqwlsk2jn2k".to_string(),
        liquidation_target: "pica".to_string(),
    };

    let info = mock_info("addr0000", &[]);
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
    let resp: Config = from_binary(&res).unwrap();
    let ibc_channel_id = resp.ibc_channel_id;
    let ibc_to_address = resp.ibc_to_address;
    let liquidation_target = resp.liquidation_target;

    assert_eq!("channel-10".to_string(), ibc_channel_id);
    assert_eq!("centauri1yrg6daqkxyeqye4aac09stzvvwppqwlsk2jn2k".to_string(), ibc_to_address);
    assert_eq!("pica".to_string(), liquidation_target);
}

#[test]
fn test_liquidate() {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        ibc_channel_id: "channel-10".to_string(),
        ibc_to_address: "neutron1yrg6daqkxyeqye4aac09stzvvwppqwlsk2jn2k".to_string(),
        liquidation_target: "uosmo".to_string(),
        owner: "addr0000".to_string(),
    };
    let info = mock_info("addr0000", &[]);

    instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // Update the owner
    let msg = ExecuteMsg::Liquidate {};

    let info = mock_info("addr0000", &[]);
    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    // TODO
    assert_eq!(1, 1);
}

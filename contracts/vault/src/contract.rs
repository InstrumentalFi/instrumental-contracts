use cosmwasm_std::{
    ensure_eq, entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use pablo_vault_types::vault::{Config, ExecuteMsg, InstantiateMsg, QueryMsg, State};

use crate::{
    error::ContractError,
    state::{CONFIG, STATE},
};

pub const CONTRACT_NAME: &str = "crates.io:pablo-vault";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DAY_IN_SECONDS: u64 = 24 * 60 * 60; // 24 hours
pub const TWO_DAYS_IN_SECONDS: u64 = 48 * 60 * 60; // 48 hours

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    msg.validate()?;

    CONFIG.save(
        deps.storage,
        &Config {
            token_a: msg.token_a,
            token_b: msg.token_b,
            owner: info.sender,
            compound_wait_period: DAY_IN_SECONDS,
            harvest_wait_period: DAY_IN_SECONDS,
        },
    )?;

    STATE.save(
        deps.storage,
        &State {
            last_harvest: env.block.time,
            last_compound: env.block.time,
        },
    )?;

    Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposit {} => execute_deposit(deps, env, info, msg),
        ExecuteMsg::Withdraw {} => execute_withdraw(deps, env, info, msg),
        ExecuteMsg::Harvest {} => execute_harvest(deps, env, info, msg),
        ExecuteMsg::Compound {} => execute_compound(deps, env, info, msg),
        ExecuteMsg::DistributeRewards {} => execute_distribute_rewards(deps, env, info, msg),
        ExecuteMsg::UpdateConfig {
            compound_wait_period,
            harvest_wait_period,
        } => execute_update_config(deps, info, compound_wait_period, harvest_wait_period),
    }
}

/// Deposits an equal amount of two tokens into the vault, returning a new token representing
/// ownership of a deposit.
pub fn execute_deposit(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!();
}

/// Withdraws a position from the vault by sending a token representing ownership of a deposit
/// ownership over a deposit. This burns the ownership token and returns the underlying tokens to
/// the caller
pub fn execute_withdraw(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!();
}

/// Harvests rewards from the rewards contract and holds rewards on the vault contract
pub fn execute_harvest(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!();
}

/// Compounds rewards by
/// * Selling the rewards token
/// * Buying equal amounts of the underlying for the LP  (e.g. DOT/sDOT)
/// * Investing underlying in the LP
/// * Staking the LP Token in the rewards contract
pub fn execute_compound(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!();
}

/// Distribute rewards to the rewards contract
pub fn execute_distribute_rewards(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!();
}

/// Sets the harvest wait period. If the `execute_harvest` function is called
/// before the wait period has expired an error will be returned
pub fn execute_update_config(
    deps: DepsMut,
    info: MessageInfo,
    harvest_wait_period: Option<String>,
    compound_wait_period: Option<String>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    ensure_eq!(info.sender, config.owner, ContractError::Unauthorized {});

    if let Some(compound_wait_period) = compound_wait_period {
        config.compound_wait_period = compound_wait_period.parse::<u64>().unwrap();
    }

    if let Some(harvest_wait_period) = harvest_wait_period {
        config.harvest_wait_period = harvest_wait_period.parse::<u64>().unwrap();
    }
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default().add_attribute("action", "update_config"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::TokenBalances {} => to_binary(&query_token_balances(deps)?),
    }
}

/// Returns the configuration set during contract instnatiation
pub fn query_config(deps: Deps) -> StdResult<Config> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(config)
}

/// Return the current state of the contract
pub fn query_state(deps: Deps) -> StdResult<State> {
    let state = STATE.load(deps.storage)?;
    Ok(state)
}

/// Returns token balances held by the contract
pub fn query_token_balances(_deps: Deps) -> StdResult<Config> {
    unimplemented!();
}

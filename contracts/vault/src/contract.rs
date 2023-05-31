use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use pablo_vault_types::vault::{
    Config, ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, State,
};

use crate::{
    error::ContractError,
    state::{CONFIG, STATE},
};

pub const CONTRACT_NAME: &str = "crates.io:pablo-vault";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DAY_IN_SECONDS: u64 = 24 * 60 * 60; // 24 hours

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
        ExecuteMsg::SetHarvestWaitPeriod {} => {
            execute_set_harvest_wait_period(deps, env, info, msg)
        }
        ExecuteMsg::SetCompoundWaitPeriod {} => {
            execute_set_compound_wait_period(deps, env, info, msg)
        }
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
pub fn execute_set_harvest_wait_period(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!();
}

/// Sets the compound wait period. If the `execute_compound` function is called
/// before the wait period has expired an error will be returned
pub fn execute_set_compound_wait_period(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!();
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::LastHarvest {} => to_binary(&query_last_harvest(deps)?),
        QueryMsg::LastCompound {} => to_binary(&query_last_compound(deps)?),
        QueryMsg::HarvestWaitPeriod {} => to_binary(&query_harvest_wait_period(deps)?),
        QueryMsg::CompoundWaitPeriod {} => to_binary(&query_compound_wait_period(deps)?),
        QueryMsg::TokenBalances {} => to_binary(&query_token_balances(deps)?),
    }
}

/// Returns the configuration set during contract instnatiation
pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config: Config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        token_a: config.token_a,
        token_b: config.token_b,
        owner: config.owner,
        harvest_wait_period: config.harvest_wait_period,
        compound_wait_period: config.compound_wait_period,
    })
}

/// Returns a timestamp of the last harvest
pub fn query_last_harvest(_deps: Deps) -> StdResult<ConfigResponse> {
    unimplemented!();
}

/// Returns a timestamp of the last compound
pub fn query_last_compound(_deps: Deps) -> StdResult<ConfigResponse> {
    unimplemented!();
}

/// Returns the harvest wait period
pub fn query_harvest_wait_period(_deps: Deps) -> StdResult<ConfigResponse> {
    unimplemented!();
}

/// Returns the compound wait period
pub fn query_compound_wait_period(_deps: Deps) -> StdResult<ConfigResponse> {
    unimplemented!();
}

/// Returns token balances held by the contract
pub fn query_token_balances(_deps: Deps) -> StdResult<ConfigResponse> {
    unimplemented!();
}

use crate::{
    error::ContractError,
    handle::{
        handle_claim, handle_pause, handle_stake, handle_unpause, handle_unstake,
        handle_update_config, handle_update_rewards,
    },
    query::{query_claimable, query_user_staked_amount},
    state::{
        query_config, query_state, Config, State, CONFIG, REWARDS_PER_TOKEN, STATE, TOTAL_STAKED,
    },
};

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, ReplyOn, Response,
    StdResult, SubMsg, Uint128,
};
use cw2::set_contract_version;
use fee_distribution::staking::{ExecuteMsg, InstantiateMsg, QueryMsg};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgCreateDenomResponse};

pub const INSTANTIATE_REPLY_ID: u64 = 1u64;

// version info for migration info
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// NOTE: decimal places assumed for all calulations, this may change
pub const DECIMALS: u128 = 1_000_000u128;
pub const DECIMAL_PLACES: u32 = 6u32;
pub const SCALE_FACTOR: u128 = 10_000u128;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    deps.api.debug("instantiate");
    set_contract_version(deps.storage, format!("crates.io:{CONTRACT_NAME}"), CONTRACT_VERSION)?;

    let create_denom_submsg = SubMsg {
        id: INSTANTIATE_REPLY_ID,
        msg: MsgCreateDenom {
            sender: env.contract.address.to_string(),
            subdenom: format!("staked{}", msg.deposit_denom),
        }
        .into(),
        gas_limit: None,
        reply_on: ReplyOn::Success,
    };

    CONFIG.save(
        deps.storage,
        &Config {
            owner: info.sender,
            fee_collector: deps.api.addr_validate(&msg.fee_collector)?,
            deposit_denom: msg.deposit_denom.clone(),
            staked_denom: "".to_string(),
            reward_denom: msg.reward_denom.clone(),
            deposit_decimals: msg.deposit_decimals,
            reward_decimals: msg.reward_decimals,
            tokens_per_interval: msg.tokens_per_interval,
        },
    )?;

    STATE.save(
        deps.storage,
        &State {
            is_open: false,
            last_distribution: env.block.time,
        },
    )?;

    TOTAL_STAKED.save(deps.storage, &Uint128::zero())?;
    REWARDS_PER_TOKEN.save(deps.storage, &Uint128::zero())?;

    Ok(Response::new().add_submessage(create_denom_submsg).add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        INSTANTIATE_REPLY_ID => {
            let MsgCreateDenomResponse {
                new_token_denom,
            } = msg.result.try_into()?;

            CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
                config.staked_denom = new_token_denom;
                Ok(config)
            })?;

            Ok(Response::new())
        }
        _ => Err(ContractError::generic_err(format!("reply (id {:?}) invalid", msg.id))),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateConfig {
            tokens_per_interval,
        } => handle_update_config(deps, info, tokens_per_interval),
        ExecuteMsg::UpdateRewards {} => handle_update_rewards(deps, env),
        ExecuteMsg::Stake {} => handle_stake(deps, env, info),
        ExecuteMsg::Unstake {} => handle_unstake(deps, env, info),
        ExecuteMsg::Claim {
            recipient,
        } => handle_claim(deps, env, info, recipient),
        ExecuteMsg::Pause {} => handle_pause(deps, info),
        ExecuteMsg::Unpause {} => handle_unpause(deps, info),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::GetClaimable {
            user,
        } => to_binary(&query_claimable(deps, env, user)?),
        QueryMsg::GetUserStakedAmount {
            user,
        } => to_binary(&query_user_staked_amount(deps, user)?),
    }
}

use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError,
    StdResult, SubMsgResult, Uint128,
};
use cw2::set_contract_version;
use cw_utils::parse_instantiate_response_data;
use fee_distribution::staking::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::{
    error::ContractError,
    handle::{
        handle_claim, handle_pause, handle_stake, handle_unpause, handle_update_config,
        handle_update_rewards,
    },
    messages::{create_instantiate_token_msg, receive_cw20},
    query::{query_claimable, query_user_staked_amount},
    state::{query_config, query_state, Config, State, CONFIG, REWARDS_PER_TOKEN, STATE},
};

pub const INSTANTIATE_REPLY_ID: u64 = 1u64;

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
    set_contract_version(deps.storage, format!("crates.io:{CONTRACT_NAME}"), CONTRACT_VERSION)?;

    CONFIG.save(
        deps.storage,
        &Config {
            owner: info.sender,
            // this contract should be owner of fee collector
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

    REWARDS_PER_TOKEN.save(deps.storage, &Uint128::zero())?;

    let create_token_msg = create_instantiate_token_msg(
        msg.token_code_id,
        msg.token_name,
        msg.deposit_denom,
        msg.deposit_decimals as u8,
        env.contract.address.to_string(),
    );

    Ok(Response::new().add_submessage(create_token_msg).add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id {
        INSTANTIATE_REPLY_ID => {
            let mut config = CONFIG.load(deps.storage)?;

            if !config.staked_denom.is_empty() {
                return Err(ContractError::Unauthorized {});
            }

            let data = match msg.result {
                SubMsgResult::Ok(res) => res.data.unwrap(),
                SubMsgResult::Err(err) => {
                    return Err(ContractError::generic_err(format!(
                        "reply (id {:?}) error: {:?}",
                        msg.id, err
                    )))
                }
            };

            let init_response = parse_instantiate_response_data(data.as_slice())
                .map_err(|e| StdError::generic_err(format!("{e}")))?;

            deps.api.addr_validate(&init_response.contract_address)?;

            config.staked_denom = init_response.contract_address;

            CONFIG.save(deps.storage, &config)?;

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
        ExecuteMsg::Receive(msg) => receive_cw20(deps, env, info, msg),
        // ExecuteMsg::Unstake {} => handle_unstake(deps, env, info),
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

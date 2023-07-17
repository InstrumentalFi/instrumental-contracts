use crate::{
    distributor::update_rewards,
    error::ContractError,
    helper::{create_distribute_message, parse_funds},
    messages::{create_burn_token_msg, create_mint_token_msg},
    state::{UserStake, CONFIG, STATE, TOTAL_STAKED, USER_STAKE},
};

use cosmwasm_std::{
    ensure, ensure_eq, ensure_ne, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw20::Cw20ReceiveMsg;
use osmosis_std::types::{cosmos::bank::v1beta1::MsgSend, cosmos::base::v1beta1::Coin};

pub fn handle_update_config(
    deps: DepsMut,
    info: MessageInfo,
    tokens_per_interval: Option<Uint128>,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    ensure_eq!(info.sender, config.owner, ContractError::Unauthorized {});

    if let Some(tokens_per_interval) = tokens_per_interval {
        config.tokens_per_interval = tokens_per_interval;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default().add_attribute("action", "update_config"))
}

pub fn handle_update_rewards(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let (_, rewards) = update_rewards(deps, env.clone(), env.contract.address.clone())?;

    let mut response = Response::new();
    if !rewards.is_zero() {
        let distribute_msg = create_distribute_message(
            config.fee_collector.to_string(),
            config.reward_denom,
            rewards,
            env.contract.address.to_string(),
        );

        response = response.add_message(distribute_msg);
    }

    Ok(response.add_attribute("action", "update_rewards"))
}

pub fn handle_pause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    let config = CONFIG.load(deps.storage)?;

    ensure_eq!(info.sender, config.owner, ContractError::Unauthorized {});

    if !state.is_open {
        return Err(ContractError::Paused {});
    }
    state.is_open = false;

    STATE.save(deps.storage, &state)?;

    Ok(Response::default().add_attribute("action", "paused"))
}

pub fn handle_unpause(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    let config = CONFIG.load(deps.storage)?;

    ensure_eq!(info.sender, config.owner, ContractError::Unauthorized {});

    if state.is_open {
        return Err(ContractError::NotPaused {});
    }

    state.is_open = true;

    STATE.save(deps.storage, &state)?;

    Ok(Response::default().add_attribute("action", "unpaused"))
}

pub fn handle_claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Option<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    let sender = info.sender;

    ensure!(state.is_open, ContractError::Paused {});

    let recipient = match recipient {
        Some(recipient) => {
            deps.api.addr_validate(recipient.as_str())?;
            recipient
        }
        None => sender.to_string(),
    };

    let (deps, rewards) = update_rewards(deps, env.clone(), sender.clone())?;

    let mut claimable_amount = Uint128::zero();
    USER_STAKE.update(deps.storage, sender, |res| -> StdResult<_> {
        let mut stake = match res {
            Some(stake) => stake,
            None => UserStake::default(),
        };

        claimable_amount = stake.claimable_rewards;
        stake.claimable_rewards = Uint128::zero();

        Ok(stake)
    })?;

    let mut response = Response::new();
    if !rewards.is_zero() {
        let distribute_msg = create_distribute_message(
            config.fee_collector.to_string(),
            config.reward_denom.clone(),
            rewards,
            env.contract.address.to_string(),
        );

        response = response.add_message(distribute_msg);
    }

    if !claimable_amount.is_zero() {
        let msg_claim = MsgSend {
            from_address: env.contract.address.to_string(),
            to_address: recipient,
            amount: vec![Coin {
                denom: config.reward_denom,
                amount: claimable_amount.into(),
            }],
        };
        response = response.add_message(msg_claim);
    }

    Ok(response.add_attribute("action", "claim"))
}

pub fn handle_stake(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    let sender = info.sender;
    let sent_funds: Uint128 = parse_funds(info.funds, config.deposit_denom.clone())?;

    ensure!(state.is_open, ContractError::Paused {});
    ensure_ne!(sent_funds, Uint128::zero(), ContractError::InvalidFunds {});

    let (deps, rewards) = update_rewards(deps, env.clone(), sender.clone())?;

    USER_STAKE.update(deps.storage, sender.clone(), |res| -> StdResult<_> {
        let mut stake = match res {
            Some(stake) => stake,
            None => UserStake::default(),
        };

        stake.staked_amounts = stake.staked_amounts.checked_add(sent_funds).unwrap();

        Ok(stake)
    })?;

    TOTAL_STAKED
        .update(deps.storage, |balance| -> StdResult<Uint128> {
            Ok(balance.checked_add(sent_funds).unwrap())
        })
        .unwrap();

    let mut response = Response::new();
    if !rewards.is_zero() {
        let distribute_msg = create_distribute_message(
            config.fee_collector.to_string(),
            config.reward_denom,
            rewards,
            env.contract.address.to_string(),
        );

        response = response.add_message(distribute_msg);
    }

    let msg_mint = create_mint_token_msg(sent_funds, sender.to_string(), config.staked_denom);

    Ok(response.add_message(msg_mint).add_attribute("action", "stake"))
}

pub fn handle_unstake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw20_msg: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let state = STATE.load(deps.storage)?;

    let sender = deps.api.addr_validate(cw20_msg.sender.as_str())?;
    let sent_funds: Uint128 = cw20_msg.amount;

    deps.api.debug(&format!("unstake: sender: {}, amount: {}", sender, sent_funds));

    ensure!(state.is_open, ContractError::Paused {});

    ensure_eq!(info.sender, config.staked_denom, ContractError::InvalidFunds {});
    ensure_ne!(sent_funds, Uint128::zero(), ContractError::InvalidFunds {});

    let (deps, rewards) = update_rewards(deps, env.clone(), sender.clone())?;

    USER_STAKE.update(deps.storage, sender.clone(), |res| -> StdResult<_> {
        let mut stake = match res {
            Some(stake) => stake,
            None => UserStake::default(),
        };

        stake.staked_amounts = stake.staked_amounts.checked_sub(sent_funds).unwrap();

        Ok(stake)
    })?;

    TOTAL_STAKED
        .update(deps.storage, |balance| -> StdResult<Uint128> {
            Ok(balance.checked_sub(sent_funds).unwrap())
        })
        .unwrap();

    let mut response = Response::new();
    if !rewards.is_zero() {
        let distribute_msg = create_distribute_message(
            config.fee_collector.to_string(),
            config.reward_denom,
            rewards,
            env.contract.address.to_string(),
        );

        response = response.add_message(distribute_msg);
    }

    let msg_burn = create_burn_token_msg(sent_funds, config.staked_denom.clone());

    let msg_unstake = MsgSend {
        from_address: env.contract.address.to_string(),
        to_address: sender.to_string(),
        amount: vec![Coin {
            denom: config.deposit_denom,
            amount: sent_funds.into(),
        }],
    }
    .into();

    Ok(response.add_messages(vec![msg_burn, msg_unstake]).add_attribute("action", "unstake"))
}

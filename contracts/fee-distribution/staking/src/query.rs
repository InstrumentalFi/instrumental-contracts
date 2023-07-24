use cosmwasm_std::{Deps, Env, StdResult, Uint128};

use crate::{
    helper::get_token_total_supply,
    state::{UserStake, CONFIG, REWARDS_PER_TOKEN, STATE, USER_STAKE},
};

pub fn query_user_staked_amount(deps: Deps, address: String) -> StdResult<UserStake> {
    let user = deps.api.addr_validate(&address)?;
    let user_stake = USER_STAKE.may_load(deps.storage, user)?;

    match user_stake {
        Some(stake) => Ok(stake),
        None => Ok(UserStake::default()),
    }
}

pub fn query_pending_rewards(deps: Deps, env: Env) -> StdResult<Uint128> {
    let state = STATE.load(deps.storage).unwrap();
    let config = CONFIG.load(deps.storage).unwrap();

    if state.last_distribution == env.block.time {
        return Ok(Uint128::zero());
    };

    let delta =
        Uint128::from((env.block.time.seconds() - state.last_distribution.seconds()) as u128);

    let pending_rewards = delta.checked_mul(config.tokens_per_interval).unwrap();

    Ok(pending_rewards)
}

pub fn query_claimable(deps: Deps, env: Env, address: String) -> StdResult<Uint128> {
    let config = CONFIG.load(deps.storage).unwrap();
    let decimal_places = 10u128.pow(config.reward_decimals);

    let user = deps.api.addr_validate(&address)?;

    let stake = USER_STAKE.load(deps.storage, user).unwrap_or_default();
    if stake.staked_amounts.is_zero() {
        return Ok(Uint128::zero());
    };

    let pending_rewards =
        query_pending_rewards(deps, env)?.checked_mul(decimal_places.into()).unwrap();

    let supply = get_token_total_supply(deps);
    let rewards_per_token = REWARDS_PER_TOKEN.load(deps.storage)?;

    let next_reward_per_token =
        rewards_per_token.checked_add(pending_rewards.checked_div(supply).unwrap()).unwrap();

    let latest_rewards = stake
        .staked_amounts
        .checked_mul(
            next_reward_per_token.checked_sub(stake.previous_cumulative_rewards_per_token).unwrap(),
        )
        .unwrap()
        .checked_div(decimal_places.into())
        .unwrap();

    Ok(stake.claimable_rewards.checked_add(latest_rewards).unwrap())
}

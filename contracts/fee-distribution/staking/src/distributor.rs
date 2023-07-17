use crate::{
    helper::{get_bank_balance, get_token_total_supply},
    query::query_pending_rewards,
    state::{CONFIG, REWARDS_PER_TOKEN, STATE, USER_STAKE},
};

use cosmwasm_std::{Addr, Deps, DepsMut, Env, StdResult, Storage, Uint128};

pub fn calculate_rewards(deps: Deps, env: Env) -> Uint128 {
    let config = CONFIG.load(deps.storage).unwrap();

    let block_rewards = query_pending_rewards(deps, env).unwrap();

    let balance = get_bank_balance(deps, config.fee_collector.to_string(), config.reward_denom);

    if block_rewards > balance {
        return balance;
    }

    block_rewards
}

pub fn update_distribution_time(storage: &mut dyn Storage, env: Env) -> StdResult<()> {
    STATE
        .update(storage, |mut s| -> StdResult<_> {
            s.last_distribution = env.block.time;
            Ok(s)
        })
        .unwrap();

    Ok(())
}

pub fn update_rewards(deps: DepsMut, env: Env, account: Addr) -> StdResult<(DepsMut, Uint128)> {
    let config = CONFIG.load(deps.storage).unwrap();
    let decimal_places = 10u128.pow(config.reward_decimals);

    let block_rewards = calculate_rewards(deps.as_ref(), env.clone());
    update_distribution_time(deps.storage, env.clone()).unwrap();

    let supply = get_token_total_supply(deps.as_ref());

    let mut cumulative_rewards = REWARDS_PER_TOKEN.load(deps.storage).unwrap();
    if !supply.is_zero() && !block_rewards.is_zero() {
        cumulative_rewards = cumulative_rewards
            .checked_add(
                block_rewards
                    .checked_mul(decimal_places.into())
                    .unwrap()
                    .checked_div(supply)
                    .unwrap(),
            )
            .unwrap();

        REWARDS_PER_TOKEN.save(deps.storage, &cumulative_rewards)?;
    }

    // if account is contract, don't update rewards, exit early
    if account == env.contract.address {
        return Ok((deps, block_rewards));
    }

    let mut user = USER_STAKE.load(deps.storage, account.clone()).unwrap_or_default();

    let delta_rewards =
        cumulative_rewards.checked_sub(user.previous_cumulative_rewards_per_token).unwrap();

    let account_reward = user
        .staked_amounts
        .checked_mul(delta_rewards)
        .unwrap()
        .checked_div(decimal_places.into())
        .unwrap();

    user.claimable_rewards = user.claimable_rewards.checked_add(account_reward).unwrap();
    user.previous_cumulative_rewards_per_token = cumulative_rewards;

    if !user.claimable_rewards.is_zero() && !user.staked_amounts.is_zero() {
        let next_cumulative_reward =
            user.cumulative_rewards.checked_add(user.claimable_rewards).unwrap();

        // TODO: tidy
        user.average_staked_amounts = user
            .average_staked_amounts
            .checked_mul(user.cumulative_rewards)
            .unwrap()
            .checked_div(next_cumulative_reward)
            .unwrap()
            .checked_add(
                user.staked_amounts
                    .checked_mul(user.claimable_rewards)
                    .unwrap()
                    .checked_div(next_cumulative_reward)
                    .unwrap(),
            )
            .unwrap();

        user.cumulative_rewards = next_cumulative_reward;
    }

    USER_STAKE.save(deps.storage, account, &user)?;

    Ok((deps, block_rewards))
}

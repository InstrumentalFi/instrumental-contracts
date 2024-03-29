use apollo_cw_asset::{Asset, AssetInfo};
use cosmwasm_std::{attr, Addr, Binary, DepsMut, Env, Event, Response, StdError, Uint128};
use cw_storage_plus::Item;
use cw_vault_token::{CwTokenError, VaultToken};
use serde::{de::DeserializeOwned, Serialize};

pub const DEFAULT_VAULT_TOKENS_PER_STAKED_BASE_TOKEN: Uint128 = Uint128::new(1_000_000);

pub struct BaseVault<'a, V> {
    /// The vault token implementation for this vault
    pub vault_token: Item<'a, V>,

    /// The token that is depositable to the vault and which is used for
    /// accounting (calculating to/from vault tokens).
    pub base_token: Item<'a, AssetInfo>,

    /// The total number of base tokens held by the vault.
    /// We need to store this rather than query it to prevent manipulation of
    /// the vault token price and prevent an exploit similar to the Cream
    /// Finance October 2021 exploit.
    pub total_staked_base_tokens: Item<'a, Uint128>,
}

/// Create default empty struct. The Items here will not have anything saved
/// so you must call base_vault.init() to save values for each of them before
/// being able to read them.
impl<V> Default for BaseVault<'_, V> {
    fn default() -> Self {
        BaseVault {
            vault_token: Item::new("vault_token"),
            base_token: Item::new("base_token"),
            total_staked_base_tokens: Item::new("total_staked_base_tokens"),
        }
    }
}

impl<'a, V> BaseVault<'a, V>
where
    V: Serialize + DeserializeOwned + VaultToken,
{
    /// Save values for all of the Items in the struct and instantiate the vault
    /// token.
    pub fn init(
        &self,
        deps: DepsMut,
        base_token: AssetInfo,
        vault_token: V,
        init_info: Option<Binary>,
    ) -> Result<Response, CwTokenError> {
        self.vault_token.save(deps.storage, &vault_token)?;
        self.base_token.save(deps.storage, &base_token)?;
        self.total_staked_base_tokens.save(deps.storage, &Uint128::zero())?;

        vault_token.instantiate(deps, init_info)
    }

    /// Helper function to send `amount` number of base tokens to `recipient`.
    pub fn send_base_tokens(
        &self,
        deps: DepsMut,
        recipient: &Addr,
        amount: Uint128,
    ) -> Result<Response, StdError> {
        let asset = Asset {
            info: self.base_token.load(deps.storage)?,
            amount,
        };

        let msg = asset.transfer_msg(recipient)?;

        let event = Event::new("apollo/vaults/base_vault").add_attributes(vec![
            attr("action", "send_base_tokens"),
            attr("recipient", recipient),
            attr("amount", amount),
        ]);

        Ok(Response::new().add_message(msg).add_event(event))
    }

    /// Converts an amount of base_tokens to an amount of vault_tokens.
    pub fn calculate_vault_tokens(
        &self,
        base_tokens: Uint128,
        total_staked_amount: Uint128,
        vault_token_supply: Uint128,
    ) -> Result<Uint128, StdError> {
        let vault_tokens = if total_staked_amount.is_zero() {
            base_tokens.checked_mul(DEFAULT_VAULT_TOKENS_PER_STAKED_BASE_TOKEN)?
        } else {
            vault_token_supply.multiply_ratio(base_tokens, total_staked_amount)
        };

        Ok(vault_tokens)
    }

    /// Converts an amount of vault_tokens to an amount of base_tokens.
    pub fn calculate_base_tokens(
        &self,
        vault_tokens: Uint128,
        total_staked_amount: Uint128,
        vault_token_supply: Uint128,
    ) -> Result<Uint128, StdError> {
        let base_tokens = if vault_token_supply.is_zero() {
            vault_tokens.checked_div(DEFAULT_VAULT_TOKENS_PER_STAKED_BASE_TOKEN)?
        } else {
            total_staked_amount.multiply_ratio(vault_tokens, vault_token_supply)
        };

        Ok(base_tokens)
    }

    /// Returns a `Response` with a message to burn the specified amount of
    /// vault tokens, as well as the amount of base_tokens that this amount
    /// of vault tokens represents. Also updates total_staked_base_tokens.
    /// This function burns from the contract balance, and thus the tokens must
    /// have been transfered to the contract before calling this function.
    pub fn burn_vault_tokens_for_base_tokens(
        &self,
        deps: DepsMut,
        env: &Env,
        vault_tokens: Uint128,
    ) -> Result<(Uint128, Response), StdError> {
        // Load state
        let vault_token = self.vault_token.load(deps.storage)?;
        let total_staked_amount = self.total_staked_base_tokens.load(deps.storage)?;
        let vault_token_supply = vault_token.query_total_supply(deps.as_ref())?;

        // Calculate how many base tokens the given amount of vault tokens represents
        let base_tokens =
            self.calculate_base_tokens(vault_tokens, total_staked_amount, vault_token_supply)?;

        // Update total staked amount
        self.total_staked_base_tokens
            .save(deps.storage, &total_staked_amount.checked_sub(base_tokens)?)?;

        let event = Event::new("apollo/vaults/base_vault").add_attributes(vec![
            attr("action", "burn_vault_tokens_for_base_tokens"),
            attr("burned_vault_token_amount", vault_tokens),
            attr("calculated_receive_base_token_amount", base_tokens),
        ]);

        // Return calculated amount of base_tokens and message to burn vault tokens
        Ok((base_tokens, vault_token.burn(deps, env, vault_tokens)?.add_event(event)))
    }
}

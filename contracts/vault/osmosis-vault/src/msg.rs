use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    /// Address that is allowed to update config.
    //pub admin: String,
    /// The ID of the pool that this vault will autocompound.
    pub pool_id: u64,

    /// Configurable parameters for the contract.
    //pub config: ConfigUnchecked,
    /// The subdenom that will be used for the native vault token, e.g.
    /// the denom of the vault token will be:
    /// "factory/{vault_contract}/{vault_token_subdenom}".
    pub vault_token_subdenom: String,
}

use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("User stake not found")]
    UserStakeNotFound {},

    #[error("Invalid funds")]
    InvalidFunds {},

    #[error("Invalid liquidation")]
    InvalidLiquidation {},

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("Vault does not exist, cannot perform operation")]
    VaultDoesNotExist {},

    #[error("Vault is not safe, cannot perform operation")]
    UnsafeVault {},

    #[error("Vault is safe, cannot be liquidated")]
    SafeVault {},

    #[error("Contract is paused")]
    Paused {},

    #[error("Contract is not paused")]
    NotPaused {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}

impl ContractError {
    pub fn generic_err(msg: impl Into<String>) -> ContractError {
        ContractError::Std(StdError::generic_err(msg))
    }
}

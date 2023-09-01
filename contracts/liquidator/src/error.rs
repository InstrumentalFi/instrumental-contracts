use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Owner not set")]
    NoOwner {},

    #[error("Owner update error")]
    OwnerUpdateError {},

    #[error("InvalidTokenShare")]
    InvalidTokenShare {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid Pool Route: {reason:?}")]
    InvalidPoolRoute {
        reason: String,
    },
    #[error("Amount was zero")]
    ZeroAmount {},

    #[error("Insufficient Funds")]
    InsufficientFunds {},

    #[error("Failed Swap: {reason:?}")]
    FailedSwap {
        reason: String,
    },

    #[error("Custom Error val: {val:?}")]
    CustomError {
        val: String,
    },
}

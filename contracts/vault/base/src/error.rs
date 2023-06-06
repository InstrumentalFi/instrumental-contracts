use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Invalid chain prefix: {0}")]
    InvalidChainPrefix(String),
}

pub type ContractResult<T> = Result<T, ContractError>;

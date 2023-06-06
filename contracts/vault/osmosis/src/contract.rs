//use cosmwasm_std::Empty;
use vault_base::VaultBase;

pub type OsmosisVault<'a> = VaultBase<'a>;

pub const CONTRACT_NAME: &str = "crates.io:osmosis-vault";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(feature = "library"))]
pub mod entry {
    use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response};
    use pablo_vault_types::vault::{ExecuteMsg, InstantiateMsg, QueryMsg};
    use vault_base::ContractResult;

    use super::*;
    // use crate::migrations;

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> ContractResult<Response> {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        OsmosisVault::default().instantiate(deps, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> ContractResult<Response> {
        OsmosisVault::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<Binary> {
        OsmosisVault::default().query(deps, env, msg)
    }

    // #[entry_point]
    // pub fn migrate(deps: DepsMut, _env: Env, _msg: Empty) -> ContractResult<Response> {
    //     migrations::v1_0_0::migrate(deps)
    // }
}

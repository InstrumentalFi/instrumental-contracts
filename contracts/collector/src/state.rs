use cosmwasm_std::{Addr, Deps, DepsMut, StdError::GenericErr, StdResult, Storage};
use cw_storage_plus::Item;

pub const WHITELIST_ADDRESS: Item<Addr> = Item::new("whitelist-address");
pub const TOKEN_LIST: Item<Vec<String>> = Item::new("token-list");
pub const TOKEN_LIMIT: usize = 3usize;

// function checks if an addr is already added and adds it if not
// We also check that we have not reached the limit of tokens here
pub fn save_token(deps: DepsMut, denom: String) -> StdResult<()> {
    // check if the list exists already
    let mut token_list = match TOKEN_LIST.may_load(deps.storage)? {
        None => vec![],
        Some(list) => list,
    };

    // check if we already added the token
    if token_list.contains(&denom) {
        return Err(GenericErr {
            msg: "This token is already added".to_string(),
        });
    };

    // check if we have reached the capacity
    if token_list.len() >= TOKEN_LIMIT {
        return Err(GenericErr {
            msg: "The token capacity is already reached".to_string(),
        });
    };

    // add the token
    token_list.push(denom);
    TOKEN_LIST.save(deps.storage, &token_list)
}

// this function reads Addrs stored in the TOKEN_LIST.
// note that this function ONLY takes the first TOKEN_LIMIT terms
pub fn read_token_list(deps: Deps, limit: usize) -> StdResult<Vec<String>> {
    match TOKEN_LIST.may_load(deps.storage)? {
        None => Err(GenericErr {
            msg: "No tokens are stored".to_string(),
        }),
        Some(list) => {
            let take = limit.min(list.len());
            Ok(list[..take].to_vec())
        }
    }
}

// this function checks whether the token is stored already
pub fn is_token(storage: &dyn Storage, token: String) -> bool {
    match TOKEN_LIST.may_load(storage).unwrap() {
        None => false,
        Some(list) => list.contains(&token),
    }
}

// this function deletes the entry under the given key
pub fn remove_token(deps: DepsMut, denom: String) -> StdResult<()> {
    // check if the list exists
    let mut token_list = match TOKEN_LIST.may_load(deps.storage)? {
        None => {
            return Err(GenericErr {
                msg: "No tokens are stored".to_string(),
            })
        }
        Some(value) => value,
    };

    // check if the token is added
    if !token_list.contains(&denom) {
        return Err(GenericErr {
            msg: "This token has not been added".to_string(),
        });
    }

    // change token_list
    let index = token_list.clone().iter().position(|x| x.eq(&denom)).unwrap();
    token_list.swap_remove(index);

    // saves the updated token_list
    TOKEN_LIST.save(deps.storage, &token_list)
}

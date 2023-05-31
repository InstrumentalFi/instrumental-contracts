use cw_storage_plus::Item;
use pablo_vault_types::vault::{Config, State};

pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");

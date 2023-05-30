use cw_storage_plus::Item;
use pablo_vault_types::vault::Config;

pub const CONFIG: Item<Config> = Item::new("config");

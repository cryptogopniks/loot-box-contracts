use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::treasury::types::{Balance, Config, TransferAdminState};

pub const CONTRACT_NAME: &str = "cryptogopniks-loot-box-treasury";

pub const PLATFORM_CODE_ID: u64 = 1;
pub const SAVE_PLATFORM_REPLY: u64 = 0;

pub const PAGINATION_MAX_LIMIT: u32 = 10_000;
pub const PAGINATION_DEFAULT_LIMIT: u32 = 1_000;

pub const TRANSFER_ADMIN_TIMEOUT: u64 = 7 * 24 * 3600;

pub const IS_LOCKED: Item<bool> = Item::new("is locked");
pub const TRANSFER_ADMIN_STATE: Item<TransferAdminState> = Item::new("transfer admin state");
pub const CONFIG: Item<Config> = Item::new("config");

pub const BALANCE: Item<Balance> = Item::new("balance");

pub const PLATFORM_LIST: Item<Vec<Addr>> = Item::new("platform list");
pub const REMOVED_PLATFORM_LIST: Item<Vec<Addr>> = Item::new("removed platform list");

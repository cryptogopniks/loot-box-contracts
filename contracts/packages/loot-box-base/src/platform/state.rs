use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::{Item, Map};

use crate::platform::types::{Balance, BoxStats, Config, TransferAdminState, UserInfo};

pub const CONTRACT_NAME: &str = "cryptogopniks-loot-box";

pub const NORMALIZED_DECIMAL_DEFAULT: &str = "0.5";
pub const BOX_PRICE_DEFAULT: u128 = 100;
pub const DENOM_DEFAULT: &str = "ustars";

pub const PAGINATION_MAX_LIMIT: u32 = 10_000;
pub const PAGINATION_DEFAULT_LIMIT: u32 = 1_000;

pub const TRANSFER_ADMIN_TIMEOUT: u64 = 7 * 24 * 3600;

pub const IS_LOCKED: Item<bool> = Item::new("is locked");
pub const TRANSFER_ADMIN_STATE: Item<TransferAdminState> = Item::new("transfer admin state");
pub const CONFIG: Item<Config> = Item::new("config");

pub const BOX_STATS: Item<BoxStats> = Item::new("box stats");
pub const BALANCE: Item<Balance> = Item::new("balance");

pub const NORMALIZED_DECIMAL: Item<Decimal> = Item::new("normalized decimal");

pub const USERS: Map<&Addr, UserInfo> = Map::new("users");

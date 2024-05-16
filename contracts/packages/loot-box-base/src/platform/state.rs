use cw_storage_plus::Item;

use crate::platform::types::{BoxList, Config, TransferAdminState};

pub const CONTRACT_NAME: &str = "cryptogopniks-loot-box";

pub const PROXY_ADDRESS: &str = "stars1atcndw8yfrulzux6vg6wtw2c0u4y5wvy9423255h472f4x3gn8dq0v8j45";

pub const BOX_PRICE: u128 = 100_000_000;
pub const BOX_LIST_LENGTH: u32 = 1_000;

pub const PAGINATION_MAX_LIMIT: u32 = 10_000;
pub const PAGINATION_DEFAULT_LIMIT: u32 = 1_000;

pub const TRANSFER_ADMIN_TIMEOUT: u64 = 7 * 24 * 3600;

pub const CONFIG: Item<Config> = Item::new("config");
pub const JOB_ID: Item<String> = Item::new("job id");
pub const TRANSFER_ADMIN_STATE: Item<TransferAdminState> = Item::new("transfer admin state");
pub const BOX_LIST: Item<BoxList> = Item::new("box list");

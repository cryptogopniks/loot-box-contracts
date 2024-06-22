use cosmwasm_std::{Addr, Deps, Env, StdResult};

use loot_box_base::treasury::{
    state::{BALANCE, CONFIG, PLATFORM_LIST, REMOVED_PLATFORM_LIST},
    types::{Balance, Config},
};

pub fn query_config(deps: Deps, _env: Env) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

pub fn query_balance(deps: Deps, _env: Env) -> StdResult<Balance> {
    BALANCE.load(deps.storage)
}

pub fn query_platform_list(deps: Deps, _env: Env) -> StdResult<Vec<Addr>> {
    PLATFORM_LIST.load(deps.storage)
}

pub fn query_removed_platform_list(deps: Deps, _env: Env) -> StdResult<Vec<Addr>> {
    REMOVED_PLATFORM_LIST.load(deps.storage)
}

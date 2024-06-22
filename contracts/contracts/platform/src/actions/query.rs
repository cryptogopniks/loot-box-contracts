use cosmwasm_std::{Deps, Env, Order, StdResult};
use cw_storage_plus::Bound;

use loot_box_base::platform::{
    msg::QueryUserListResponseItem,
    state::{BOX_STATS, CONFIG, PAGINATION_DEFAULT_LIMIT, PAGINATION_MAX_LIMIT, USERS},
    types::{BoxStats, Config, UserInfo},
};

pub fn query_config(deps: Deps, _env: Env) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

pub fn query_box_stats(deps: Deps, _env: Env) -> StdResult<BoxStats> {
    BOX_STATS.load(deps.storage)
}

pub fn query_user(deps: Deps, _env: Env, address: String) -> StdResult<UserInfo> {
    Ok(USERS
        .load(deps.storage, &deps.api.addr_validate(&address)?)
        .unwrap_or_default())
}

pub fn query_user_list(
    deps: Deps,
    _env: Env,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<QueryUserListResponseItem>> {
    let limit = limit
        .unwrap_or(PAGINATION_DEFAULT_LIMIT)
        .min(PAGINATION_MAX_LIMIT) as usize;

    let binding;
    let start_bound = match start_after {
        Some(addr) => {
            binding = deps.api.addr_validate(&addr)?;
            Some(Bound::exclusive(&binding))
        }
        None => None,
    };

    let users = USERS
        .range(deps.storage, start_bound, None, Order::Ascending)
        .take(limit)
        .map(|x| {
            let (address, info) = x.unwrap();

            QueryUserListResponseItem { address, info }
        })
        .collect::<Vec<QueryUserListResponseItem>>();

    Ok(users)
}

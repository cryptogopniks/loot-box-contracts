use cosmwasm_std::{Deps, Env, StdResult};

use loot_box_base::platform::{
    state::{BOX_LIST, CONFIG},
    types::{BoxList, Config},
};

pub fn query_config(deps: Deps, _env: Env) -> StdResult<Config> {
    CONFIG.load(deps.storage)
}

pub fn query_box_list(deps: Deps, _env: Env) -> StdResult<BoxList> {
    BOX_LIST.load(deps.storage)
}

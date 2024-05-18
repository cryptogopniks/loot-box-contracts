#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdResult,
};

use loot_box_base::{
    error::ContractError,
    platform::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
};

use crate::actions::{
    execute as e, instantiate::try_instantiate, other::migrate_contract, query as q,
};

/// Creates a new contract with the specified parameters packed in the "msg" variable
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    try_instantiate(deps, env, info, msg)
}

/// Exposes all the execute functions available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Buy {} => unimplemented!(),

        ExecuteMsg::Open {} => unimplemented!(),

        ExecuteMsg::Claim {} => unimplemented!(),

        ExecuteMsg::Send { amount, recipient } => unimplemented!(),

        ExecuteMsg::AcceptAdminRole {} => e::try_accept_admin_role(deps, env, info),

        ExecuteMsg::Deposit {} => unimplemented!(),

        ExecuteMsg::Withdraw {} => unimplemented!(),

        ExecuteMsg::UpdateConfig {
            admin,
            worker,
            box_price,
            denom,
            distribution,
        } => unimplemented!(),
    }
}

/// Exposes all the queries available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfig {} => to_json_binary(&q::query_config(deps, env)?),

        QueryMsg::QueryBoxStats {} => to_json_binary(&q::query_box_stats(deps, env)?),

        QueryMsg::QueryBalance {} => to_json_binary(&q::query_balance(deps, env)?),

        QueryMsg::QueryUser { address } => to_json_binary(&q::query_user(deps, env, address)?),

        QueryMsg::QueryUserList { start_after, limit } => {
            to_json_binary(&q::query_user_list(deps, env, start_after, limit)?)
        }
    }
}

/// Exposes all the replies available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    let Reply { id: _, result: _ } = reply;

    Err(ContractError::UndefinedReplyId)
}

/// Used for contract migration
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_contract(deps, env, msg)
}

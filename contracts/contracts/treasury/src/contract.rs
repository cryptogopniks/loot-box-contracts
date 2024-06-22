#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdResult,
};

use loot_box_base::{
    error::ContractError,
    treasury::{
        msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
        state::SAVE_PLATFORM_REPLY,
    },
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
        ExecuteMsg::IncreaseBalance {} => e::try_increase_balance(deps, env, info),

        ExecuteMsg::Send {
            amount,
            denom,
            recipient,
        } => e::try_send(deps, env, info, amount, denom, recipient),

        ExecuteMsg::IncreaseRewards { amount, denom } => {
            e::try_increase_rewards(deps, env, info, amount, denom)
        }

        ExecuteMsg::SendNft {
            collection,
            token_id,
            recipient,
        } => e::try_send_nft(deps, env, info, collection, token_id, recipient),

        ExecuteMsg::AcceptAdminRole {} => e::try_accept_admin_role(deps, env, info),

        ExecuteMsg::CreatePlatform {
            box_price,
            denom,
            distribution,
        } => e::try_create_platform(deps, env, info, box_price, denom, distribution),

        ExecuteMsg::AddPlatform { address } => e::try_add_platform(deps, env, info, address),

        ExecuteMsg::RemovePlatform { address } => e::try_remove_platform(deps, env, info, address),

        ExecuteMsg::Deposit {} => e::try_deposit(deps, env, info),

        ExecuteMsg::Withdraw { amount, denom } => e::try_withdraw(deps, env, info, amount, denom),

        ExecuteMsg::DepositNft { nft_info_list } => {
            e::try_deposit_nft(deps, env, info, nft_info_list)
        }

        ExecuteMsg::WithdrawNft { nft_info_list } => {
            e::try_withdraw_nft(deps, env, info, nft_info_list)
        }

        ExecuteMsg::UpdateNftPrice { nft_info_list } => {
            e::try_update_nft_price(deps, env, info, nft_info_list)
        }

        ExecuteMsg::UpdateConfig {
            admin,
            worker,
            platform_code_id,
        } => e::try_update_config(deps, env, info, admin, worker, platform_code_id),

        ExecuteMsg::Lock {} => e::try_lock(deps, env, info),

        ExecuteMsg::Unlock {} => e::try_unlock(deps, env, info),
    }
}

/// Exposes all the queries available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::QueryConfig {} => to_json_binary(&q::query_config(deps, env)?),

        QueryMsg::QueryBalance {} => to_json_binary(&q::query_balance(deps, env)?),

        QueryMsg::QueryPlatformList {} => to_json_binary(&q::query_platform_list(deps, env)?),

        QueryMsg::QueryRemovedPlatformList {} => {
            to_json_binary(&q::query_removed_platform_list(deps, env)?)
        }
    }
}

/// Exposes all the replies available in the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response, ContractError> {
    let Reply { id, result } = reply;

    match id {
        SAVE_PLATFORM_REPLY => e::save_platform_address(deps, env, &result),
        _ => Err(ContractError::UndefinedReplyId),
    }
}

/// Used for contract migration
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    migrate_contract(deps, env, msg)
}

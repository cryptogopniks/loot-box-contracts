use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw2::set_contract_version;

use loot_box_base::{
    error::ContractError,
    treasury::{
        msg::InstantiateMsg,
        state::{
            BALANCE, CONFIG, CONTRACT_NAME, IS_LOCKED, PLATFORM_CODE_ID, PLATFORM_LIST,
            REMOVED_PLATFORM_LIST, TRANSFER_ADMIN_STATE,
        },
        types::{Balance, Config, TransferAdminState},
    },
};

const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn try_instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let sender = &info.sender;
    let block_time = env.block.time.seconds();

    CONFIG.save(
        deps.storage,
        &Config {
            admin: sender.to_owned(),
            worker: msg
                .worker
                .map(|x| deps.api.addr_validate(&x))
                .transpose()
                .unwrap_or(Some(sender.to_owned())),
            platform_code_id: Some(msg.platform_code_id.unwrap_or(PLATFORM_CODE_ID)),
            denom_list: msg.denom_list.unwrap_or_default(),
        },
    )?;

    IS_LOCKED.save(deps.storage, &false)?;
    TRANSFER_ADMIN_STATE.save(
        deps.storage,
        &TransferAdminState {
            new_admin: sender.clone(),
            deadline: block_time,
        },
    )?;

    let mut balance = Balance::default();
    balance.pool = CONFIG
        .load(deps.storage)?
        .denom_list
        .into_iter()
        .map(|x| (Uint128::zero(), x))
        .collect();

    BALANCE.save(deps.storage, &balance)?;
    PLATFORM_LIST.save(deps.storage, &vec![])?;
    REMOVED_PLATFORM_LIST.save(deps.storage, &vec![])?;

    Ok(Response::new().add_attribute("action", "try_instantiate"))
}

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw2::set_contract_version;

use loot_box_base::{
    converters::str_to_dec,
    error::ContractError,
    platform::{
        msg::InstantiateMsg,
        state::{
            BOX_PRICE_DEFAULT, BOX_STATS, CONFIG, CONTRACT_NAME, DENOM_DEFAULT, IS_LOCKED,
            MEAN_WEIGHT, NORMALIZED_DECIMAL, TRANSFER_ADMIN_STATE,
        },
        types::{BoxStats, Config, TransferAdminState, WeightInfo},
    },
    utils::unwrap_field,
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

    // 20 % platform fee
    let distribution_default = vec![
        (0, "0.46"),
        (50, "0.40"),
        (250, "0.08"),
        (500, "0.04"),
        (1000, "0.02"),
    ];

    let worker = msg
        .worker
        .map(|x| deps.api.addr_validate(&x))
        .transpose()
        .unwrap_or(Some(sender.to_owned()));

    CONFIG.save(
        deps.storage,
        &Config {
            admin: unwrap_field(worker.clone(), "admin")?,
            worker,
            treasury: deps.api.addr_validate(&msg.treasury)?,
            box_price: msg.box_price.unwrap_or(Uint128::new(BOX_PRICE_DEFAULT)),
            denom: msg.denom.unwrap_or(DENOM_DEFAULT.to_string()),
            distribution: msg.distribution.unwrap_or(
                distribution_default
                    .into_iter()
                    .map(|(rewards, weight)| WeightInfo {
                        box_rewards: Uint128::new(rewards),
                        weight: str_to_dec(weight),
                    })
                    .collect(),
            ),
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

    BOX_STATS.save(deps.storage, &BoxStats::default())?;

    NORMALIZED_DECIMAL.save(deps.storage, &str_to_dec(MEAN_WEIGHT))?;

    Ok(Response::new().add_attribute("action", "try_instantiate"))
}

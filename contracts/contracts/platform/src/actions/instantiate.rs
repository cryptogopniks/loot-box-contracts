use cosmwasm_std::{Decimal, DepsMut, Env, MessageInfo, Response, Uint128};
use cw2::set_contract_version;

use loot_box_base::{
    converters::str_to_dec,
    error::ContractError,
    platform::{
        msg::InstantiateMsg,
        state::{BOX_LIST, BOX_LIST_LENGTH, BOX_PRICE, CONFIG, CONTRACT_NAME, NORMALIZED_DECIMAL},
        types::{BoxList, Config},
    },
};

const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn try_instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let sender = &info.sender;

    CONFIG.save(
        deps.storage,
        &Config {
            admin: sender.to_owned(),
            worker: msg
                .worker
                .map(|x| deps.api.addr_validate(&x))
                .transpose()
                .unwrap_or(Some(sender.to_owned())),
            box_price: msg.box_price.unwrap_or(Uint128::new(BOX_PRICE)),
            price_and_weight_list: msg
                .price_and_weight_list
                .unwrap_or(vec![(Uint128::new(BOX_PRICE), Decimal::one())]),
            box_list_length: msg.box_list_length.unwrap_or(BOX_LIST_LENGTH),
        },
    )?;

    BOX_LIST.save(deps.storage, &BoxList::default())?;
    NORMALIZED_DECIMAL.save(deps.storage, &str_to_dec("0.5"))?;

    Ok(Response::new().add_attribute("action", "try_instantiate"))
}

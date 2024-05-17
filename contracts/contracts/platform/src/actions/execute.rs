use cosmwasm_std::{
    to_json_binary, Addr, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
    WasmMsg,
};

use loot_box_base::{
    converters::u128_to_dec,
    error::ContractError,
    platform::{
        state::{BOX_LIST, CONFIG, JOB_ID, TRANSFER_ADMIN_STATE, TRANSFER_ADMIN_TIMEOUT},
        types::{BoxList, Config, TransferAdminState},
    },
    utils::{check_funds, unwrap_field, AuthType, FundsType},
};

pub fn try_nois_receive(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    callback: nois::NoisCallback,
) -> Result<Response, ContractError> {
    let block_time = env.block.time.seconds();
    let sender_address = info.sender;
    let Config { proxy, .. } = CONFIG.load(deps.storage)?;
    let proxy = unwrap_field(proxy, "proxy")?;

    check_authorization(
        deps.as_ref(),
        &sender_address,
        AuthType::Specified {
            allowlist: vec![Some(proxy)],
        },
    )?;

    let nois::NoisCallback { randomness, .. } = callback;
    let randomness: [u8; 32] = randomness
        .to_array()
        .map_err(|_| ContractError::InvalidRandomness)?;

    BOX_LIST.update(deps.storage, |x| -> StdResult<BoxList> {
        Ok(BoxList {
            update_date: block_time,
            price_list: nois::shuffle(randomness, x.price_list),
        })
    })?;

    JOB_ID.remove(deps.storage);

    Ok(Response::new().add_attribute("action", "try_nois_receive"))
}

pub fn try_request_box_list(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let sender_address = info.sender;
    let Config {
        proxy,
        price_and_weight_list,
        box_list_length,
        ..
    } = CONFIG.load(deps.storage)?;
    let proxy = unwrap_field(proxy, "proxy")?;

    check_authorization(deps.as_ref(), &sender_address, AuthType::Admin)?;

    let job_id = format!("{}{}", sender_address.to_string(), env.block.time.seconds());
    validate_job_id(&job_id)?;

    // don't allow new request before the data will be received
    if JOB_ID.may_load(deps.storage)?.is_some() {
        Err(ContractError::JobIsInProgress)?;
    }

    JOB_ID.save(deps.storage, &job_id)?;

    // generate list of boxes with different prices according to weights
    let mut box_price_list: Vec<Uint128> = vec![];

    for (price, weight) in price_and_weight_list {
        let box_amount = (weight * u128_to_dec(box_list_length))
            .to_uint_floor()
            .u128() as usize;
        let same_box_price_list = vec![price; box_amount];
        box_price_list = [box_price_list, same_box_price_list].concat();
    }

    if box_price_list.len() != box_list_length as usize {
        Err(ContractError::IncorrectBoxPriceListLength)?;
    }

    // TODO: use separate storage
    BOX_LIST.update(deps.storage, |mut x| -> StdResult<BoxList> {
        x.price_list = box_price_list;
        Ok(x)
    })?;

    // send msg to proxy
    let msg = WasmMsg::Execute {
        contract_addr: proxy.to_string(),
        msg: to_json_binary(&nois::ProxyExecuteMsg::GetNextRandomness { job_id })?,
        funds: info.funds,
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "try_request_box_list"))
}

pub fn try_accept_admin_role(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let block_time = env.block.time.seconds();
    let TransferAdminState {
        new_admin,
        deadline,
    } = TRANSFER_ADMIN_STATE.load(deps.storage)?;

    check_authorization(
        deps.as_ref(),
        &sender_address,
        AuthType::Specified {
            allowlist: vec![Some(new_admin)],
        },
    )?;

    if block_time >= deadline {
        Err(ContractError::TransferAdminDeadline)?;
    }

    CONFIG.update(deps.storage, |mut x| -> StdResult<Config> {
        x.admin = sender_address;
        Ok(x)
    })?;

    TRANSFER_ADMIN_STATE.update(deps.storage, |mut x| -> StdResult<TransferAdminState> {
        x.deadline = block_time;
        Ok(x)
    })?;

    Ok(Response::new().add_attribute("action", "try_accept_admin_role"))
}

pub fn try_update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    admin: Option<String>,
    worker: Option<String>,
    proxy: Option<String>,
    box_price: Option<Uint128>,
    price_and_weight_list: Option<Vec<(Uint128, Decimal)>>,
    box_list_length: Option<u32>,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    let mut config = CONFIG.load(deps.storage)?;
    let mut is_config_updated = false;

    if let Some(x) = admin {
        check_authorization(deps.as_ref(), &sender_address, AuthType::Admin)?;
        let block_time = env.block.time.seconds();
        let new_admin = &deps.api.addr_validate(&x)?;

        TRANSFER_ADMIN_STATE.save(
            deps.storage,
            &TransferAdminState {
                new_admin: new_admin.to_owned(),
                deadline: block_time + TRANSFER_ADMIN_TIMEOUT,
            },
        )?;

        is_config_updated = true;
    }

    if let Some(x) = worker {
        check_authorization(deps.as_ref(), &sender_address, AuthType::Admin)?;
        config.worker = Some(deps.api.addr_validate(&x)?);
        is_config_updated = true;
    }

    if let Some(x) = proxy {
        check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;
        config.proxy = Some(deps.api.addr_validate(&x)?);
        is_config_updated = true;
    }

    if let Some(x) = box_price {
        check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;
        config.box_price = x;
        is_config_updated = true;
    }

    if let Some(x) = price_and_weight_list {
        check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;
        // TODO: add weight checker
        config.price_and_weight_list = x;
        is_config_updated = true;
    }

    if let Some(x) = box_list_length {
        check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;
        config.box_list_length = x;
        is_config_updated = true;
    }

    // don't allow empty messages
    if !is_config_updated {
        Err(ContractError::NoParameters)?;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "try_update_config"))
}

fn check_authorization(deps: Deps, sender: &Addr, auth_type: AuthType) -> StdResult<()> {
    let Config { admin, worker, .. } = CONFIG.load(deps.storage)?;
    let worker = unwrap_field(worker, "worker");

    match auth_type {
        AuthType::Any => {}
        AuthType::Admin => {
            if sender != admin {
                Err(ContractError::Unauthorized)?;
            }
        }
        AuthType::AdminOrWorker => {
            if !((sender == admin) || (worker.is_ok() && sender == worker?)) {
                Err(ContractError::Unauthorized)?;
            }
        }
        AuthType::Specified { allowlist } => {
            let is_included = allowlist.iter().any(|some_address| {
                if let Some(x) = some_address {
                    if sender == x {
                        return true;
                    }
                }

                false
            });

            if !is_included {
                Err(ContractError::Unauthorized)?;
            }
        }
        AuthType::AdminOrWorkerOrSpecified { allowlist } => {
            let is_included = allowlist.iter().any(|some_address| {
                if let Some(x) = some_address {
                    if sender == x {
                        return true;
                    }
                }

                false
            });

            if !((sender == admin) || (worker.is_ok() && sender == worker?) || is_included) {
                Err(ContractError::Unauthorized)?;
            }
        }
        AuthType::AdminOrSpecified { allowlist } => {
            let is_included = allowlist.iter().any(|some_address| {
                if let Some(x) = some_address {
                    if sender == x {
                        return true;
                    }
                }

                false
            });

            if !((sender == admin) || is_included) {
                Err(ContractError::Unauthorized)?;
            }
        }
    };

    Ok(())
}

fn validate_job_id(job_id: &str) -> Result<(), ContractError> {
    if job_id.len() > nois::MAX_JOB_ID_LEN {
        Err(ContractError::JobIdTooLong)
    } else {
        Ok(())
    }
}

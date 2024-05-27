use cosmwasm_std::{
    coins, to_json_binary, Addr, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response,
    StdResult, SubMsg, SubMsgResult, Uint128, WasmMsg,
};

use loot_box_base::{
    error::ContractError,
    platform,
    treasury::{
        state::{
            BALANCE, CONFIG, IS_LOCKED, PLATFORM_LIST, REMOVED_PLATFORM_LIST, SAVE_PLATFORM_REPLY,
            TRANSFER_ADMIN_STATE, TRANSFER_ADMIN_TIMEOUT,
        },
        types::{Balance, Config, NftInfo, TransferAdminState},
    },
    utils::{check_funds, unwrap_field, AuthType, FundsType},
};

use crate::helpers::{check_authorization, check_collections_holder, check_lockout};

pub fn try_increase_balance(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, asset_amount, asset_info) = check_funds(
        deps.as_ref(),
        &info,
        FundsType::Single {
            sender: None,
            amount: None,
        },
    )?;
    check_authorization(
        deps.as_ref(),
        &sender_address,
        AuthType::Specified {
            allowlist: PLATFORM_LIST
                .load(deps.storage)?
                .into_iter()
                .map(Some)
                .collect(),
        },
    )?;

    let denom = asset_info.try_get_native()?;
    BALANCE.update(deps.storage, |mut x| -> StdResult<Balance> {
        if x.pool
            .iter()
            .all(|(_current_amount, current_denom)| current_denom != &denom)
        {
            x.pool.push((Uint128::zero(), denom.clone()));
        }

        x.pool = x
            .pool
            .into_iter()
            .map(|(current_amount, current_denom)| {
                if current_denom == denom {
                    return (current_amount + asset_amount, current_denom);
                }

                (current_amount, current_denom)
            })
            .collect();

        Ok(x)
    })?;

    Ok(Response::new().add_attribute("action", "try_increase_balance"))
}

pub fn try_send(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    denom: String,
    recipient: String,
) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(
        deps.as_ref(),
        &sender_address,
        AuthType::Specified {
            allowlist: PLATFORM_LIST
                .load(deps.storage)?
                .into_iter()
                .map(Some)
                .collect(),
        },
    )?;

    BALANCE.update(deps.storage, |mut x| -> StdResult<Balance> {
        x.pool = x
            .pool
            .into_iter()
            .map(|(current_amount, current_denom)| {
                if current_denom == denom {
                    return (current_amount - amount, current_denom);
                }

                (current_amount, current_denom)
            })
            .collect();

        x.rewards = x
            .rewards
            .into_iter()
            .map(|(current_amount, current_denom)| {
                if current_denom == denom {
                    return (current_amount - amount, current_denom);
                }

                (current_amount, current_denom)
            })
            .collect();

        Ok(x)
    })?;

    let msg = BankMsg::Send {
        to_address: recipient,
        amount: coins(amount.u128(), denom),
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "try_send"))
}

pub fn try_increase_rewards(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    denom: String,
) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(
        deps.as_ref(),
        &sender_address,
        AuthType::Specified {
            allowlist: PLATFORM_LIST
                .load(deps.storage)?
                .into_iter()
                .map(Some)
                .collect(),
        },
    )?;

    BALANCE.update(deps.storage, |mut x| -> StdResult<Balance> {
        if x.rewards
            .iter()
            .all(|(_current_amount, current_denom)| current_denom != &denom)
        {
            x.rewards.push((Uint128::zero(), denom.clone()));
        }

        x.rewards = x
            .rewards
            .into_iter()
            .map(|(current_amount, current_denom)| {
                if current_denom == denom {
                    return (current_amount + amount, current_denom);
                }

                (current_amount, current_denom)
            })
            .collect();

        Ok(x)
    })?;

    Ok(Response::new().add_attribute("action", "try_increase_rewards"))
}

pub fn try_send_nft(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    collection: String,
    token_id: String,
    recipient: String,
) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(
        deps.as_ref(),
        &sender_address,
        AuthType::Specified {
            allowlist: PLATFORM_LIST
                .load(deps.storage)?
                .into_iter()
                .map(Some)
                .collect(),
        },
    )?;

    BALANCE.update(deps.storage, |mut x| -> StdResult<Balance> {
        x.nft_pool
            .retain(|y| !(y.collection == collection && y.token_id == token_id));

        Ok(x)
    })?;

    let msg = WasmMsg::Execute {
        contract_addr: collection,
        msg: to_json_binary(&cw721::Cw721ExecuteMsg::TransferNft {
            recipient,
            token_id,
        })?,
        funds: vec![],
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "try_send_nft"))
}

pub fn try_accept_admin_role(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
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

pub fn try_create_platform(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut inst_msg: platform::msg::InstantiateMsg,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;

    inst_msg.treasury = env.contract.address.to_string();

    let msg: CosmosMsg = CosmosMsg::Wasm(WasmMsg::Instantiate {
        admin: Some(sender_address.to_string()),
        code_id: unwrap_field(
            CONFIG.load(deps.storage)?.platform_code_id,
            "platform_code_id",
        )?,
        label: "loot-box-platform".to_string(),
        msg: to_json_binary(&inst_msg)?,
        funds: vec![],
    });

    let submsg = SubMsg::reply_on_success(msg, SAVE_PLATFORM_REPLY);

    Ok(Response::new()
        .add_submessage(submsg)
        .add_attribute("action", "try_create_platform"))
}

pub fn save_platform_address(
    deps: DepsMut,
    _env: Env,
    result: &SubMsgResult,
) -> Result<Response, ContractError> {
    let res = result
        .to_owned()
        .into_result()
        .map_err(|e| ContractError::CustomError { val: e })?;

    let instantiate_event = unwrap_field(
        res.events.iter().find(|x| x.ty == "instantiate"),
        "instantiate_event",
    )?;

    let platform_address = &unwrap_field(
        instantiate_event
            .attributes
            .iter()
            .find(|x| x.key == "_contract_address"),
        "platform_address",
    )?
    .value;

    PLATFORM_LIST.update(deps.storage, |mut x| -> StdResult<Vec<Addr>> {
        x.push(deps.api.addr_validate(platform_address)?);

        Ok(x)
    })?;

    // update treasury config
    let platform::types::Config {
        denom: platform_denom,
        ..
    } = deps
        .querier
        .query_wasm_smart(platform_address, &platform::msg::QueryMsg::QueryConfig {})?;

    CONFIG.update(deps.storage, |mut x| -> StdResult<Config> {
        if !x.denom_list.contains(&platform_denom) {
            x.denom_list.push(platform_denom);
        }

        Ok(x)
    })?;

    Ok(Response::new().add_attribute("platform_address", platform_address))
}

pub fn try_add_platform(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;

    let platform = deps.api.addr_validate(&address)?;

    REMOVED_PLATFORM_LIST.update(deps.storage, |mut x| -> StdResult<Vec<Addr>> {
        if !x.contains(&platform) {
            Err(ContractError::PlatformIsNotFound)?;
        }

        x.retain(|y| y != platform);

        Ok(x)
    })?;

    PLATFORM_LIST.update(deps.storage, |mut x| -> StdResult<Vec<Addr>> {
        if x.contains(&platform) {
            Err(ContractError::PlatformDuplication)?;
        }

        x.push(platform);

        Ok(x)
    })?;

    Ok(Response::new().add_attribute("action", "try_add_platform"))
}

pub fn try_remove_platform(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;

    let platform = deps.api.addr_validate(&address)?;

    PLATFORM_LIST.update(deps.storage, |mut x| -> StdResult<Vec<Addr>> {
        if !x.contains(&platform) {
            Err(ContractError::PlatformIsNotFound)?;
        }

        x.retain(|y| y != platform);

        Ok(x)
    })?;

    REMOVED_PLATFORM_LIST.update(deps.storage, |mut x| -> StdResult<Vec<Addr>> {
        if x.contains(&platform) {
            Err(ContractError::PlatformDuplication)?;
        }

        x.push(platform);

        Ok(x)
    })?;

    Ok(Response::new().add_attribute("action", "try_remove_platform"))
}

pub fn try_deposit(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let (sender_address, asset_amount, asset_info) = check_funds(
        deps.as_ref(),
        &info,
        FundsType::Single {
            sender: None,
            amount: None,
        },
    )?;
    check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;

    let denom = asset_info.try_get_native()?;
    let Config { denom_list, .. } = CONFIG.load(deps.storage)?;

    // check fund denom
    if !denom_list.contains(&denom) {
        Err(ContractError::WrongAssetType)?;
    }

    BALANCE.update(deps.storage, |mut x| -> StdResult<Balance> {
        if x.pool
            .iter()
            .all(|(_current_amount, current_denom)| current_denom != &denom)
        {
            x.pool.push((Uint128::zero(), denom.clone()));
        }

        x.pool = x
            .pool
            .into_iter()
            .map(|(current_amount, current_denom)| {
                if current_denom == denom {
                    return (current_amount + asset_amount, current_denom);
                }

                (current_amount, current_denom)
            })
            .collect();

        if x.deposited
            .iter()
            .all(|(_current_amount, current_denom)| current_denom != &denom)
        {
            x.deposited.push((Uint128::zero(), denom.clone()));
        }

        x.deposited = x
            .deposited
            .into_iter()
            .map(|(current_amount, current_denom)| {
                if current_denom == denom {
                    return (current_amount + asset_amount, current_denom);
                }

                (current_amount, current_denom)
            })
            .collect();

        Ok(x)
    })?;

    Ok(Response::new().add_attribute("action", "try_deposit"))
}

pub fn try_withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    denom: String,
) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;

    if amount.is_zero() {
        Err(ContractError::ZeroAmount)?;
    }

    let Config { worker, .. } = CONFIG.load(deps.storage)?;
    check_authorization(
        deps.as_ref(),
        &sender_address,
        AuthType::Specified {
            allowlist: vec![worker],
        },
    )?;

    BALANCE.update(deps.storage, |mut x| -> StdResult<Balance> {
        let (mut pool_amount, _pool_denom) = x
            .pool
            .iter()
            .cloned()
            .find(|(_amount, pool_denom)| pool_denom == &denom)
            .ok_or(ContractError::AssetIsNotFound)?;

        let (mut deposited_amount, _deposited_denom) = x
            .deposited
            .iter()
            .cloned()
            .find(|(_amount, deposited_denom)| deposited_denom == &denom)
            .ok_or(ContractError::AssetIsNotFound)?;

        if amount > pool_amount {
            Err(ContractError::NotEnoughLiquidity)?;
        }

        pool_amount -= amount;

        if amount > deposited_amount {
            deposited_amount = Uint128::zero();
        } else {
            deposited_amount -= amount;
        }

        x.pool = x
            .pool
            .into_iter()
            .map(|(amount, pool_denom)| {
                if pool_denom == denom {
                    return (pool_amount, pool_denom);
                }

                (amount, pool_denom)
            })
            .collect();

        x.deposited = x
            .deposited
            .into_iter()
            .map(|(amount, deposited_denom)| {
                if deposited_denom == denom {
                    return (deposited_amount, deposited_denom);
                }

                (amount, deposited_denom)
            })
            .collect();

        Ok(x)
    })?;

    let msg = BankMsg::Send {
        to_address: sender_address.to_string(),
        amount: coins(amount.u128(), denom),
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "try_withdraw"))
}

pub fn try_deposit_nft(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    nft_info_list: Vec<NftInfo<String>>,
) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;

    let mut response = Response::new().add_attribute("action", "try_deposit_nft");
    let platform_list = PLATFORM_LIST.load(deps.storage)?;

    let mut distribution_list: Vec<(Uint128, String)> = vec![];

    for platform in platform_list {
        let platform::types::Config {
            denom,
            distribution,
            ..
        } = deps
            .querier
            .query_wasm_smart(&platform, &platform::msg::QueryMsg::QueryConfig {})?;

        for weight_info in distribution {
            distribution_list.push((weight_info.box_rewards, denom.clone()));
        }
    }

    if nft_info_list.is_empty() {
        Err(ContractError::EmptyCollectionList)?;
    }

    // check collection addresses
    let nft_info_list = nft_info_list
        .into_iter()
        .map(|x| {
            Ok(NftInfo {
                collection: deps.api.addr_validate(&x.collection)?,
                token_id: x.token_id,
                price_option: x.price_option,
            })
        })
        .collect::<StdResult<Vec<NftInfo<Addr>>>>()?;

    // check nft duplication
    let mut collection_and_token_id_list: Vec<String> = nft_info_list
        .iter()
        .map(|x| format!("{}{}", x.collection, x.token_id))
        .collect();

    collection_and_token_id_list.sort_unstable();
    collection_and_token_id_list.dedup();

    if collection_and_token_id_list.len() != nft_info_list.len() {
        Err(ContractError::NftDuplication)?;
    }

    // check nft prices
    if !nft_info_list.iter().all(|x| {
        x.price_option.iter().all(|(nft_price, nft_denom)| {
            distribution_list
                .iter()
                .any(|(amount, denom)| amount == nft_price && denom == nft_denom)
        })
    }) {
        Err(ContractError::ImproperNftPrice)?;
    }

    if nft_info_list.iter().any(|x| {
        x.price_option
            .iter()
            .any(|(amount, _denom)| amount.is_zero())
    }) {
        Err(ContractError::ImproperNftPrice)?;
    }

    // check if nfts are on user balance
    check_collections_holder(deps.as_ref(), &sender_address, &nft_info_list)?;

    // add transfer msgs
    for NftInfo {
        collection,
        token_id,
        ..
    } in &nft_info_list
    {
        let cw721_msg = cw721::Cw721ExecuteMsg::TransferNft {
            recipient: env.contract.address.to_string(),
            token_id: token_id.to_string(),
        };

        let msg = WasmMsg::Execute {
            contract_addr: collection.to_string(),
            msg: to_json_binary(&cw721_msg)?,
            funds: vec![],
        };

        response = response.add_message(msg);
    }

    BALANCE.update(deps.storage, |mut x| -> StdResult<Balance> {
        x.nft_pool = [x.nft_pool, nft_info_list].concat();

        Ok(x)
    })?;

    Ok(response)
}

pub fn try_withdraw_nft(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    nft_info_list: Vec<NftInfo<String>>,
) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;

    let mut response = Response::new().add_attribute("action", "try_withdraw_nft");
    let mut balance = BALANCE.load(deps.storage)?;
    let Config { worker, .. } = CONFIG.load(deps.storage)?;
    check_authorization(
        deps.as_ref(),
        &sender_address,
        AuthType::Specified {
            allowlist: vec![worker],
        },
    )?;

    if nft_info_list.is_empty() {
        Err(ContractError::EmptyCollectionList)?;
    }

    // check collection addresses
    let nft_info_list = nft_info_list
        .into_iter()
        .map(|x| {
            Ok(NftInfo {
                collection: deps.api.addr_validate(&x.collection)?,
                token_id: x.token_id,
                price_option: x.price_option,
            })
        })
        .collect::<StdResult<Vec<NftInfo<Addr>>>>()?;

    // check nft duplication
    let mut collection_and_token_id_list: Vec<String> = nft_info_list
        .iter()
        .map(|x| format!("{}{}", x.collection, x.token_id))
        .collect();

    collection_and_token_id_list.sort_unstable();
    collection_and_token_id_list.dedup();

    if collection_and_token_id_list.len() != nft_info_list.len() {
        Err(ContractError::NftDuplication)?;
    }

    // check if nfts are available
    if nft_info_list.iter().any(|x| !balance.nft_pool.contains(x)) {
        Err(ContractError::NftIsNotFound)?;
    }

    // add transfer msgs
    for NftInfo {
        collection,
        token_id,
        ..
    } in &nft_info_list
    {
        let cw721_msg = cw721::Cw721ExecuteMsg::TransferNft {
            recipient: sender_address.to_string(),
            token_id: token_id.to_string(),
        };

        let msg = WasmMsg::Execute {
            contract_addr: collection.to_string(),
            msg: to_json_binary(&cw721_msg)?,
            funds: vec![],
        };

        response = response.add_message(msg);
    }

    balance.nft_pool.retain(|x| !nft_info_list.contains(x));
    BALANCE.save(deps.storage, &balance)?;

    Ok(response)
}

pub fn try_update_nft_price(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    nft_info_list: Vec<NftInfo<String>>,
) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;

    let mut balance = BALANCE.load(deps.storage)?;
    let Config { worker, .. } = CONFIG.load(deps.storage)?;
    check_authorization(
        deps.as_ref(),
        &sender_address,
        AuthType::Specified {
            allowlist: vec![worker],
        },
    )?;

    if nft_info_list.is_empty() {
        Err(ContractError::EmptyCollectionList)?;
    }

    // check collection addresses
    let nft_info_list = nft_info_list
        .into_iter()
        .map(|x| {
            Ok(NftInfo {
                collection: deps.api.addr_validate(&x.collection)?,
                token_id: x.token_id,
                price_option: x.price_option,
            })
        })
        .collect::<StdResult<Vec<NftInfo<Addr>>>>()?;

    // check nft duplication
    let mut collection_and_token_id_list: Vec<String> = nft_info_list
        .iter()
        .map(|x| format!("{}{}", x.collection, x.token_id))
        .collect();

    collection_and_token_id_list.sort_unstable();
    collection_and_token_id_list.dedup();

    if collection_and_token_id_list.len() != nft_info_list.len() {
        Err(ContractError::NftDuplication)?;
    }

    // check nft prices
    if nft_info_list.iter().any(|x| {
        x.price_option
            .iter()
            .any(|(amount, _denom)| amount.is_zero())
    }) {
        Err(ContractError::ImproperNftPrice)?;
    }

    // check if nfts are available
    if nft_info_list.iter().any(|x| !balance.nft_pool.contains(x)) {
        Err(ContractError::NftIsNotFound)?;
    }

    // update prices
    balance.nft_pool = balance
        .nft_pool
        .into_iter()
        .map(|mut x| {
            if let Some(y) = nft_info_list.iter().find(|y| y == &&x) {
                x.price_option = y.price_option.clone();
            }

            x
        })
        .collect();
    BALANCE.save(deps.storage, &balance)?;

    Ok(Response::new().add_attribute("action", "try_update_nft_price"))
}

pub fn try_update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    admin: Option<String>,
    worker: Option<String>,
    platform_code_id: Option<u64>,
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
        check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;
        config.worker = Some(deps.api.addr_validate(&x)?);
        is_config_updated = true;
    }

    if let Some(x) = platform_code_id {
        check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;
        config.platform_code_id = Some(x);
        is_config_updated = true;
    }

    // don't allow empty messages
    if !is_config_updated {
        Err(ContractError::NoParameters)?;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "try_update_config"))
}

pub fn try_lock(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;

    IS_LOCKED.save(deps.storage, &true)?;

    Ok(Response::new().add_attributes([("action", "try_lock")]))
}

pub fn try_unlock(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;

    IS_LOCKED.save(deps.storage, &false)?;

    Ok(Response::new().add_attributes([("action", "try_unlock")]))
}

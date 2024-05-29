use cosmwasm_std::{
    coins, to_json_binary, Addr, Decimal, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
    WasmMsg,
};

use hashing_helper::base::calc_hash_bytes;
use loot_box_base::{
    converters::address_to_salt,
    error::ContractError,
    hash_generator::types::Hash,
    platform::{
        state::{
            BOX_STATS, CONFIG, IS_LOCKED, NORMALIZED_DECIMAL, OPENING_COOLDOWN,
            TRANSFER_ADMIN_STATE, TRANSFER_ADMIN_TIMEOUT, USERS,
        },
        types::{BoxStats, Config, OpeningInfo, TransferAdminState, UserInfo, WeightInfo},
    },
    utils::{check_funds, AuthType, FundsType},
};

use crate::helpers::{check_authorization, check_lockout, pick_rewards};

pub fn try_buy(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, asset_amount, asset_info) = check_funds(
        deps.as_ref(),
        &info,
        FundsType::Single {
            sender: None,
            amount: None,
        },
    )?;
    check_authorization(deps.as_ref(), &sender_address, AuthType::Any)?;

    let Config {
        treasury,
        box_price,
        denom,
        ..
    } = CONFIG.load(deps.storage)?;

    // check if platform is in platfrom list
    let platfrom_list = deps.querier.query_wasm_smart::<Vec<Addr>>(
        &treasury,
        &loot_box_base::treasury::msg::QueryMsg::QueryPlatformList {},
    )?;
    if !platfrom_list.contains(&env.contract.address) {
        Err(ContractError::PlatformIsNotInList)?;
    }

    // check fund denom
    if asset_info.try_get_native()? != denom {
        Err(ContractError::WrongAssetType)?;
    }

    if asset_amount.is_zero() {
        Err(ContractError::ZeroAmount)?;
    }

    // check fund amount
    if !(asset_amount % box_price).is_zero() {
        Err(ContractError::ImproperAssetAmount)?;
    };

    let box_amount = asset_amount / box_price;

    USERS.update(deps.storage, &sender_address, |x| -> StdResult<UserInfo> {
        let mut user = x.unwrap_or_default();
        user.bought += box_amount;
        user.boxes += box_amount;

        Ok(user)
    })?;

    BOX_STATS.update(deps.storage, |mut x| -> StdResult<BoxStats> {
        x.sold += box_amount;

        Ok(x)
    })?;

    let msg = WasmMsg::Execute {
        contract_addr: treasury.to_string(),
        msg: to_json_binary(&loot_box_base::treasury::msg::ExecuteMsg::IncreaseBalance {})?,
        funds: coins(asset_amount.u128(), denom),
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "try_buy")
        .add_attribute("box_amount", box_amount.u128().to_string()))
}

pub fn try_open(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(deps.as_ref(), &sender_address, AuthType::Any)?;

    let mut response = Response::new().add_attribute("action", "try_open");
    let block_time = env.block.time.seconds();
    let normalized_decimal = NORMALIZED_DECIMAL.load(deps.storage)?;
    let Config {
        treasury,
        denom,
        distribution,
        ..
    } = CONFIG.load(deps.storage)?;
    let mut user = USERS
        .load(deps.storage, &sender_address)
        .unwrap_or_default();

    // don't allow to open multiple boxes in single tx
    if block_time < user.opening_date + OPENING_COOLDOWN {
        Err(ContractError::MultipleBoxesPerTx)?;
    }

    // check box amount
    if user.boxes.is_zero() {
        Err(ContractError::ZeroBoxAmount)?;
    }

    // get random rewards
    let password = &format!("{}{}", normalized_decimal, env.block.time.nanos());
    let salt = &address_to_salt(&sender_address);
    let hash_bytes = calc_hash_bytes(password, salt)?;
    let random_weight = Hash::from(hash_bytes).to_norm_dec();
    let rewards = pick_rewards(&distribution, random_weight);

    NORMALIZED_DECIMAL.save(deps.storage, &random_weight)?;

    // get treasury balance
    let balance = deps
        .querier
        .query_wasm_smart::<loot_box_base::treasury::types::Balance>(
            &treasury,
            &loot_box_base::treasury::msg::QueryMsg::QueryBalance {},
        )?;

    let (pool_amount, _) = balance
        .pool
        .iter()
        .find(|(_current_amount, current_denom)| current_denom == &denom)
        .ok_or(ContractError::AssetIsNotFound)?;

    let same_price_nft = balance.nft_pool.iter().find(|x| {
        x.price_option
            .iter()
            .any(|(nft_price, nft_denom)| nft_price == rewards && nft_denom == &denom)
    });

    let last_digit = random_weight
        .to_string()
        .chars()
        .last()
        .unwrap_or('0')
        .to_string()
        .parse::<u8>()
        .unwrap_or_default();

    match same_price_nft {
        Some(x) if last_digit % 2 == 1 => {
            // send nft
            let msg = WasmMsg::Execute {
                contract_addr: treasury.to_string(),
                msg: to_json_binary(&loot_box_base::treasury::msg::ExecuteMsg::SendNft {
                    collection: x.collection.to_string(),
                    token_id: x.token_id.to_string(),
                    recipient: sender_address.to_string(),
                })?,
                funds: vec![],
            };

            response = response
                .add_message(msg)
                .add_attribute("nft", rewards.u128().to_string())
                .add_attribute("collection", x.collection.to_string())
                .add_attribute("token_id", x.token_id.to_string());
        }
        _ => {
            // send rewards if balance is enough else accumulate rewards
            if &rewards <= pool_amount {
                if !rewards.is_zero() {
                    let msg = WasmMsg::Execute {
                        contract_addr: treasury.to_string(),
                        msg: to_json_binary(&loot_box_base::treasury::msg::ExecuteMsg::Send {
                            amount: rewards,
                            denom,
                            recipient: sender_address.to_string(),
                        })?,
                        funds: vec![],
                    };

                    response = response.add_message(msg)
                }

                response = response.add_attribute("coins", rewards.u128().to_string());
            } else {
                user.rewards += rewards;

                let msg = WasmMsg::Execute {
                    contract_addr: treasury.to_string(),
                    msg: to_json_binary(
                        &loot_box_base::treasury::msg::ExecuteMsg::IncreaseRewards {
                            amount: rewards,
                            denom,
                        },
                    )?,
                    funds: vec![],
                };

                response = response
                    .add_message(msg)
                    .add_attribute("rewards", rewards.u128().to_string());
            }
        }
    }

    user.opening_date = block_time;
    user.boxes -= Uint128::one();

    if !user.opened.iter().any(|x| x.box_rewards == rewards) {
        user.opened.push(OpeningInfo {
            box_rewards: rewards,
            opened: Uint128::zero(),
        });
    }

    user.opened = user
        .opened
        .into_iter()
        .map(|mut x| {
            if x.box_rewards == rewards {
                x.opened += Uint128::one();
            }

            x
        })
        .collect();

    USERS.save(deps.storage, &sender_address, &user)?;

    BOX_STATS.update(deps.storage, |mut x| -> StdResult<BoxStats> {
        if !x.opened.iter().any(|y| y.box_rewards == rewards) {
            x.opened.push(OpeningInfo {
                box_rewards: rewards,
                opened: Uint128::zero(),
            });
        }

        x.opened = x
            .opened
            .into_iter()
            .map(|mut x| {
                if x.box_rewards == rewards {
                    x.opened += Uint128::one();
                }

                x
            })
            .collect();

        Ok(x)
    })?;

    Ok(response)
}

pub fn try_claim(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(deps.as_ref(), &sender_address, AuthType::Any)?;

    let Config {
        treasury, denom, ..
    } = CONFIG.load(deps.storage)?;
    let mut user = USERS
        .load(deps.storage, &sender_address)
        .unwrap_or_default();

    // check rewards
    if user.rewards.is_zero() {
        Err(ContractError::ZeroRewardsAmount)?;
    }

    // check treasury balance
    let loot_box_base::treasury::types::Balance { pool, .. } = deps.querier.query_wasm_smart(
        &treasury,
        &loot_box_base::treasury::msg::QueryMsg::QueryBalance {},
    )?;

    let (pool_amount, _) = pool
        .iter()
        .find(|(_current_amount, current_denom)| current_denom == &denom)
        .ok_or(ContractError::AssetIsNotFound)?;

    if &user.rewards > pool_amount {
        Err(ContractError::NotEnoughLiquidity)?;
    }

    let msg = WasmMsg::Execute {
        contract_addr: treasury.to_string(),
        msg: to_json_binary(&loot_box_base::treasury::msg::ExecuteMsg::Send {
            amount: user.rewards,
            denom,
            recipient: sender_address.to_string(),
        })?,
        funds: vec![],
    };

    user.rewards = Uint128::zero();
    USERS.save(deps.storage, &sender_address, &user)?;

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("action", "try_claim"))
}

pub fn try_send(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
    recipient: String,
) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(deps.as_ref(), &sender_address, AuthType::Any)?;

    if amount.is_zero() {
        Err(ContractError::ZeroBoxAmount)?;
    }

    let recipient = deps.api.addr_validate(&recipient)?;

    USERS.update(deps.storage, &sender_address, |x| -> StdResult<UserInfo> {
        let mut user = x.unwrap_or_default();

        // check box amount
        if user.boxes.is_zero() {
            Err(ContractError::ZeroBoxAmount)?;
        }

        user.boxes -= amount;
        user.sent += amount;

        Ok(user)
    })?;

    USERS.update(deps.storage, &recipient, |x| -> StdResult<UserInfo> {
        let mut recipient = x.unwrap_or_default();
        recipient.boxes += amount;
        recipient.received += amount;

        Ok(recipient)
    })?;

    Ok(Response::new().add_attribute("action", "try_send"))
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

#[allow(clippy::too_many_arguments)]
pub fn try_update_config(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    admin: Option<String>,
    worker: Option<String>,
    box_price: Option<Uint128>,
    denom: Option<String>,
    distribution: Option<Vec<WeightInfo>>,
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

    if let Some(x) = box_price {
        check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;
        config.box_price = x;
        is_config_updated = true;
    }

    if let Some(x) = denom {
        check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;
        config.denom = x;
        is_config_updated = true;
    }

    if let Some(x) = distribution {
        check_authorization(deps.as_ref(), &sender_address, AuthType::AdminOrWorker)?;

        if x.iter().any(|y| y.weight > Decimal::one()) {
            Err(ContractError::WeightIsOutOfRange)?;
        }

        let weight_sum = x.iter().fold(Decimal::zero(), |acc, cur| acc + cur.weight);
        if weight_sum != Decimal::one() {
            Err(ContractError::WeightsAreUnbalanced)?;
        }

        config.distribution = x;
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

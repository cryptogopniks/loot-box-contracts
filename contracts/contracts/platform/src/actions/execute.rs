use cosmwasm_std::{
    coins, BankMsg, Decimal, DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};

use hashing_helper::base::calc_hash_bytes;
use loot_box_base::{
    converters::address_to_salt,
    error::ContractError,
    hash_generator::types::Hash,
    platform::{
        state::{
            BALANCE, BOX_STATS, CONFIG, NORMALIZED_DECIMAL, TRANSFER_ADMIN_STATE,
            TRANSFER_ADMIN_TIMEOUT, USERS,
        },
        types::{Balance, BoxStats, Config, TransferAdminState, UserInfo, WeightInfo},
    },
    utils::{check_funds, AuthType, FundsType},
};

use crate::helpers::{check_authorization, check_lockout, pick_rewards};

pub fn try_buy(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
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
        box_price, denom, ..
    } = CONFIG.load(deps.storage)?;

    // check fund denom
    if asset_info.try_get_native()? != denom {
        Err(ContractError::WrongAssetType)?;
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

    BALANCE.update(deps.storage, |mut x| -> StdResult<Balance> {
        x.pool += asset_amount;

        Ok(x)
    })?;

    Ok(Response::new()
        .add_attribute("action", "try_buy")
        .add_attribute("box_amount", box_amount.u128().to_string()))
}

pub fn try_open(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(deps.as_ref(), &sender_address, AuthType::Any)?;

    let mut response = Response::new().add_attribute("action", "try_open");
    let block_time = env.block.time.nanos();
    let normalized_decimal = NORMALIZED_DECIMAL.load(deps.storage)?;
    let Config {
        denom,
        distribution,
        ..
    } = CONFIG.load(deps.storage)?;
    let mut balance = BALANCE.load(deps.storage)?;
    let mut user = USERS
        .load(deps.storage, &sender_address)
        .unwrap_or_default();

    // check box amount
    if user.boxes.is_zero() {
        Err(ContractError::ZeroBoxAmount)?;
    }

    // get random rewards
    let password = &format!("{}{}", normalized_decimal.to_string(), block_time);
    let salt = &address_to_salt(&sender_address);
    let hash_bytes = calc_hash_bytes(password, salt)?;
    let random_weight = Hash::from(hash_bytes).to_norm_dec();
    let rewards = pick_rewards(&distribution, random_weight);

    NORMALIZED_DECIMAL.save(deps.storage, &random_weight)?;

    // send rewards if balance is enough else accumulate rewards
    if rewards <= balance.pool {
        balance.pool -= rewards;

        response = response.add_message(BankMsg::Send {
            to_address: sender_address.to_string(),
            amount: coins(rewards.u128(), denom),
        });
    } else {
        balance.rewards += rewards;
        user.rewards += rewards;
    }

    user.boxes -= Uint128::one();
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

    BALANCE.save(deps.storage, &balance)?;

    response = response.add_attribute("rewards", rewards.u128().to_string());

    Ok(response)
}

pub fn try_claim(deps: DepsMut, _env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;
    check_authorization(deps.as_ref(), &sender_address, AuthType::Any)?;

    let Config { denom, .. } = CONFIG.load(deps.storage)?;
    let mut balance = BALANCE.load(deps.storage)?;
    let mut user = USERS
        .load(deps.storage, &sender_address)
        .unwrap_or_default();

    // check rewards
    if user.rewards.is_zero() {
        Err(ContractError::ZeroRewardsAmount)?;
    }

    if user.rewards > balance.pool {
        Err(ContractError::NotEnoughLiquidity)?;
    }

    balance.pool -= user.rewards;
    balance.rewards -= user.rewards;
    user.rewards = Uint128::zero();

    USERS.save(deps.storage, &sender_address, &user)?;
    BALANCE.save(deps.storage, &balance)?;

    let msg = BankMsg::Send {
        to_address: sender_address.to_string(),
        amount: coins(user.rewards.u128(), denom),
    };

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

    let Config { denom, .. } = CONFIG.load(deps.storage)?;

    // check fund denom
    if asset_info.try_get_native()? != denom {
        Err(ContractError::WrongAssetType)?;
    }

    BALANCE.update(deps.storage, |mut x| -> StdResult<Balance> {
        x.pool += asset_amount;
        x.deposited += asset_amount;

        Ok(x)
    })?;

    Ok(Response::new().add_attribute("action", "try_deposit"))
}

pub fn try_withdraw(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response, ContractError> {
    check_lockout(deps.as_ref())?;
    let (sender_address, ..) = check_funds(deps.as_ref(), &info, FundsType::Empty)?;

    let Config { worker, denom, .. } = CONFIG.load(deps.storage)?;
    check_authorization(
        deps.as_ref(),
        &sender_address,
        AuthType::Specified {
            allowlist: vec![worker],
        },
    )?;

    BALANCE.update(deps.storage, |mut x| -> StdResult<Balance> {
        if amount > x.pool {
            Err(ContractError::NotEnoughLiquidity)?;
        }

        x.pool -= amount;

        if amount > x.deposited {
            x.deposited = Uint128::zero();
        } else {
            x.deposited -= amount;
        }

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

// TODO: check roles
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
        check_authorization(deps.as_ref(), &sender_address, AuthType::Admin)?;
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

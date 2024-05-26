use cosmwasm_std::{Addr, Deps, StdResult};

use loot_box_base::{
    error::ContractError,
    treasury::{
        state::{CONFIG, IS_LOCKED},
        types::{Config, NftInfo},
    },
    utils::{unwrap_field, AuthType},
};

pub fn check_lockout(deps: Deps) -> StdResult<()> {
    if IS_LOCKED.load(deps.storage)? {
        Err(ContractError::ContractIsLocked)?;
    }

    Ok(())
}

pub fn check_authorization(deps: Deps, sender: &Addr, auth_type: AuthType) -> StdResult<()> {
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

pub fn check_collections_holder<A: ToString>(
    deps: Deps,
    holder: &Addr,
    collections: &Vec<NftInfo<A>>,
) -> StdResult<()> {
    const MAX_LIMIT: u32 = 100;
    const ITER_LIMIT: u32 = 50;

    for NftInfo {
        collection,
        token_id,
        ..
    } in collections
    {
        let mut token_list: Vec<String> = vec![];
        let mut token_amount_sum: u32 = 0;
        let mut i: u32 = 0;
        let mut last_token: Option<String> = None;

        while (i == 0 || token_amount_sum == MAX_LIMIT) && i < ITER_LIMIT {
            i += 1;

            let query_tokens_msg = cw721::Cw721QueryMsg::Tokens {
                owner: holder.to_string(),
                start_after: last_token,
                limit: Some(MAX_LIMIT),
            };

            let cw721::TokensResponse { tokens } = deps
                .querier
                .query_wasm_smart(collection.to_string(), &query_tokens_msg)?;

            for token in tokens.clone() {
                token_list.push(token);
            }

            token_amount_sum = tokens.len() as u32;
            last_token = tokens.last().cloned();
        }

        let are_tokens_owned = token_list.contains(token_id);

        if !are_tokens_owned {
            Err(ContractError::NftIsNotFound)?;
        }
    }

    Ok(())
}

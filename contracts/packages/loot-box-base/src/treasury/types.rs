use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct Config {
    pub admin: Addr,          // contract maintainer
    pub worker: Option<Addr>, // app customer
    pub platform_code_id: Option<u64>,
    pub denom_list: Vec<String>, // updated by platform contracts
}

#[derive(Default)]
#[cw_serde]
pub struct Balance {
    pub pool: Vec<(Uint128, String)>,
    pub nft_pool: Vec<NftInfo<Addr>>,
    pub rewards: Vec<(Uint128, String)>,
    pub deposited: Vec<(Uint128, String)>,
}

#[cw_serde]
pub struct NftInfo<A: ToString> {
    pub collection: A,
    pub token_id: String,
    pub price_option: Vec<(Uint128, String)>,
}

#[cw_serde]
pub struct TransferAdminState {
    pub new_admin: Addr,
    pub deadline: u64,
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub worker: Option<Addr>,

    pub box_price: Uint128,
    pub denom: String,
    pub distribution: Vec<WeightInfo>,
}

#[cw_serde]
pub struct WeightInfo {
    pub box_rewards: Uint128,
    pub weight: Decimal,
}

#[derive(Default)]
#[cw_serde]
pub struct BoxStats {
    pub sold: Uint128,
    pub opened: Vec<OpeningInfo>,
}

#[cw_serde]
pub struct OpeningInfo {
    pub box_rewards: Uint128,
    pub opened: Uint128,
}

#[derive(Default)]
#[cw_serde]
pub struct Balance {
    pub pool: Uint128,
    pub nft_pool: Vec<NftInfo<Addr>>,
    pub rewards: Uint128,
    pub deposited: Uint128,
}

#[cw_serde]
pub struct NftInfo<A: ToString> {
    pub collection: A,
    pub token_id: String,
    pub price: Uint128,
}

#[derive(Default)]
#[cw_serde]
pub struct UserInfo {
    pub boxes: Uint128,
    pub rewards: Uint128,
    pub bought: Uint128,
    pub opened: Vec<OpeningInfo>,
    pub sent: Uint128,
    pub received: Uint128,
    pub opening_block: u64,
}

#[cw_serde]
pub struct TransferAdminState {
    pub new_admin: Addr,
    pub deadline: u64,
}

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};

// TODO: add box states: sold, open, open 50, open 250, etc.

#[derive(Default)]
#[cw_serde]
pub struct BoxList {
    pub update_date: u64,
    pub price_list: Vec<Uint128>, // TODO: add box id
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub worker: Option<Addr>,
    pub proxy: Option<Addr>,

    pub box_price: Uint128,
    pub price_and_weight_list: Vec<(Uint128, Decimal)>,
    pub box_list_length: u32,
}

#[cw_serde]
pub struct TransferAdminState {
    pub new_admin: Addr,
    pub deadline: u64,
}

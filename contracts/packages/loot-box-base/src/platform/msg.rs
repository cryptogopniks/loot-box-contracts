use cosmwasm_schema::{cw_serde, QueryResponses};

use cosmwasm_std::{Decimal, Uint128};

#[cw_serde]
pub struct MigrateMsg {
    pub version: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub worker: Option<String>,

    pub box_price: Option<Uint128>,
    pub price_and_weight_list: Option<Vec<(Uint128, Decimal)>>,
    pub box_list_length: Option<u32>,
}

#[cw_serde]
pub enum ExecuteMsg {
    // any
    AcceptAdminRole {},

    // admin
    UpdateConfig {
        admin: Option<String>,
        worker: Option<String>,

        box_price: Option<Uint128>,
        price_and_weight_list: Option<Vec<(Uint128, Decimal)>>,
        box_list_length: Option<u32>,
    },

    RequestBoxList {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::platform::types::Config)]
    QueryConfig {},

    #[returns(crate::platform::types::BoxList)]
    QueryBoxList {},
}

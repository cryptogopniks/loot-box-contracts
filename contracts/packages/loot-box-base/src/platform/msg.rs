use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::platform::types::RawPriceItem;

#[cw_serde]
pub struct InstantiateMsg {
    pub worker: Option<String>,
    pub scheduler: Option<String>,
}

#[cw_serde]
pub struct MigrateMsg {
    pub version: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    // any
    AcceptAdminRole {},

    // admin
    UpdateConfig {
        admin: Option<String>,
        worker: Option<String>,
        scheduler: Option<String>,
    },

    // scheduler
    UpdatePrices {
        data: Vec<RawPriceItem>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::platform::types::Config)]
    QueryConfig {},

    #[returns(Vec<crate::platform::types::PriceItem>)]
    QueryPrices {
        collection_addresses: Option<Vec<String>>,
        start_after: Option<String>,
        limit: Option<u32>,
    },

    #[returns(u64)]
    QueryBlockTime {},
}

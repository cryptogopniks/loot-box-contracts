use cosmwasm_schema::{cw_serde, QueryResponses};

use cosmwasm_std::{Addr, Uint128};

use crate::platform::types::{UserInfo, WeightInfo};

#[cw_serde]
pub struct MigrateMsg {
    pub version: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub worker: Option<String>,

    pub box_price: Option<Uint128>,
    pub denom: Option<String>,
    pub distribution: Option<Vec<WeightInfo>>,
}

#[cw_serde]
pub enum ExecuteMsg {
    // users
    Buy {},

    Open {},

    Claim {},

    Send {
        amount: Uint128,
        recipient: String,
    },

    // any
    AcceptAdminRole {},

    // admin
    Deposit {},

    Withdraw {},

    // DepositNftList {},
    // WithdrawNftList {},
    //
    UpdateConfig {
        admin: Option<String>,
        worker: Option<String>,

        box_price: Option<Uint128>,
        denom: Option<String>,
        distribution: Option<Vec<WeightInfo>>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::platform::types::Config)]
    QueryConfig {},

    #[returns(crate::platform::types::BoxStats)]
    QueryBoxStats {},

    #[returns(crate::platform::types::Balance)]
    QueryBalance {},

    #[returns(crate::platform::types::UserInfo)]
    QueryUser { address: String },

    #[returns(Vec<QueryUserListResponseItem>)]
    QueryUserList {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct QueryUserListResponseItem {
    pub address: Addr,
    pub info: UserInfo,
}

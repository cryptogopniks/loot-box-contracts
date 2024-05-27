use cosmwasm_schema::{cw_serde, QueryResponses};

use cosmwasm_std::Uint128;

use crate::{platform, treasury::types::NftInfo};

#[cw_serde]
pub struct MigrateMsg {
    pub version: String,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub worker: Option<String>,
    pub platform_code_id: Option<u64>,
}

#[cw_serde]
pub enum ExecuteMsg {
    // platform contracts
    IncreaseBalance {},

    Send {
        amount: Uint128,
        denom: String,
        recipient: String,
    },

    IncreaseRewards {
        amount: Uint128,
        denom: String,
    },

    SendNft {
        collection: String,
        token_id: String,
        recipient: String,
    },

    // new_admin
    AcceptAdminRole {},

    // admin
    CreatePlatform {
        inst_msg: platform::msg::InstantiateMsg,
    },

    AddPlatform {
        address: String,
    },

    RemovePlatform {
        address: String,
    },

    // admin, worker
    Deposit {},

    DepositNft {
        nft_info_list: Vec<NftInfo<String>>,
    },

    UpdateConfig {
        admin: Option<String>,
        worker: Option<String>,
        platform_code_id: Option<u64>,
    },

    Lock {},

    Unlock {},

    // worker
    Withdraw {
        amount: Uint128,
        denom: String,
    },

    WithdrawNft {
        nft_info_list: Vec<NftInfo<String>>,
    },

    UpdateNftPrice {
        nft_info_list: Vec<NftInfo<String>>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(crate::treasury::types::Config)]
    QueryConfig {},

    #[returns(crate::treasury::types::Balance)]
    QueryBalance {},

    #[returns(Vec<cosmwasm_std::Addr>)]
    QueryPlatformList {},

    #[returns(Vec<cosmwasm_std::Addr>)]
    QueryRemovedPlatformList {},
}

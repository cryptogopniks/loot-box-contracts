use cosmwasm_std::StdResult;
use cw_multi_test::{AppResponse, Executor};

use loot_box_base::{
    error::parse_err,
    platform::{
        msg::{ExecuteMsg, QueryMsg},
        types::Config,
    },
};

use crate::helpers::suite::{
    core::Project,
    types::{ProjectAccount, ProjectNft},
};

pub trait PlatformExtension {
    fn platform_try_accept_admin_role(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn platform_try_update_config(
        &mut self,
        sender: ProjectAccount,
        admin: &Option<ProjectAccount>,
        worker: &Option<ProjectAccount>,
        scheduler: &Option<ProjectAccount>,
    ) -> StdResult<AppResponse>;

    fn platform_try_update_prices(
        &mut self,
        sender: ProjectAccount,
        data: &[RawPriceItem],
    ) -> StdResult<AppResponse>;

    fn platform_query_config(&self) -> StdResult<Config>;

    fn platform_query_prices(
        &self,
        collection_addresses: &Option<Vec<ProjectNft>>,
        start_after: &Option<ProjectNft>,
        limit: &Option<u32>,
    ) -> StdResult<Vec<PriceItem>>;

    fn platform_query_block_time(&self) -> StdResult<u64>;
}

impl PlatformExtension for Project {
    #[track_caller]
    fn platform_try_accept_admin_role(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::AcceptAdminRole {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_update_config(
        &mut self,
        sender: ProjectAccount,
        admin: &Option<ProjectAccount>,
        worker: &Option<ProjectAccount>,
        scheduler: &Option<ProjectAccount>,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::UpdateConfig {
                    admin: admin.as_ref().map(|x| x.to_string()),
                    worker: worker.as_ref().map(|x| x.to_string()),
                    scheduler: scheduler.as_ref().map(|x| x.to_string()),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_update_prices(
        &mut self,
        sender: ProjectAccount,
        data: &[RawPriceItem],
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::UpdatePrices {
                    data: data.to_owned(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_query_config(&self) -> StdResult<Config> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_platform_address(), &QueryMsg::QueryConfig {})
    }

    #[track_caller]
    fn platform_query_prices(
        &self,
        collection_addresses: &Option<Vec<ProjectNft>>,
        start_after: &Option<ProjectNft>,
        limit: &Option<u32>,
    ) -> StdResult<Vec<PriceItem>> {
        let collection_addresses = collection_addresses
            .as_ref()
            .map(|x| x.iter().map(|y| y.to_string()).collect());

        self.app.wrap().query_wasm_smart(
            self.get_platform_address(),
            &QueryMsg::QueryPrices {
                collection_addresses,
                start_after: start_after.as_ref().map(|x| x.to_string()),
                limit: limit.to_owned(),
            },
        )
    }

    #[track_caller]
    fn platform_query_block_time(&self) -> StdResult<u64> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_platform_address(), &QueryMsg::QueryBlockTime {})
    }
}

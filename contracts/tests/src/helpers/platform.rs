use cosmwasm_std::{Addr, StdResult, Uint128};
use cw_multi_test::{AppResponse, Executor};

use loot_box_base::{
    error::parse_err,
    platform::{
        msg::{ExecuteMsg, QueryMsg, QueryUserListResponseItem},
        types::{BoxStats, Config, UserInfo, WeightInfo},
    },
};

use crate::helpers::suite::{
    core::{add_funds_to_exec_msg, Project},
    types::{ProjectAccount, ProjectCoin},
};

pub trait PlatformExtension {
    fn platform_try_buy(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
        amount: u128,
        asset: impl Into<ProjectCoin>,
    ) -> StdResult<AppResponse>;

    fn platform_try_open(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
    ) -> StdResult<AppResponse>;

    fn platform_try_claim(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
    ) -> StdResult<AppResponse>;

    fn platform_try_send(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
        amount: u128,
        recipient: ProjectAccount,
    ) -> StdResult<AppResponse>;

    fn platform_try_accept_admin_role(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
    ) -> StdResult<AppResponse>;

    #[allow(clippy::too_many_arguments)]
    fn platform_try_update_config(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
        admin: &Option<ProjectAccount>,
        worker: &Option<ProjectAccount>,
        box_price: &Option<u128>,
        denom: &Option<ProjectCoin>,
        distribution: &Option<Vec<WeightInfo>>,
    ) -> StdResult<AppResponse>;

    fn platform_try_lock(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
    ) -> StdResult<AppResponse>;

    fn platform_try_unlock(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
    ) -> StdResult<AppResponse>;

    fn platform_query_config(&self, platform_address: &Addr) -> StdResult<Config>;

    fn platform_query_box_stats(&self, platform_address: &Addr) -> StdResult<BoxStats>;

    fn platform_query_user(
        &self,
        platform_address: &Addr,
        address: ProjectAccount,
    ) -> StdResult<UserInfo>;

    fn platform_query_user_list(
        &self,
        platform_address: &Addr,
        start_after: &Option<ProjectAccount>,
        limit: &Option<u32>,
    ) -> StdResult<Vec<QueryUserListResponseItem>>;
}

impl PlatformExtension for Project {
    #[track_caller]
    fn platform_try_buy(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
        amount: u128,
        asset: impl Into<ProjectCoin>,
    ) -> StdResult<AppResponse> {
        let msg = &ExecuteMsg::Buy {};

        add_funds_to_exec_msg(self, sender, platform_address, msg, amount, asset.into())
    }

    #[track_caller]
    fn platform_try_open(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                platform_address.to_owned(),
                &ExecuteMsg::Open {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_claim(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                platform_address.to_owned(),
                &ExecuteMsg::Claim {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_send(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
        amount: u128,
        recipient: ProjectAccount,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                platform_address.to_owned(),
                &ExecuteMsg::Send {
                    amount: Uint128::new(amount),
                    recipient: recipient.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_accept_admin_role(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                platform_address.to_owned(),
                &ExecuteMsg::AcceptAdminRole {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_update_config(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
        admin: &Option<ProjectAccount>,
        worker: &Option<ProjectAccount>,
        box_price: &Option<u128>,
        denom: &Option<ProjectCoin>,
        distribution: &Option<Vec<WeightInfo>>,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                platform_address.to_owned(),
                &ExecuteMsg::UpdateConfig {
                    admin: admin.as_ref().map(|x| x.to_string()),
                    worker: worker.as_ref().map(|x| x.to_string()),
                    box_price: box_price.as_ref().map(|x| Uint128::new(x.to_owned())),
                    denom: denom.as_ref().map(|x| x.to_string()),
                    distribution: distribution.to_owned(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_lock(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                platform_address.to_owned(),
                &ExecuteMsg::Lock {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_unlock(
        &mut self,
        platform_address: &Addr,
        sender: ProjectAccount,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                platform_address.to_owned(),
                &ExecuteMsg::Unlock {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_query_config(&self, platform_address: &Addr) -> StdResult<Config> {
        self.app
            .wrap()
            .query_wasm_smart(platform_address.to_owned(), &QueryMsg::QueryConfig {})
    }

    #[track_caller]
    fn platform_query_box_stats(&self, platform_address: &Addr) -> StdResult<BoxStats> {
        self.app
            .wrap()
            .query_wasm_smart(platform_address.to_owned(), &QueryMsg::QueryBoxStats {})
    }

    #[track_caller]
    fn platform_query_user(
        &self,
        platform_address: &Addr,
        address: ProjectAccount,
    ) -> StdResult<UserInfo> {
        self.app.wrap().query_wasm_smart(
            platform_address.to_owned(),
            &QueryMsg::QueryUser {
                address: address.to_string(),
            },
        )
    }

    #[track_caller]
    fn platform_query_user_list(
        &self,
        platform_address: &Addr,
        start_after: &Option<ProjectAccount>,
        limit: &Option<u32>,
    ) -> StdResult<Vec<QueryUserListResponseItem>> {
        self.app.wrap().query_wasm_smart(
            platform_address.to_owned(),
            &QueryMsg::QueryUserList {
                start_after: start_after.as_ref().map(|x| x.to_string()),
                limit: limit.to_owned(),
            },
        )
    }
}

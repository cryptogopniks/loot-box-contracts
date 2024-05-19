use cosmwasm_std::{StdResult, Uint128};
use cw_multi_test::{AppResponse, Executor};

use loot_box_base::{
    error::parse_err,
    platform::{
        msg::{ExecuteMsg, QueryMsg},
        types::{Config, WeightInfo},
    },
};

use crate::helpers::suite::{
    core::Project,
    types::{ProjectAccount, ProjectCoin},
};

pub trait PlatformExtension {
    fn platform_try_accept_admin_role(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn platform_try_update_config(
        &mut self,
        sender: ProjectAccount,
        admin: &Option<ProjectAccount>,
        worker: &Option<ProjectAccount>,
        box_price: &Option<u128>,
        denom: &Option<ProjectCoin>,
        distribution: &Option<Vec<WeightInfo>>,
    ) -> StdResult<AppResponse>;

    fn platform_query_config(&self) -> StdResult<Config>;
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
        box_price: &Option<u128>,
        denom: &Option<ProjectCoin>,
        distribution: &Option<Vec<WeightInfo>>,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
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
    fn platform_query_config(&self) -> StdResult<Config> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_platform_address(), &QueryMsg::QueryConfig {})
    }
}

use cosmwasm_std::{StdResult, Uint128};
use cw_multi_test::{AppResponse, Executor};

use loot_box_base::{
    converters::str_to_dec,
    error::parse_err,
    platform::{
        msg::{ExecuteMsg, QueryMsg},
        types::{BoxList, Config},
    },
};

use crate::helpers::suite::{core::Project, types::ProjectAccount};

pub trait PlatformExtension {
    fn platform_try_request_box_list(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn platform_try_pick_number(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn platform_try_accept_admin_role(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn platform_try_update_config(
        &mut self,
        sender: ProjectAccount,
        admin: &Option<ProjectAccount>,
        worker: &Option<ProjectAccount>,
        box_price: &Option<u128>,
        price_and_weight_list: &Option<Vec<(u128, &str)>>,
        box_list_length: &Option<u32>,
    ) -> StdResult<AppResponse>;

    fn platform_query_config(&self) -> StdResult<Config>;

    fn platform_query_box_list(&self) -> StdResult<BoxList>;
}

impl PlatformExtension for Project {
    #[track_caller]
    fn platform_try_request_box_list(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::RequestBoxList {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_pick_number(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::PickNumber {},
                &[],
            )
            .map_err(parse_err)
    }

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
        price_and_weight_list: &Option<Vec<(u128, &str)>>,
        box_list_length: &Option<u32>,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::UpdateConfig {
                    admin: admin.as_ref().map(|x| x.to_string()),
                    worker: worker.as_ref().map(|x| x.to_string()),
                    box_price: box_price.as_ref().map(|x| Uint128::new(x.to_owned())),
                    price_and_weight_list: price_and_weight_list.as_ref().map(|x| {
                        x.to_owned()
                            .into_iter()
                            .map(|(price, weight)| (Uint128::new(price), str_to_dec(weight)))
                            .collect()
                    }),
                    box_list_length: box_list_length.to_owned(),
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
    fn platform_query_box_list(&self) -> StdResult<BoxList> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_platform_address(), &QueryMsg::QueryBoxList {})
    }
}

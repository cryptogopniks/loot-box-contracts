use cosmwasm_std::{StdResult, Uint128};
use cw_multi_test::{AppResponse, Executor};

use loot_box_base::{
    error::parse_err,
    platform::{
        msg::{ExecuteMsg, QueryMsg, QueryUserListResponseItem},
        types::{Balance, BoxStats, Config, NftInfo, UserInfo, WeightInfo},
    },
};

use crate::helpers::suite::{
    core::{add_funds_to_exec_msg, Project},
    types::{ProjectAccount, ProjectCoin},
};

pub trait PlatformExtension {
    fn platform_try_buy(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
        asset: impl Into<ProjectCoin>,
    ) -> StdResult<AppResponse>;

    fn platform_try_open(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn platform_try_claim(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn platform_try_send(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
        recipient: ProjectAccount,
    ) -> StdResult<AppResponse>;

    fn platform_try_accept_admin_role(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn platform_try_deposit(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
        asset: impl Into<ProjectCoin>,
    ) -> StdResult<AppResponse>;

    fn platform_try_deposit_nft(
        &mut self,
        sender: ProjectAccount,
        nft_info_list: &[NftInfo<String>],
    ) -> StdResult<AppResponse>;

    fn platform_try_update_config(
        &mut self,
        sender: ProjectAccount,
        admin: &Option<ProjectAccount>,
        worker: &Option<ProjectAccount>,
        box_price: &Option<u128>,
        denom: &Option<ProjectCoin>,
        distribution: &Option<Vec<WeightInfo>>,
    ) -> StdResult<AppResponse>;

    fn platform_try_lock(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn platform_try_unlock(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn platform_try_withdraw(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
    ) -> StdResult<AppResponse>;

    fn platform_try_withdraw_nft(
        &mut self,
        sender: ProjectAccount,
        nft_info_list: &[NftInfo<String>],
    ) -> StdResult<AppResponse>;

    fn platform_try_update_nft_price(
        &mut self,
        sender: ProjectAccount,
        nft_info_list: &[NftInfo<String>],
    ) -> StdResult<AppResponse>;

    fn platform_query_config(&self) -> StdResult<Config>;

    fn platform_query_box_stats(&self) -> StdResult<BoxStats>;

    fn platform_query_balance(&self) -> StdResult<Balance>;

    fn platform_query_user(&self, address: ProjectAccount) -> StdResult<UserInfo>;

    fn platform_query_user_list(
        &self,
        start_after: &Option<ProjectAccount>,
        limit: &Option<u32>,
    ) -> StdResult<Vec<QueryUserListResponseItem>>;
}

impl PlatformExtension for Project {
    #[track_caller]
    fn platform_try_buy(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
        asset: impl Into<ProjectCoin>,
    ) -> StdResult<AppResponse> {
        let contract_address = &self.get_platform_address();
        let msg = &ExecuteMsg::Buy {};

        add_funds_to_exec_msg(self, sender, contract_address, msg, amount, asset.into())
    }

    #[track_caller]
    fn platform_try_open(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::Open {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_claim(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::Claim {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_send(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
        recipient: ProjectAccount,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::Send {
                    amount: Uint128::new(amount),
                    recipient: recipient.to_string(),
                },
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
    fn platform_try_deposit(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
        asset: impl Into<ProjectCoin>,
    ) -> StdResult<AppResponse> {
        let contract_address = &self.get_platform_address();
        let msg = &ExecuteMsg::Deposit {};

        add_funds_to_exec_msg(self, sender, contract_address, msg, amount, asset.into())
    }

    #[track_caller]
    fn platform_try_deposit_nft(
        &mut self,
        sender: ProjectAccount,
        nft_info_list: &[NftInfo<String>],
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::DepositNft {
                    nft_info_list: nft_info_list.to_owned(),
                },
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
    fn platform_try_lock(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::Lock {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_unlock(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::Unlock {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_withdraw(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::Withdraw {
                    amount: Uint128::new(amount),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_withdraw_nft(
        &mut self,
        sender: ProjectAccount,
        nft_info_list: &[NftInfo<String>],
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::WithdrawNft {
                    nft_info_list: nft_info_list.to_owned(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn platform_try_update_nft_price(
        &mut self,
        sender: ProjectAccount,
        nft_info_list: &[NftInfo<String>],
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_platform_address(),
                &ExecuteMsg::UpdateNftPrice {
                    nft_info_list: nft_info_list.to_owned(),
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
    fn platform_query_box_stats(&self) -> StdResult<BoxStats> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_platform_address(), &QueryMsg::QueryBoxStats {})
    }

    #[track_caller]
    fn platform_query_balance(&self) -> StdResult<Balance> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_platform_address(), &QueryMsg::QueryBalance {})
    }

    #[track_caller]
    fn platform_query_user(&self, address: ProjectAccount) -> StdResult<UserInfo> {
        self.app.wrap().query_wasm_smart(
            self.get_platform_address(),
            &QueryMsg::QueryUser {
                address: address.to_string(),
            },
        )
    }

    #[track_caller]
    fn platform_query_user_list(
        &self,
        start_after: &Option<ProjectAccount>,
        limit: &Option<u32>,
    ) -> StdResult<Vec<QueryUserListResponseItem>> {
        self.app.wrap().query_wasm_smart(
            self.get_platform_address(),
            &QueryMsg::QueryUserList {
                start_after: start_after.as_ref().map(|x| x.to_string()),
                limit: limit.to_owned(),
            },
        )
    }
}

use cosmwasm_std::{Addr, StdResult, Uint128};
use cw_multi_test::{AppResponse, Executor};

use loot_box_base::{
    error::parse_err,
    platform,
    treasury::{
        msg::{ExecuteMsg, QueryMsg},
        types::{Balance, Config, NftInfo},
    },
};

use crate::helpers::suite::{
    core::{add_funds_to_exec_msg, Project},
    types::{ProjectAccount, ProjectCoin, ProjectNft},
};

pub trait TreasuryExtension {
    fn treasury_try_increase_balance(
        &mut self,
        sender: &Addr,
        amount: u128,
        asset: impl Into<ProjectCoin>,
    ) -> StdResult<AppResponse>;

    fn treasury_try_send(
        &mut self,
        sender: &Addr,
        amount: u128,
        denom: ProjectCoin,
        recipient: ProjectAccount,
    ) -> StdResult<AppResponse>;

    fn treasury_try_increase_rewards(
        &mut self,
        sender: &Addr,
        amount: u128,
        denom: ProjectCoin,
    ) -> StdResult<AppResponse>;

    fn treasury_try_send_nft(
        &mut self,
        sender: &Addr,
        collection: ProjectNft,
        token_id: &str,
        recipient: ProjectAccount,
    ) -> StdResult<AppResponse>;

    fn treasury_try_accept_admin_role(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn treasury_try_create_platform(
        &mut self,
        sender: ProjectAccount,
        inst_msg: &platform::msg::InstantiateMsg,
    ) -> StdResult<AppResponse>;

    fn treasury_try_add_platform(
        &mut self,
        sender: ProjectAccount,
        address: &Addr,
    ) -> StdResult<AppResponse>;

    fn treasury_try_remove_platform(
        &mut self,
        sender: ProjectAccount,
        address: &Addr,
    ) -> StdResult<AppResponse>;

    fn treasury_try_deposit(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
        asset: impl Into<ProjectCoin>,
    ) -> StdResult<AppResponse>;

    fn treasury_try_deposit_nft(
        &mut self,
        sender: ProjectAccount,
        nft_info_list: &[NftInfo<String>],
    ) -> StdResult<AppResponse>;

    fn treasury_try_update_config(
        &mut self,
        sender: ProjectAccount,
        admin: &Option<ProjectAccount>,
        worker: &Option<ProjectAccount>,
        platform_code_id: &Option<u64>,
    ) -> StdResult<AppResponse>;

    fn treasury_try_lock(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn treasury_try_unlock(&mut self, sender: ProjectAccount) -> StdResult<AppResponse>;

    fn treasury_try_withdraw(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
        denom: ProjectCoin,
    ) -> StdResult<AppResponse>;

    fn treasury_try_withdraw_nft(
        &mut self,
        sender: ProjectAccount,
        nft_info_list: &[NftInfo<impl ToString>],
    ) -> StdResult<AppResponse>;

    fn treasury_try_update_nft_price(
        &mut self,
        sender: ProjectAccount,
        nft_info_list: &[NftInfo<impl ToString>],
    ) -> StdResult<AppResponse>;

    fn treasury_query_config(&self) -> StdResult<Config>;

    fn treasury_query_balance(&self) -> StdResult<Balance>;

    fn treasury_query_platform_list(&self) -> StdResult<Vec<Addr>>;

    fn treasury_query_removed_platform_list(&self) -> StdResult<Vec<Addr>>;
}

impl TreasuryExtension for Project {
    #[track_caller]
    fn treasury_try_increase_balance(
        &mut self,
        sender: &Addr,
        amount: u128,
        asset: impl Into<ProjectCoin>,
    ) -> StdResult<AppResponse> {
        let contract_address = &self.get_treasury_address();
        let msg = &ExecuteMsg::IncreaseBalance {};

        add_funds_to_exec_msg(
            self,
            sender.to_owned(),
            contract_address,
            msg,
            amount,
            asset.into(),
        )
    }

    #[track_caller]
    fn treasury_try_send(
        &mut self,
        sender: &Addr,
        amount: u128,
        denom: ProjectCoin,
        recipient: ProjectAccount,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.to_owned(),
                self.get_treasury_address(),
                &ExecuteMsg::Send {
                    amount: Uint128::new(amount),
                    denom: denom.to_string(),
                    recipient: recipient.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_try_increase_rewards(
        &mut self,
        sender: &Addr,
        amount: u128,
        denom: ProjectCoin,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.to_owned(),
                self.get_treasury_address(),
                &ExecuteMsg::IncreaseRewards {
                    amount: Uint128::new(amount),
                    denom: denom.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_try_send_nft(
        &mut self,
        sender: &Addr,
        collection: ProjectNft,
        token_id: &str,
        recipient: ProjectAccount,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.to_owned(),
                self.get_treasury_address(),
                &ExecuteMsg::SendNft {
                    collection: collection.to_string(),
                    token_id: token_id.to_string(),
                    recipient: recipient.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_try_accept_admin_role(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_treasury_address(),
                &ExecuteMsg::AcceptAdminRole {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_try_create_platform(
        &mut self,
        sender: ProjectAccount,
        inst_msg: &platform::msg::InstantiateMsg,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_treasury_address(),
                &ExecuteMsg::CreatePlatform {
                    inst_msg: inst_msg.to_owned(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_try_add_platform(
        &mut self,
        sender: ProjectAccount,
        address: &Addr,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_treasury_address(),
                &ExecuteMsg::AddPlatform {
                    address: address.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_try_remove_platform(
        &mut self,
        sender: ProjectAccount,
        address: &Addr,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_treasury_address(),
                &ExecuteMsg::RemovePlatform {
                    address: address.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_try_deposit(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
        asset: impl Into<ProjectCoin>,
    ) -> StdResult<AppResponse> {
        let contract_address = &self.get_treasury_address();
        let msg = &ExecuteMsg::Deposit {};

        add_funds_to_exec_msg(self, sender, contract_address, msg, amount, asset.into())
    }

    #[track_caller]
    fn treasury_try_deposit_nft(
        &mut self,
        sender: ProjectAccount,
        nft_info_list: &[NftInfo<String>],
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_treasury_address(),
                &ExecuteMsg::DepositNft {
                    nft_info_list: nft_info_list.to_owned(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_try_update_config(
        &mut self,
        sender: ProjectAccount,
        admin: &Option<ProjectAccount>,
        worker: &Option<ProjectAccount>,
        platform_code_id: &Option<u64>,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_treasury_address(),
                &ExecuteMsg::UpdateConfig {
                    admin: admin.as_ref().map(|x| x.to_string()),
                    worker: worker.as_ref().map(|x| x.to_string()),
                    platform_code_id: platform_code_id.to_owned(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_try_lock(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_treasury_address(),
                &ExecuteMsg::Lock {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_try_unlock(&mut self, sender: ProjectAccount) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_treasury_address(),
                &ExecuteMsg::Unlock {},
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_try_withdraw(
        &mut self,
        sender: ProjectAccount,
        amount: u128,
        denom: ProjectCoin,
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_treasury_address(),
                &ExecuteMsg::Withdraw {
                    amount: Uint128::new(amount),
                    denom: denom.to_string(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_try_withdraw_nft(
        &mut self,
        sender: ProjectAccount,
        nft_info_list: &[NftInfo<impl ToString>],
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_treasury_address(),
                &ExecuteMsg::WithdrawNft {
                    nft_info_list: nft_info_list
                        .iter()
                        .map(|x| NftInfo {
                            collection: x.collection.to_string(),
                            token_id: x.token_id.clone(),
                            price_option: x.price_option.clone(),
                        })
                        .collect(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_try_update_nft_price(
        &mut self,
        sender: ProjectAccount,
        nft_info_list: &[NftInfo<impl ToString>],
    ) -> StdResult<AppResponse> {
        self.app
            .execute_contract(
                sender.into(),
                self.get_treasury_address(),
                &ExecuteMsg::UpdateNftPrice {
                    nft_info_list: nft_info_list
                        .iter()
                        .map(|x| NftInfo {
                            collection: x.collection.to_string(),
                            token_id: x.token_id.clone(),
                            price_option: x.price_option.clone(),
                        })
                        .collect(),
                },
                &[],
            )
            .map_err(parse_err)
    }

    #[track_caller]
    fn treasury_query_config(&self) -> StdResult<Config> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_treasury_address(), &QueryMsg::QueryConfig {})
    }

    #[track_caller]
    fn treasury_query_balance(&self) -> StdResult<Balance> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_treasury_address(), &QueryMsg::QueryBalance {})
    }

    #[track_caller]
    fn treasury_query_platform_list(&self) -> StdResult<Vec<Addr>> {
        self.app
            .wrap()
            .query_wasm_smart(self.get_treasury_address(), &QueryMsg::QueryPlatformList {})
    }

    #[track_caller]
    fn treasury_query_removed_platform_list(&self) -> StdResult<Vec<Addr>> {
        self.app.wrap().query_wasm_smart(
            self.get_treasury_address(),
            &QueryMsg::QueryRemovedPlatformList {},
        )
    }
}

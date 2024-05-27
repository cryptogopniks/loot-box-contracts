use cosmwasm_std::{Addr, StdResult, Uint128};
use cw_multi_test::{AppResponse, ContractWrapper, Executor};

use serde::Serialize;
use strum::IntoEnumIterator;

use loot_box_base::{error::parse_err, platform::types::WeightInfo};

use crate::helpers::suite::{
    core::Project,
    types::{GetDecimals, ProjectAccount, ProjectNft, ProjectToken},
};

use super::types::ProjectCoin;

pub trait WithCodes {
    // store packages
    fn store_cw20_base_code(&mut self) -> u64;
    fn store_cw721_base_code(&mut self) -> u64;

    // store contracts
    fn store_platform_code(&mut self) -> u64;
    fn store_treasury_code(&mut self) -> u64;

    // instantiate packages
    fn instantiate_cw20_base_token(&mut self, code_id: u64, project_token: ProjectToken) -> Addr;
    fn instantiate_cw721_base_token(&mut self, code_id: u64, project_nft: ProjectNft) -> Addr;

    // instantiate contracts
    fn instantiate_platform(
        &mut self,
        platform_code_id: u64,
        worker: &Option<ProjectAccount>,
        treasury: &Addr,
        box_price: &Option<u128>,
        denom: &Option<ProjectCoin>,
        distribution: &Option<Vec<WeightInfo>>,
    ) -> Addr;

    fn instantiate_treasury(
        &mut self,
        treasury_code_id: u64,
        platform_code_id: u64,
        worker: &Option<ProjectAccount>,
    ) -> Addr;

    fn migrate_contract(
        &mut self,
        sender: ProjectAccount,
        contract_address: Addr,
        contract_new_code_id: u64,
        migrate_msg: impl Serialize,
    ) -> StdResult<AppResponse>;
}

impl WithCodes for Project {
    // store packages
    fn store_cw20_base_code(&mut self) -> u64 {
        self.app.store_code(Box::new(ContractWrapper::new(
            cw20_base::contract::execute,
            cw20_base::contract::instantiate,
            cw20_base::contract::query,
        )))
    }

    fn store_cw721_base_code(&mut self) -> u64 {
        self.app.store_code(Box::new(ContractWrapper::new(
            cw721_base::entry::execute,
            cw721_base::entry::instantiate,
            cw721_base::entry::query,
        )))
    }

    // store contracts
    fn store_platform_code(&mut self) -> u64 {
        self.app.store_code(Box::new(
            ContractWrapper::new(
                platform::contract::execute,
                platform::contract::instantiate,
                platform::contract::query,
            )
            .with_reply(platform::contract::reply)
            .with_migrate(platform::contract::migrate),
        ))
    }

    fn store_treasury_code(&mut self) -> u64 {
        self.app.store_code(Box::new(
            ContractWrapper::new(
                treasury::contract::execute,
                treasury::contract::instantiate,
                treasury::contract::query,
            )
            .with_reply(treasury::contract::reply)
            .with_migrate(treasury::contract::migrate),
        ))
    }

    // instantiate packages
    fn instantiate_cw20_base_token(&mut self, code_id: u64, project_token: ProjectToken) -> Addr {
        let token_postfix: u8 = project_token
            .to_string()
            .strip_prefix("contract")
            .unwrap()
            .parse()
            .unwrap();

        let symbol = format!("TKN{}", "N".repeat(token_postfix as usize)); // max 10 tokens

        let initial_balances: Vec<cw20::Cw20Coin> = ProjectAccount::iter()
            .map(|project_account| {
                let amount = project_account.get_initial_funds_amount()
                    * 10u128.pow(project_token.get_decimals() as u32);

                cw20::Cw20Coin {
                    address: project_account.to_string(),
                    amount: Uint128::from(amount),
                }
            })
            .collect();

        self.instantiate_contract(
            code_id,
            &format!("token{}", "n".repeat(token_postfix as usize)),
            &cw20_base::msg::InstantiateMsg {
                name: format!("cw20-base token {}", symbol),
                symbol,
                decimals: project_token.get_decimals(),
                initial_balances,
                mint: None,
                marketing: None,
            },
        )
    }

    fn instantiate_cw721_base_token(&mut self, code_id: u64, project_nft: ProjectNft) -> Addr {
        let token_postfix: u8 = project_nft
            .to_string()
            .strip_prefix("contract")
            .unwrap()
            .parse()
            .unwrap();

        let symbol = format!("NFT{}", "T".repeat(token_postfix as usize)); // max 10 tokens

        self.instantiate_contract(
            code_id,
            &format!("nft{}", "t".repeat(token_postfix as usize)),
            &cw721_base::msg::InstantiateMsg {
                name: format!("cw721-base token {}", symbol),
                symbol,
                minter: ProjectAccount::Owner.to_string(),
            },
        )
    }

    fn instantiate_platform(
        &mut self,
        platform_code_id: u64,
        worker: &Option<ProjectAccount>,
        treasury: &Addr,
        box_price: &Option<u128>,
        denom: &Option<ProjectCoin>,
        distribution: &Option<Vec<WeightInfo>>,
    ) -> Addr {
        self.instantiate_contract(
            platform_code_id,
            "platform",
            &loot_box_base::platform::msg::InstantiateMsg {
                worker: worker.as_ref().map(|x| x.to_string()),
                treasury: treasury.to_string(),
                box_price: box_price.as_ref().map(|x| Uint128::new(x.to_owned())),
                denom: denom.as_ref().map(|x| x.to_string()),
                distribution: distribution.to_owned(),
            },
        )
    }

    fn instantiate_treasury(
        &mut self,
        treasury_code_id: u64,
        platform_code_id: u64,
        worker: &Option<ProjectAccount>,
    ) -> Addr {
        self.instantiate_contract(
            treasury_code_id,
            "treasury",
            &loot_box_base::treasury::msg::InstantiateMsg {
                worker: worker.as_ref().map(|x| x.to_string()),
                platform_code_id: Some(platform_code_id),
            },
        )
    }

    fn migrate_contract(
        &mut self,
        sender: ProjectAccount,
        contract_address: Addr,
        contract_new_code_id: u64,
        migrate_msg: impl Serialize,
    ) -> StdResult<AppResponse> {
        self.app
            .migrate_contract(
                sender.into(),
                contract_address,
                &migrate_msg,
                contract_new_code_id,
            )
            .map_err(parse_err)
    }
}

use cw_multi_test::Executor;
use speculoos::assert_that;

use cosmwasm_std::{StdResult, Uint128};

use loot_box_base::{
    assets::{Currency, Funds, Token, TokenUnverified},
    error::ContractError,
    platform::msg::MigrateMsg,
};

use crate::helpers::{
    platform::PlatformExtension,
    suite::{
        core::{assert_error, Project},
        types::{GetDecimals, ProjectAccount, ProjectCoin, ProjectNft, ProjectToken},
    },
};

#[test]
fn default() -> StdResult<()> {
    let mut project = Project::new();

    let block_time = project.get_block_time();

    Ok(())
}

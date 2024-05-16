use cw_multi_test::Executor;
use speculoos::assert_that;

use cosmwasm_std::{StdResult, Uint128};

use loot_box_base::{
    assets::{Currency, Funds, Token, TokenUnverified},
    error::ContractError,
    platform::{
        msg::MigrateMsg,
        types::{PriceItem, RawPriceItem},
    },
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

    project.platform_try_update_prices(
        ProjectAccount::Scheduler,
        &[
            RawPriceItem {
                collection_address: ProjectNft::Pinjeons.to_string(),
                price: Funds::new(
                    Uint128::new(300),
                    &Currency::new(
                        &TokenUnverified::new_native(&ProjectCoin::Kuji.to_string()),
                        ProjectCoin::Kuji.get_decimals(),
                    ),
                ),
            },
            RawPriceItem {
                collection_address: ProjectNft::Gopniks.to_string(),
                price: Funds::new(
                    Uint128::new(800),
                    &Currency::new(
                        &TokenUnverified::new_cw20(&ProjectToken::Atom.to_string()),
                        ProjectToken::Atom.get_decimals(),
                    ),
                ),
            },
        ],
    )?;

    let res = project.platform_query_prices(&None, &None, &None)?;

    project.wait(3);

    assert_that(&res).is_equal_to(vec![
        PriceItem {
            address: ProjectNft::Gopniks.into(),
            price: Funds::new(
                Uint128::new(800),
                &Currency::new(
                    &Token::new_cw20(&ProjectToken::Atom.into()),
                    ProjectToken::Atom.get_decimals(),
                ),
            ),
            price_update_date: block_time,
        },
        PriceItem {
            address: ProjectNft::Pinjeons.into(),
            price: Funds::new(
                Uint128::new(300),
                &Currency::new(
                    &Token::new_native(&ProjectCoin::Kuji.to_string()),
                    ProjectCoin::Kuji.get_decimals(),
                ),
            ),
            price_update_date: block_time,
        },
    ]);

    Ok(())
}

#[test]
fn unauthorized() -> StdResult<()> {
    let mut project = Project::new();

    let res = project
        .platform_try_update_prices(
            ProjectAccount::Admin,
            &[
                RawPriceItem {
                    collection_address: ProjectNft::Pinjeons.to_string(),
                    price: Funds::new(
                        Uint128::new(300),
                        &Currency::new(
                            &TokenUnverified::new_native(&ProjectCoin::Kuji.to_string()),
                            ProjectCoin::Kuji.get_decimals(),
                        ),
                    ),
                },
                RawPriceItem {
                    collection_address: ProjectNft::Gopniks.to_string(),
                    price: Funds::new(
                        Uint128::new(800),
                        &Currency::new(
                            &TokenUnverified::new_cw20(&ProjectToken::Atom.to_string()),
                            ProjectToken::Atom.get_decimals(),
                        ),
                    ),
                },
            ],
        )
        .unwrap_err();

    assert_error(&res, ContractError::Unauthorized);

    Ok(())
}

#[test]
fn duplicated_collections() -> StdResult<()> {
    let mut project = Project::new();

    let res = project
        .platform_try_update_prices(
            ProjectAccount::Scheduler,
            &[
                RawPriceItem {
                    collection_address: ProjectNft::Pinjeons.to_string(),
                    price: Funds::new(
                        Uint128::new(300),
                        &Currency::new(
                            &TokenUnverified::new_native(&ProjectCoin::Kuji.to_string()),
                            ProjectCoin::Kuji.get_decimals(),
                        ),
                    ),
                },
                RawPriceItem {
                    collection_address: ProjectNft::Pinjeons.to_string(),
                    price: Funds::new(
                        Uint128::new(300),
                        &Currency::new(
                            &TokenUnverified::new_native(&ProjectCoin::Kuji.to_string()),
                            ProjectCoin::Kuji.get_decimals(),
                        ),
                    ),
                },
            ],
        )
        .unwrap_err();

    assert_error(&res, ContractError::CollectionDuplication);

    Ok(())
}

#[test]
fn empty_raw_price_item_vec() -> StdResult<()> {
    let mut project = Project::new();

    let res = project
        .platform_try_update_prices(ProjectAccount::Scheduler, &[])
        .unwrap_err();

    assert_error(&res, ContractError::EmptyRawPriceItemVector);

    Ok(())
}

#[test]
fn zeros_in_prices() -> StdResult<()> {
    let mut project = Project::new();

    let res = project
        .platform_try_update_prices(
            ProjectAccount::Scheduler,
            &[RawPriceItem {
                collection_address: ProjectNft::Pinjeons.to_string(),
                price: Funds::new(
                    Uint128::new(0),
                    &Currency::new(
                        &TokenUnverified::new_native(&ProjectCoin::Kuji.to_string()),
                        ProjectCoin::Kuji.get_decimals(),
                    ),
                ),
            }],
        )
        .unwrap_err();

    assert_error(&res, ContractError::ZerosInPrices);

    Ok(())
}

#[test]
fn migrate_default() {
    let mut project = Project::new();

    project
        .app
        .migrate_contract(
            ProjectAccount::Admin.into(),
            project.get_platform_address(),
            &MigrateMsg {
                version: "1.0.0".to_string(),
            },
            project.get_platform_code_id(),
        )
        .unwrap();
}

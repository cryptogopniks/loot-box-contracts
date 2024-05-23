use cw_multi_test::AppResponse;
use speculoos::assert_that;

use cosmwasm_std::{Decimal, StdResult, Uint128};

use loot_box_base::{
    converters::{str_to_dec, u128_to_dec},
    error::ContractError,
    platform::types::{NftInfo, OpeningInfo, WeightInfo},
};

use crate::helpers::{
    platform::PlatformExtension,
    suite::{
        core::{assert_error, Project},
        types::{ProjectAccount, ProjectCoin, ProjectNft},
    },
};

// 1) aggressive distribution
// x0 + x1 + x2 + x3 + x4 = 1
// 0*x0 + 50*x1 + 250*x2 + 500*x3 + 1000*x4 = 85

// x0 + x1 + 0.2*x1 + 0.1*x1 + 0.05*x1 = 1
// 0*x0 + 50*x1 + 50*x1 + 50*x1 + 50*x1 = 85

// x0 + 1.35*x1 = 1
// 200*x1 = 85

// x0 = 0.42625
// x1 = 0.42500
// x2 = 0.08500
// x3 = 0.04250
// x4 = 0.02125

// 0.14875 winning chance

// 2) conservative distribution
// 0*x0 + 50*x1 + 150*x2 + 200*x3 + 250*x4 = 85

// x0 = 0.24209
// x1 = 0.42500
// x2 = 0.14166
// x3 = 0.10625
// x4 = 0.08500

// 0.33291 winning chance

// 3) mixed distribution
// 0*x0 + 50*x1 + 150*x2 + 250*x3 + 1000*x4 = 85

// x0 = 0.32709
// x1 = 0.42500
// x2 = 0.14166
// x3 = 0.08500
// x4 = 0.02125

// 0.24791 winning chance

fn parse_attr(res: &AppResponse, key: &str) -> Option<String> {
    res.events
        .iter()
        .find(|x| x.attributes.iter().any(|y| y.key == key))
        .and_then(|x| {
            x.attributes
                .iter()
                .find(|y| y.key == key)
                .map(|z| z.value.to_owned())
        })
}

#[test]
fn opening_probability() -> StdResult<()> {
    const BOX_PRICE: u128 = 100;
    const ROUNDS: u128 = 1000;

    let mut project = Project::new();
    project.reset_time();

    let price_and_weight_list = vec![
        (0, "0.46"),
        (50, "0.40"),
        (250, "0.08"),
        (500, "0.04"),
        (1000, "0.02"),
    ];

    // let price_and_weight_list = vec![
    //     (0, "0.42625"),
    //     (50, "0.425"),
    //     (250, "0.085"),
    //     (500, "0.0425"),
    //     (1000, "0.02125"),
    // ];

    // let price_and_weight_list = vec![
    //     (0, "0.24209"),
    //     (50, "0.425"),
    //     (150, "0.14166"),
    //     (200, "0.10625"),
    //     (250, "0.085"),
    // ];

    // let price_and_weight_list = vec![
    //     (0, "0.32709"),
    //     (50, "0.425"),
    //     (150, "0.14166"),
    //     (250, "0.085"),
    //     (1000, "0.02125"),
    // ];

    project.platform_try_update_config(
        ProjectAccount::Admin,
        &None,
        &None,
        &None,
        &None,
        &Some(
            price_and_weight_list
                .iter()
                .map(|(rewards, weight)| WeightInfo {
                    box_rewards: Uint128::new(rewards.to_owned()),
                    weight: str_to_dec(weight),
                })
                .collect(),
        ),
    )?;

    let mut stats: Vec<u128> = vec![0; price_and_weight_list.len()];
    let mut price_list: Vec<u128> = vec![];

    project.platform_try_deposit(ProjectAccount::Admin, 100 * BOX_PRICE, ProjectCoin::Stars)?;

    for _ in 0..ROUNDS {
        project.platform_try_buy(ProjectAccount::Alice, BOX_PRICE, ProjectCoin::Stars)?;
        let res = project.platform_try_open(ProjectAccount::Alice)?;

        let price = parse_attr(&res, "coins").unwrap().parse::<u128>().unwrap();
        price_list.push(price);

        let idx = price_and_weight_list
            .clone()
            .into_iter()
            .position(|(p, _w)| p == price)
            .unwrap();
        stats[idx] += 1;

        project.wait(5);
    }

    let stats = stats
        .into_iter()
        .map(|x| u128_to_dec(x) / u128_to_dec(ROUNDS))
        .collect::<Vec<Decimal>>();
    let math_exp = stats
        .iter()
        .enumerate()
        .fold(Decimal::zero(), |acc, (i, cur)| {
            acc + cur * u128_to_dec(price_and_weight_list[i].0)
        });

    assert_that(&stats).is_equal_to(
        vec!["0.463", "0.388", "0.085", "0.046", "0.018"]
            .into_iter()
            .map(|x| str_to_dec(x))
            .collect::<Vec<Decimal>>(),
    );
    assert_that(&math_exp.to_string().as_str()).is_equal_to("81.65");

    // // cumulative stats
    // let mut cumulative_price: i128 = 0;
    // let mut cumulative_price_list: Vec<i128> = vec![];

    // for price in price_list {
    //     cumulative_price = cumulative_price + (BOX_PRICE as i128) - (price as i128);
    //     cumulative_price_list.push(cumulative_price);
    // }

    // println!("{:#?}", cumulative_price_list);

    Ok(())
}

#[test]
fn opening_stats() -> StdResult<()> {
    const BOX_PRICE: u128 = 100;

    let mut project = Project::new();
    project.reset_time();

    project.platform_try_deposit(ProjectAccount::Admin, 100 * BOX_PRICE, ProjectCoin::Stars)?;

    project.platform_try_buy(ProjectAccount::Alice, 4 * BOX_PRICE, ProjectCoin::Stars)?;

    for _ in 0..3 {
        project.platform_try_open(ProjectAccount::Alice)?;
        project.wait(5);
    }

    let box_stats = project.platform_query_box_stats()?;
    let user = project.platform_query_user(ProjectAccount::Alice)?;

    let expected_opened = vec![
        OpeningInfo {
            box_rewards: Uint128::new(50),
            opened: Uint128::new(2),
        },
        OpeningInfo {
            box_rewards: Uint128::new(0),
            opened: Uint128::new(1),
        },
    ];

    assert_that(&box_stats.sold.u128()).is_equal_to(4);
    assert_that(&box_stats.opened).is_equal_to(expected_opened.clone());

    assert_that(&user.bought.u128()).is_equal_to(4);
    assert_that(&user.boxes.u128()).is_equal_to(1);
    assert_that(&user.opened).is_equal_to(expected_opened);

    Ok(())
}

#[test]
fn claim_default() -> StdResult<()> {
    const BOX_PRICE: u128 = 100;

    let mut project = Project::new();
    project.reset_time();

    project.platform_try_buy(ProjectAccount::Alice, BOX_PRICE, ProjectCoin::Stars)?;

    project.wait(11);
    let res = project.platform_try_open(ProjectAccount::Alice)?;
    let price = parse_attr(&res, "rewards")
        .unwrap()
        .parse::<u128>()
        .unwrap();
    assert_that(&price).is_equal_to(500);

    // check rewards
    let user = project.platform_query_user(ProjectAccount::Alice)?;
    let balance = project.platform_query_balance()?;

    assert_that(&user.rewards.u128()).is_equal_to(500);
    assert_that(&balance.rewards.u128()).is_equal_to(500);

    // try claim
    let res = project
        .platform_try_claim(ProjectAccount::Alice)
        .unwrap_err();
    assert_error(&res, ContractError::NotEnoughLiquidity);

    // add liquidity
    project.platform_try_deposit(ProjectAccount::Admin, 100 * BOX_PRICE, ProjectCoin::Stars)?;

    // claim
    project.platform_try_claim(ProjectAccount::Alice)?;

    // check rewards
    let user = project.platform_query_user(ProjectAccount::Alice)?;
    let balance = project.platform_query_balance()?;

    assert_that(&user.rewards.u128()).is_equal_to(0);
    assert_that(&balance.rewards.u128()).is_equal_to(0);
    assert_that(&balance.deposited.u128()).is_equal_to(100 * BOX_PRICE);

    Ok(())
}

#[test]
fn send_and_open_twice_same_time() -> StdResult<()> {
    const BOX_PRICE: u128 = 100;

    let mut project = Project::new();
    project.reset_time();

    project.platform_try_deposit(ProjectAccount::Admin, 100 * BOX_PRICE, ProjectCoin::Stars)?;

    project.platform_try_buy(ProjectAccount::Alice, 2 * BOX_PRICE, ProjectCoin::Stars)?;
    project.platform_try_send(ProjectAccount::Alice, 2, ProjectAccount::John)?;

    let alice = project.platform_query_user(ProjectAccount::Alice)?;
    let john = project.platform_query_user(ProjectAccount::John)?;

    assert_that(&alice.sent.u128()).is_equal_to(2);
    assert_that(&john.boxes.u128()).is_equal_to(2);
    assert_that(&john.received.u128()).is_equal_to(2);

    let john_balance_before = project.query_balance(ProjectAccount::John, &ProjectCoin::Stars)?;

    project.wait(11);
    project.platform_try_open(ProjectAccount::John)?;

    // try open twice same time
    let res = project.platform_try_open(ProjectAccount::John).unwrap_err();
    assert_error(&res, ContractError::MultipleBoxesPerTx);

    project.wait(1);
    project.platform_try_open(ProjectAccount::John)?;

    // check rewards
    let john = project.platform_query_user(ProjectAccount::John)?;
    let john_balance_after = project.query_balance(ProjectAccount::John, &ProjectCoin::Stars)?;

    assert_that(&john.opened.len()).is_equal_to(2);
    assert_that(&(john_balance_after - john_balance_before)).is_equal_to(550);

    Ok(())
}

#[test]
fn deposit_win_withdraw_nft() -> StdResult<()> {
    const BOX_PRICE: u128 = 100;

    let mut project = Project::new();
    project.reset_time();

    project.platform_try_deposit(ProjectAccount::Admin, 100 * BOX_PRICE, ProjectCoin::Stars)?;
    project.transfer_nft(
        ProjectAccount::Alice,
        ProjectAccount::Admin,
        ProjectNft::Gopniks,
        "1",
    );
    project.transfer_nft(
        ProjectAccount::Alice,
        ProjectAccount::Admin,
        ProjectNft::Gopniks,
        "2",
    );
    project.increase_allowances_nft(
        ProjectAccount::Admin,
        project.get_platform_address(),
        ProjectNft::Gopniks,
    );
    project.platform_try_deposit_nft(
        ProjectAccount::Admin,
        &[NftInfo {
            collection: ProjectNft::Gopniks.to_string(),
            token_id: "1".to_string(),
            price: Uint128::new(50),
        }],
    )?;
    project.platform_try_deposit_nft(
        ProjectAccount::Admin,
        &[NftInfo {
            collection: ProjectNft::Gopniks.to_string(),
            token_id: "2".to_string(),
            price: Uint128::new(50),
        }],
    )?;

    project.platform_try_buy(ProjectAccount::Kate, 100 * BOX_PRICE, ProjectCoin::Stars)?;

    project.wait(7);
    let res = project.platform_try_open(ProjectAccount::Kate)?;
    let token_id = parse_attr(&res, "token_id").unwrap();

    let (collection, cw721::TokensResponse { tokens }) =
        &project.query_all_nft(ProjectAccount::Kate)[0];

    assert_that(&token_id).is_equal_to("1".to_string());
    assert_that(&collection.to_string()).is_equal_to(ProjectNft::Gopniks.to_string());
    assert_that(&tokens.contains(&"1".to_string())).is_equal_to(true);

    let nft_info_list = project.platform_query_balance()?.nft_pool;

    project.platform_try_withdraw_nft(ProjectAccount::Owner, &nft_info_list)?;
    let (_collection, cw721::TokensResponse { tokens }) =
        &project.query_all_nft(ProjectAccount::Owner)[0];
    assert_that(&tokens.contains(&"2".to_string())).is_equal_to(true);

    Ok(())
}

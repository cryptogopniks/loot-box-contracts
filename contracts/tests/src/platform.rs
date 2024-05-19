use cw_multi_test::AppResponse;
use speculoos::assert_that;

use cosmwasm_std::{Decimal, StdResult};

use loot_box_base::converters::{str_to_dec, u128_to_dec};

use crate::helpers::{
    platform::PlatformExtension,
    suite::{
        core::Project,
        types::{ProjectAccount, ProjectCoin},
    },
};

// x0 + x1 + x2 + x3 + x4 = 1
// 0*x0 + 50*x1 + 250*x2 + 500*x3 + 1000*x4 = 80

// x0 + x1 + 0.2*x1 + 0.1*x1 + 0.05*x1 = 1
// 0*x0 + 50*x1 + 50*x1 + 50*x1 + 50*x1 = 80

// x0 + 1.35*x1 = 1
// 200*x1 = 80

// x0 = 0.4600
// x1 = 0.4000
// x2 = 0.0800
// x3 = 0.0400
// x4 = 0.0200

fn parse_attr(res: &AppResponse, key: &str) -> String {
    res.events
        .iter()
        .find(|x| x.attributes.iter().any(|y| y.key == key))
        .unwrap()
        .attributes
        .iter()
        .find(|x| x.key == key)
        .unwrap()
        .value
        .to_owned()
}

#[test]
fn opening_stats() -> StdResult<()> {
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
    let mut stats: Vec<u128> = vec![0; price_and_weight_list.len()];
    let mut price_list: Vec<u128> = vec![];

    project.platform_try_deposit(ProjectAccount::Admin, 100 * BOX_PRICE, ProjectCoin::Stars)?;

    for _ in 0..ROUNDS {
        project.platform_try_buy(ProjectAccount::Alice, BOX_PRICE, ProjectCoin::Stars)?;
        let res = project.platform_try_open(ProjectAccount::Alice)?;

        let price = parse_attr(&res, "coins").parse::<u128>().unwrap();
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
    // println!("{:#?}", stats);
    // println!("{:#?}", math_exp.to_string());

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

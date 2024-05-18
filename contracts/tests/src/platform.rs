use cw_multi_test::AppResponse;
use speculoos::assert_that;

use cosmwasm_std::{Decimal, StdResult};

use loot_box_base::converters::{str_to_dec, u128_to_dec};

use crate::helpers::{
    platform::PlatformExtension,
    suite::{core::Project, types::ProjectAccount},
};

fn parse_attr(res: &AppResponse, key: &str) -> String {
    res.events
        .last()
        .unwrap()
        .attributes
        .iter()
        .find(|x| x.key == key)
        .unwrap()
        .value
        .to_owned()
}

#[test]
fn pick_number_default() -> StdResult<()> {
    const ROUNDS: u128 = 1000;

    let mut project = Project::new();
    project.reset_time();

    let price_and_weight_list = vec![
        (0, "0.3925"),
        (50, "0.45"),
        (250, "0.09"),
        (500, "0.045"),
        (1000, "0.0225"),
    ];
    let mut stats: Vec<u128> = vec![0; price_and_weight_list.len()];
    let mut price_list: Vec<u128> = vec![];

    project.platform_try_update_config(
        ProjectAccount::Admin,
        &None,
        &None,
        &Some(100),
        &Some(price_and_weight_list.clone()),
        &None,
    )?;

    for _ in 0..ROUNDS {
        let res = project.platform_try_pick_number(ProjectAccount::Alice)?;

        let price = parse_attr(&res, "price").parse::<u128>().unwrap();
        price_list.push(price);

        let idx = price_and_weight_list
            .clone()
            .into_iter()
            .position(|(p, _w)| p == price)
            .unwrap();
        stats[idx] += 1;

        project.wait(1);
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

    assert_that(&stats).is_equal_to(vec![
        str_to_dec("0.397"),
        str_to_dec("0.434"),
        str_to_dec("0.102"),
        str_to_dec("0.045"),
        str_to_dec("0.022"),
    ]);
    assert_that(&math_exp.to_string().as_str()).is_equal_to("91.7");

    Ok(())
}

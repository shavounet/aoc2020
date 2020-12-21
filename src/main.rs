use crate::utils::GenericError;
use std::rc::Rc;
use daily_challenge::DailyChallenge;

mod utils;
mod daily_challenge;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> Result<(), GenericError> {
    println!("Hello, AOC 2020 !");
    println!("=================");


    println!("{}", day1::Day1::default().solve("src/day1/data.txt")?);
    println!("{}", day2::Day2::default().solve("src/day2/data.txt")?);
    println!("{}", day3::Day3::default().solve("src/day3/data.txt")?);

    println!("\n# Day 4");
    let day4_regex_config = Rc::new(day4::RegexConfigs::default());
    let day4_data: Vec<day4::PassportBuilder> = utils::load_data("src/day4/data.txt", "\n\n")?
        .into_iter()
        .map(|pass_builder: day4::PassportBuilder| pass_builder.set_regex_config(day4_regex_config.clone()))
        .collect::<Vec<day4::PassportBuilder>>();
    let day4_valid_count = (&day4_data).into_iter()
        .filter(|pass_builder| pass_builder.is_valid())
        .collect::<Vec<&day4::PassportBuilder>>()
        .len();
    println!(" - There is {} valid passports", day4_valid_count);
    let day4_fields_valid_count = (&day4_data).into_iter()
        .filter_map(|pass_builder|
            match pass_builder.is_fields_valid() {
                Ok(true) => Some(pass_builder),
                _ => None,
            }
        )
        .collect::<Vec<&day4::PassportBuilder>>()
        .len();
    println!(" - There is {} fully valid passports", day4_fields_valid_count);

    println!("\n# Day 5");
    let mut day5_data: Vec<day5::BoardingPass> = utils::load_data("src/day5/data.txt", "\n")?;
    day5_data.sort();
    let day5_max = day5_data.last().unwrap();
    println!(" - Max seat is {}", day5_max.get_seat_id());
    let mut day5_last_value = day5_data.first().unwrap().get_seat_id();
    let day5_my_seat = day5_data.into_iter()
        .find(|boarding_pass| {
            return if boarding_pass.get_seat_id() - day5_last_value > 1 {
                true
            } else {
                day5_last_value = boarding_pass.get_seat_id();
                false
            };
        }).unwrap();
    println!(" - My seat is {}", day5_my_seat.get_seat_id() - 1);
    Ok(())
}

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
    println!("{}", day4::Day4::default().solve("src/day4/data.txt")?);

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

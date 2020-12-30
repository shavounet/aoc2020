use crate::utils::GenericError;
use daily_challenge::DailyChallenge;

mod utils;
mod daily_challenge;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() -> Result<(), GenericError> {
    println!("Hello, AOC 2020 !");
    println!("=================");

    println!("{}", day1::Day1::default().solve("src/day1/data.txt")?);
    println!("{}", day2::Day2::default().solve("src/day2/data.txt")?);
    println!("{}", day3::Day3::default().solve("src/day3/data.txt")?);
    println!("{}", day4::Day4::default().solve("src/day4/data.txt")?);
    println!("{}", day5::Day5::default().solve("src/day5/data.txt")?);
    println!("{}", day6::Day6::default().solve("src/day6/data.txt")?);
    println!("{}", day7::Day7::default().solve("src/day7/data.txt")?);
    println!("{}", day8::Day8::default().solve("src/day8/data.txt")?);

    Ok(())
}

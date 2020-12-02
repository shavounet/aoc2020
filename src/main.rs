use crate::day2::PasswordRequirement;

mod day1;
mod day2;
mod utils;

fn main() -> Result<(), String> {
    println!("Hello, AOC 2020 !");
    println!("=================");


    println!("\n# Day 1");
    let day1_data = utils::load_data("src/day1/data.txt")?;
    match day1::find_complements2(&day1_data, 2020) {
        Some((val1, val2)) => println!(" - Found {} and {}, their product is {}", val1, val2, val1 * val2),
        None => println!(" - Could not find 2 values that match 2020...")
    }
    match day1::find_complements3(&day1_data, 2020) {
        Some((val1, val2, val3)) => println!(" - Found {}, {} and {}, their product is {}", val1, val2, val3, val1 * val2 * val3),
        None => println!(" - Could not find 3 values that match 2020...")
    }

    println!("\n# Day 2");
    let day2_data: Vec<PasswordRequirement> = utils::load_data("src/day2/data.txt")?;
    let day2_valid_count = day2_data.into_iter()
        .filter(|item| item.is_valid())
        .collect::<Vec<PasswordRequirement>>()
        .len();
    println!(" - There is {} valid passwords", day2_valid_count);

    Ok(())
}

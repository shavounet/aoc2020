use crate::utils::GenericError;

mod day1;
mod day2;
mod day3;
mod utils;

fn main() -> Result<(), GenericError> {
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
    let day2_data: Vec<day2::PasswordRequirement> = utils::load_data("src/day2/data.txt")?;
    let day2_valid_count_part1 = (&day2_data).into_iter()
        .filter(|item| item.is_valid())
        .collect::<Vec<&day2::PasswordRequirement>>()
        .len();
    println!(" - There is {} valid passwords, for part 1", day2_valid_count_part1);
    let day2_valid_count_part2 = (&day2_data).into_iter()
        .filter(|item| item.is_valid_part2())
        .collect::<Vec<&day2::PasswordRequirement>>()
        .len();
    println!(" - There is {} valid passwords, for part 2", day2_valid_count_part2);

    println!("\n# Day 3");
    let day3_data: day3::Map = utils::load_data("src/day2/data.txt")?.into();
    println!("{:?}", day3_data);
    Ok(())
}

use crate::utils::GenericError;
use crate::day3::{RowSpec, Path};

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
    let day3_data: day3::Map = utils::load_data("src/day3/data.txt")?.into();
    let day3_path = Path::new(3, 1, day3_data.len() - 1);
    let mut day3_count = 0;
    for point in day3_path {
        if day3_data.has_tree(point.0, point.1) {
            day3_count += 1;
        }
    }
    println!(" - There is {} trees", day3_count);
    let day3_count2 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].into_iter()
        .map(|(dx, dy)| Path::new(dx, dy, day3_data.len() - 1))
        .map(|path| path.filter(|&(x, y)| day3_data.has_tree(x, y)).collect::<Vec<(usize, usize)>>().len())
        .fold(1, |acc, cur| acc * cur);
    println!(" - Final count is {}", day3_count2);
    Ok(())
}

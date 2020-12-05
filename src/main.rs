use crate::utils::GenericError;
use std::rc::Rc;

mod utils;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> Result<(), GenericError> {
    println!("Hello, AOC 2020 !");
    println!("=================");


    println!("\n# Day 1");
    let day1_data = utils::load_data("src/day1/data.txt", "\n")?;
    match day1::find_complements2(&day1_data, 2020) {
        Some((val1, val2)) => println!(" - Found {} and {}, their product is {}", val1, val2, val1 * val2),
        None => println!(" - Could not find 2 values that match 2020...")
    }
    match day1::find_complements3(&day1_data, 2020) {
        Some((val1, val2, val3)) => println!(" - Found {}, {} and {}, their product is {}", val1, val2, val3, val1 * val2 * val3),
        None => println!(" - Could not find 3 values that match 2020...")
    }

    println!("\n# Day 2");
    let day2_data: Vec<day2::PasswordRequirement> = utils::load_data("src/day2/data.txt", "\n")?;
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
    let day3_data: day3::Map = utils::load_data("src/day3/data.txt", "\n")?.into();
    let day3_path = day3::Path::new(3, 1, day3_data.len() - 1);
    let mut day3_count = 0;
    for point in day3_path {
        if day3_data.has_tree(point.0, point.1) {
            day3_count += 1;
        }
    }
    println!(" - There is {} trees", day3_count);
    let day3_count2 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].into_iter()
        .map(|(dx, dy)| day3::Path::new(dx, dy, day3_data.len() - 1))
        .map(|path| path.filter(|&(x, y)| day3_data.has_tree(x, y)).collect::<Vec<(usize, usize)>>().len())
        .fold(1, |acc, cur| acc * cur);
    println!(" - Final count is {}", day3_count2);

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

mod day1;
mod utils;

fn main() -> Result<(), String> {
    println!("Hello, AOC 2020 !");
    println!("=================");


    println!("# Day 1");
    let day1_data = utils::load_data("src/day1/data.txt")?;
    match day1::find_complements2(&day1_data, 2020) {
        Some((val1, val2)) => println!("Found {} and {}, their product is {}", val1, val2, val1 * val2),
        None => println!("Could not find values that match...")
    }
    match day1::find_complements3(&day1_data, 2020) {
        Some((val1, val2, val3)) => println!("Found {}, {} and {}, their product is {}", val1, val2, val3, val1 * val2 * val3),
        None => println!("Could not find values that match...")
    }

    Ok(())
}

use crate::daily_challenge::DailyChallenge;
use crate::utils::GenericError;

#[derive(Default)]
pub struct Day10;

impl DailyChallenge for Day10 {
    type Data = usize;
    type Wrapper = Vec<usize>;

    fn get_day_num(&self) -> usize { 10 }

    fn solve_part_1(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let mut sorted_data = data.clone();
        sorted_data.sort();
        sorted_data.push(sorted_data.last().unwrap() + 3);

        let results = sorted_data.into_iter().enumerate().fold(
            (0, 0, 0, 0, None),
            |(last_value, nb1, nb2, nb3, err), (i, current_val)| {
                match current_val - last_value {
                    0 => (current_val, nb1, nb2, nb3, err),
                    1 => (current_val, nb1 + 1, nb2, nb3, err),
                    2 => (current_val, nb1, nb2 + 1, nb3, err),
                    3 => (current_val, nb1, nb2, nb3 + 1, err),
                    _ => (current_val, nb1, nb2, nb3, Some(format!("{} cannot be chained", i))),
                }
            });
        Ok(format!("result is {}", results.1 * results.3))
    }

    fn solve_part_2(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let mut sorted_data = data.clone();
        sorted_data.sort();
        sorted_data.insert(0, 0);
        sorted_data.push(sorted_data.last().unwrap() + 3);

        let mut groups = vec![];
        let mut current_start = 0;
        for i in 1..sorted_data.len() {
            if sorted_data[i] - sorted_data[i - 1] >= 3 {
                groups.push(&sorted_data[current_start..i]);
                current_start = i;
            }
        }

        let result = groups.into_iter().map(|values| {
            // A hard coded result list because why not, 0 allows us to detect failure
            match values.len() {
                1 => 1,
                2 => 1,
                3 => 2,
                4 => 4,
                5 => 7, // 2^3 - 1
                _ => 0
            }
        }).fold(1, |acc: usize, cur| acc * cur);
        Ok(format!("result is {}", result))
    }
}

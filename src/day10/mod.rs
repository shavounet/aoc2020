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
        Ok(format!("ok"))
    }
}

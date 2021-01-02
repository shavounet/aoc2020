use crate::daily_challenge::DailyChallenge;
use crate::utils::GenericError;
use std::str::FromStr;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Input(usize);

impl Input {
    pub fn is_valid(&self, previous_nums: &[Input]) -> bool {
        let len = previous_nums.len();
        for i in 0..len {
            for j in i + 1..len {
                if &previous_nums[i].0 + &previous_nums[j].0 == self.0 {
                    return true;
                }
            }
        }

        false
    }
}

impl FromStr for Input {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input(s.parse()?))
    }
}

fn find_bad_result(data: &Vec<Input>) -> Result<(usize, &Input), GenericError> {
    for i in 25..data.len() {
        let previous_nums = &data[i - 25..i];
        if !&data[i].is_valid(previous_nums) {
            return Ok((i, &data[i]));
        }
    }
    GenericError::throw("No result found")
}

fn find_contiguous_sum(data: &[Input], target: usize) -> Result<&[Input], GenericError> {
    for i in 0..data.len() - 1 {
        let mut sum = data[i].0 + data[i + 1].0;
        let mut incr = 2;
        while sum < target && i + incr < data.len() {
            sum += data[i + incr].0;
            incr += 1;
        }

        if sum == target {
            return Ok(&data[i..i + incr]);
        }
    }
    GenericError::throw("No result found")
}

#[derive(Debug, Default)]
pub struct Day9;

impl DailyChallenge for Day9 {
    type Data = Input;
    type Wrapper = Vec<Input>;

    fn get_day_num(&self) -> usize { 9 }

    fn solve_part_1(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let bad_result = find_bad_result(data)?.1.0;

        Ok(format!("the first bad value is {}", bad_result))
    }

    fn solve_part_2(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let bad_result = find_bad_result(data)?;
        let mut contiguous_set: Vec<Input> = find_contiguous_sum(&data[..bad_result.0], bad_result.1.0)?.into_iter()
            .map(|input| input.clone())
            .collect();
        contiguous_set.sort();
        let weakness = contiguous_set.first().unwrap().0 + contiguous_set.last().unwrap().0;

        Ok(format!("the weakness is {}", weakness))
    }
}

use crate::utils::{load_data, GenericError};
use std::str::FromStr;

pub trait DailyChallenge {
    type Data: FromStr;
    type Wrapper: From<Vec<Self::Data>>;

    fn get_day_num(&self) -> usize;

    fn load_data(&self, file_path: &str) -> Result<Self::Wrapper, GenericError>
        where <Self::Data as std::str::FromStr>::Err: std::error::Error
    {
        let data: Vec<Self::Data> = load_data(file_path, "\n")?;
        Ok(data.into())
    }

    fn solve_part_1(&self, data: &Self::Wrapper) -> Result<String, GenericError>;
    fn solve_part_2(&self, data: &Self::Wrapper) -> Result<String, GenericError>;

    fn solve(&self, file_path: &str) -> Result<String, GenericError>
        where <Self::Data as std::str::FromStr>::Err: std::error::Error
    {
        let mut result = String::new();
        result.push_str(format!("# Day {}", self.get_day_num()).as_str());

        let data = self.load_data(file_path)?;

        result.push_str(format!(" - Part 1 : {}", self.solve_part_1(&data)?).as_str());
        result.push_str(format!(" - Part 2 : {}", self.solve_part_2(&data)?).as_str());

        return Ok(result);
    }
}

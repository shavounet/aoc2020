use std::str::FromStr;
use crate::utils::GenericError;
use std::cmp::Ordering;
use crate::daily_challenge::DailyChallenge;

#[derive(Debug, Eq)]
pub struct BoardingPass {
    spec: String,
    row: usize,
    column: usize,
}

impl BoardingPass {
    pub fn get_seat_id(&self) -> usize {
        self.row * 8 + self.column
    }
}


impl FromStr for BoardingPass {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(GenericError::new("Invalid input".to_string()));
        }

        let mut row = 0;
        let mut column = 0;

        for char in s.chars().into_iter() {
            match char {
                'F' => row = row << 1,
                'B' => row = (row << 1) + 1,
                'L' => column = column << 1,
                'R' => column = (column << 1) + 1,
                _ => return Err(GenericError::new("Invalid char".to_string()))
            };
        }

        return Ok(BoardingPass {
            spec: s.to_string(),
            row,
            column,
        });
    }
}

impl PartialEq for BoardingPass {
    fn eq(&self, other: &Self) -> bool {
        self.spec == other.spec
    }
}

impl PartialOrd for BoardingPass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Self::cmp(self, other))
    }
}

impl Ord for BoardingPass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_seat_id().cmp(&other.get_seat_id())
    }
}

pub struct SortedBoardingPassList {
    pub boarding_pass: Vec<BoardingPass>
}

impl From<Vec<BoardingPass>> for SortedBoardingPassList {
    fn from(mut list: Vec<BoardingPass>) -> Self {
        list.sort();

        SortedBoardingPassList {
            boarding_pass: list
        }
    }
}

#[derive(Default)]
pub struct Day5 {}

impl DailyChallenge for Day5 {
    type Data = BoardingPass;
    type Wrapper = SortedBoardingPassList;

    fn get_day_num(&self) -> usize { 5 }

    fn solve_part_1(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let day5_max = data.boarding_pass.last().unwrap();
        Ok(format!("max seat is {}", day5_max.get_seat_id()))
    }

    fn solve_part_2(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let mut day5_last_value = data.boarding_pass.first().unwrap().get_seat_id();
        let day5_my_seat = (&data.boarding_pass).into_iter()
            .find(|boarding_pass| {
                return if boarding_pass.get_seat_id() - day5_last_value > 1 {
                    true
                } else {
                    day5_last_value = boarding_pass.get_seat_id();
                    false
                };
            }).unwrap();
        Ok(format!("my seat is {}", day5_my_seat.get_seat_id() - 1))
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::BoardingPass;
    use std::str::FromStr;

    #[test]
    fn build_boarding_pass() {
        let bp1_spec = "FBFBBFFRLR";
        let bp1 = BoardingPass::from_str(bp1_spec).unwrap();
        assert_eq!(bp1, BoardingPass {
            spec: bp1_spec.to_string(),
            row: 44,
            column: 5,
        });
        assert_eq!(bp1.get_seat_id(), 357);

        let bp2_spec = "BFFFBBFRRR";
        let bp2 = BoardingPass::from_str(bp2_spec).unwrap();
        assert_eq!(bp2, BoardingPass {
            spec: bp2_spec.to_string(),
            row: 70,
            column: 7,
        });
        assert_eq!(bp2.get_seat_id(), 567);

        let bp3_spec = "FFFBBBFRRR";
        let bp3 = BoardingPass::from_str(bp3_spec).unwrap();
        assert_eq!(bp3, BoardingPass {
            spec: bp3_spec.to_string(),
            row: 14,
            column: 7,
        });
        assert_eq!(bp3.get_seat_id(), 119);

        let bp4_spec = "BBFFBBFRLL";
        let bp4 = BoardingPass::from_str(bp4_spec).unwrap();
        assert_eq!(bp4, BoardingPass {
            spec: bp4_spec.to_string(),
            row: 102,
            column: 4,
        });
        assert_eq!(bp4.get_seat_id(), 820);
    }
}

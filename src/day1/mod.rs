use crate::daily_challenge::DailyChallenge;
use crate::utils::GenericError;
use std::str::FromStr;
use std::ops::{Add, Mul};
use std::fmt::{Display, Formatter};

pub fn find_complements2(input: &Vec<Expense>, goal: Expense) -> Option<(Expense, Expense)> {
    for value1 in input {
        for value2 in input {
            if value1 + value2 == goal {
                return Some((value1.clone(), value2.clone()));
            }
        }
    }

    return None;
}

pub fn find_complements3(input: &Vec<Expense>, goal: Expense) -> Option<(Expense, Expense, Expense)> {
    for value1 in input {
        for value2 in input {
            for value3 in input {
                if value1 + value2 + value3 == goal {
                    return Some((value1.clone(), value2.clone(), value3.clone()));
                }
            }
        }
    }

    return None;
}

#[derive(Default)]
pub struct Day1 {}

#[derive(Clone, Debug)]
pub struct Expense(usize);

impl DailyChallenge for Day1 {
    type Data = Expense;
    type Wrapper = Vec<Expense>;

    fn get_day_num(&self) -> usize { 1 }

    fn solve_part_1(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        match find_complements2(data, Expense(2020)) {
            Some((val1, val2)) => Ok(format!("found {} and {}, their product is {}", &val1, &val2, &val1 * &val2)),
            None => Err(GenericError::new("Could not find 2 values that match 2020...".to_string()))
        }
    }

    fn solve_part_2(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        match find_complements3(data, Expense(2020)) {
            Some((val1, val2, val3)) => Ok(format!("found {}, {} and {}, their product is {}", &val1, &val2, &val3, &val1 * &val2 * &val3)),
            None => Err(GenericError::new("Could not find 3 values that match 2020...".to_string()))
        }
    }
}

impl FromStr for Expense {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.parse()?;
        Ok(Expense(num))
    }
}

impl Add for Expense {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Expense(self.0 + rhs.0)
    }
}

impl<'a, 'b> Add<&'b Expense> for &'a Expense {
    type Output = Expense;

    fn add(self, rhs: &'b Expense) -> Self::Output {
        Expense(self.0 + rhs.0)
    }
}

impl<'a> Add<Expense> for &'a Expense {
    type Output = Expense;

    fn add(self, rhs: Expense) -> Self::Output {
        Expense(self.0 + rhs.0)
    }
}

impl<'a> Add<&'a Expense> for Expense {
    type Output = Expense;

    fn add(self, rhs: &'a Expense) -> Self::Output {
        Expense(self.0 + rhs.0)
    }
}

impl Display for Expense {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Mul for Expense {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Expense(self.0 * rhs.0)
    }
}

impl<'a, 'b> Mul<&'b Expense> for &'a Expense {
    type Output = Expense;

    fn mul(self, rhs: &'b Expense) -> Self::Output {
        Expense(self.0 * rhs.0)
    }
}

impl<'b> Mul<&'b Expense> for Expense {
    type Output = Expense;

    fn mul(self, rhs: &'b Expense) -> Self::Output {
        Expense(self.0 * rhs.0)
    }
}

impl<'a> Mul<Expense> for &'a Expense {
    type Output = Expense;

    fn mul(self, rhs: Expense) -> Self::Output {
        Expense(self.0 * rhs.0)
    }
}

impl PartialEq for Expense {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

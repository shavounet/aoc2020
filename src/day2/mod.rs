use std::str::FromStr;
use crate::utils::GenericError;

#[derive(Debug)]
pub struct PasswordRequirement {
    letter: String,
    min: usize,
    max: usize,
    password: String,
}

impl FromStr for PasswordRequirement {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spec = String::from(s);

        let spec_parts: Vec<&str> = spec.split(":").collect();
        if spec_parts.len() < 2 {
            return Err(GenericError::new("Not enough spec parts".to_string()));
        }

        let requirement_parts: Vec<&str> = spec_parts[0].split_whitespace().collect();
        if requirement_parts.len() < 2 {
            return Err(GenericError::new("Not enough requirement parts".to_string()));
        }

        let min_max_parts: Vec<&str> = requirement_parts[0].split("-").collect();
        if min_max_parts.len() < 2 {
            return Err(GenericError::new("Not enough minmax parts".to_string()));
        }

        let letter = requirement_parts[1].trim().to_string();
        if letter.len() != 1 {
            return Err(GenericError::new("Letter length is not correct".to_string()));
        };

        Ok(PasswordRequirement {
            letter,
            min: min_max_parts[0].trim().parse()?,
            max: min_max_parts[1].trim().parse()?,
            password: spec_parts[1].trim().to_string(),
        })
    }
}

impl PasswordRequirement {
    pub fn is_valid(&self) -> bool {
        let count_of_letters = self.password.chars().into_iter()
            .filter(|c| c.to_string() == self.letter)
            .collect::<Vec<char>>()
            .len();

        count_of_letters >= self.min && count_of_letters <= self.max
    }

    pub fn is_valid_part2(&self) -> bool {
        let has_first = self.password.as_str()[self.min - 1..self.min] == self.letter;
        let has_second = self.password.as_str()[self.max - 1..self.max] == self.letter;

        has_first ^ has_second
    }
}

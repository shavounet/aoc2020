use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug)]
pub struct PasswordRequirement {
    letter: String,
    min: usize,
    max: usize,
    password: String,
}

#[derive(Debug)]
pub struct PasswordRequirementError {
    message: String
}

impl From<ParseIntError> for PasswordRequirementError {
    fn from(err: ParseIntError) -> Self {
        PasswordRequirementError {
            message: err.to_string()
        }
    }
}

impl FromStr for PasswordRequirement {
    type Err = PasswordRequirementError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spec = String::from(s);

        let spec_parts: Vec<&str> = spec.split(":").collect();
        if spec_parts.len() < 2 {
            return Err(PasswordRequirementError {
                message: "Not enough spec parts".to_string()
            });
        }

        let requirement_parts: Vec<&str> = spec_parts[0].split_whitespace().collect();
        if requirement_parts.len() < 2 {
            return Err(PasswordRequirementError {
                message: "Not enough requirement parts".to_string()
            });
        }

        let min_max_parts: Vec<&str> = requirement_parts[0].split("-").collect();
        if min_max_parts.len() < 2 {
            return Err(PasswordRequirementError {
                message: "Not enough minmax parts".to_string()
            });
        }

        let letter = requirement_parts[1].trim().to_string();
        if letter.len() != 1 {
            return Err(PasswordRequirementError {
                message: "Letter length is not correct".to_string()
            });
        }

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
}

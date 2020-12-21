use std::collections::HashMap;
use std::str::FromStr;
use crate::utils::{GenericError, load_data};
use regex::Regex;
use std::rc::Rc;
use crate::daily_challenge::DailyChallenge;

#[derive(Debug)]
pub struct PassportBuilder {
    fields: HashMap<String, String>,
    regex_configs: Option<Rc<RegexConfigs>>,
}

impl FromStr for PassportBuilder {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: HashMap<String, String> = s.split_whitespace().into_iter()
            .map(|field_str| field_str.split(":").collect::<Vec<&str>>())
            .map(|field_spec| {
                if field_spec.len() != 2 {
                    Err(GenericError::new("Length to not match".to_string()))
                } else {
                    Ok((field_spec[0].to_string(), field_spec[1].to_string()))
                }
            })
            .collect::<Result<HashMap<String, String>, Self::Err>>()?;

        Ok(PassportBuilder {
            fields,
            regex_configs: None,
        })
    }
}

impl PassportBuilder {
    pub fn set_regex_config(mut self, regex_config: Rc<RegexConfigs>) -> PassportBuilder {
        self.regex_configs = Some(regex_config);
        self
    }

    pub fn is_valid(&self) -> bool {
        self.fields.contains_key("byr")
            && self.fields.contains_key("iyr")
            && self.fields.contains_key("eyr")
            && self.fields.contains_key("hgt")
            && self.fields.contains_key("hcl")
            && self.fields.contains_key("ecl")
            && self.fields.contains_key("pid")
    }

    pub fn is_fields_valid(&self) -> Result<bool, GenericError> {
        Ok(self.is_byr_valid()?
            && self.is_iyr_valid()?
            && self.is_eyr_valid()?
            && self.is_hgt_valid()?
            && self.is_hcl_valid()?
            && self.is_ecl_valid()?
            && self.is_pid_valid()?
        )
    }

    pub fn is_byr_valid(&self) -> Result<bool, GenericError> {
        self.is_valid_num("byr", 1920, 2002)
    }

    pub fn is_iyr_valid(&self) -> Result<bool, GenericError> {
        self.is_valid_num("iyr", 2010, 2020)
    }

    pub fn is_eyr_valid(&self) -> Result<bool, GenericError> {
        self.is_valid_num("eyr", 2020, 2030)
    }

    pub fn is_hgt_valid(&self) -> Result<bool, GenericError> {
        let hgt_regex = &self.regex_configs.as_ref()
            .ok_or(GenericError::new("Regex config not defined".to_string()))?
            .hgt_regex;

        self.is_valid_regex("hgt", hgt_regex)?;

        let value = self.fields.get("hgt")
            .ok_or(GenericError::new("Field not found".to_string()))?;
        let fields = hgt_regex.captures(value)
            .ok_or(GenericError::new("Field cannot be captured".to_string()))?;

        if fields.len() < 3 {
            return Ok(false);
        }

        let num: usize = fields[1].parse()?;
        match &fields[2] {
            "cm" => Ok(num >= 150 && num <= 193),
            "in" => Ok(num >= 59 && num <= 76),
            _ => Ok(false)
        }
    }

    pub fn is_hcl_valid(&self) -> Result<bool, GenericError> {
        let hcl_regex = &self.regex_configs.as_ref()
            .ok_or(GenericError::new("Regex config not defined".to_string()))?
            .hcl_regex;

        self.is_valid_regex("hcl", hcl_regex)
    }

    pub fn is_ecl_valid(&self) -> Result<bool, GenericError> {
        let ecl_regex = &self.regex_configs.as_ref()
            .ok_or(GenericError::new("Regex config not defined".to_string()))?
            .ecl_regex;

        self.is_valid_regex("ecl", ecl_regex)
    }

    pub fn is_pid_valid(&self) -> Result<bool, GenericError> {
        let pid_regex = &self.regex_configs.as_ref()
            .ok_or(GenericError::new("Regex config not defined".to_string()))?
            .pid_regex;

        self.is_valid_regex("pid", pid_regex)
    }

    fn is_valid_num(&self, field: &str, min: usize, max: usize) -> Result<bool, GenericError> {
        if !self.fields.contains_key(field) {
            return Ok(false);
        }

        let value = self.fields.get(field)
            .ok_or(GenericError::new("Field not found".to_string()))?;
        let num: usize = value.parse()?;

        Ok(num >= min && num <= max)
    }

    fn is_valid_regex(&self, field: &str, regex: &Regex) -> Result<bool, GenericError> {
        if !self.fields.contains_key(field) {
            return Ok(false);
        }
        let value = self.fields.get(field)
            .ok_or(GenericError::new("Field not found".to_string()))?;

        Ok(regex.is_match(value))
    }
}

#[derive(Debug)]
pub struct RegexConfigs {
    hgt_regex: Regex,
    hcl_regex: Regex,
    ecl_regex: Regex,
    pid_regex: Regex,
}

impl Default for RegexConfigs {
    fn default() -> Self {
        RegexConfigs {
            hgt_regex: Regex::new(r"^(\d+)(in|cm)$").unwrap(),
            hcl_regex: Regex::new(r"^#[\da-f]{6}$").unwrap(),
            ecl_regex: Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap(),
            pid_regex: Regex::new(r"^\d{9}$").unwrap(),
        }
    }
}

pub struct ValidatedPasswordList {
    pub passwords: Vec<PassportBuilder>
}

impl From<Vec<PassportBuilder>> for ValidatedPasswordList {
    fn from(passwords: Vec<PassportBuilder>) -> Self {
        let day4_regex_config = Rc::new(RegexConfigs::default());
        let passwords_with_regex: Vec<PassportBuilder> = passwords.into_iter()
            .map(|pass_builder: PassportBuilder| pass_builder.set_regex_config(day4_regex_config.clone()))
            .collect::<Vec<PassportBuilder>>();

        ValidatedPasswordList {
            passwords: passwords_with_regex
        }
    }
}

#[derive(Default)]
pub struct Day4 {}

impl DailyChallenge for Day4 {
    type Data = PassportBuilder;
    type Wrapper = ValidatedPasswordList;

    fn get_day_num(&self) -> usize { 4 }

    fn load_data(&self, file_path: &str) -> Result<Self::Wrapper, GenericError>
        where <Self::Data as std::str::FromStr>::Err: std::error::Error
    {
        let data: Vec<Self::Data> = load_data(file_path, "\n\n")?;
        Ok(data.into())
    }

    fn solve_part_1(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let count = (&data.passwords).into_iter()
            .filter(|pass_builder| pass_builder.is_valid())
            .collect::<Vec<&PassportBuilder>>()
            .len();

        Ok(format!("{} valid passports", count))
    }

    fn solve_part_2(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let day4_fields_valid_count = (&data.passwords).into_iter()
            .filter_map(|pass_builder|
                match pass_builder.is_fields_valid() {
                    Ok(true) => Some(pass_builder),
                    _ => None,
                }
            )
            .collect::<Vec<&PassportBuilder>>()
            .len();
        Ok(format!("{} fully valid passports", day4_fields_valid_count))
    }
}

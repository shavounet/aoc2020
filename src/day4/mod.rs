use std::collections::HashMap;
use std::str::FromStr;
use crate::utils::GenericError;
use regex::Regex;
use std::rc::Rc;

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

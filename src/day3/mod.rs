use std::str::FromStr;
use crate::utils::GenericError;

#[derive(Debug)]
pub struct Map {
    rows: Vec<RowSpec>
}

#[derive(Debug)]
pub struct RowSpec {
    items: Vec<Item>
}

#[derive(Debug)]
pub enum Item {
    Tree,
    None,
}

impl FromStr for RowSpec {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items:Vec<Item> = s.chars().into_iter()
            .filter(|item| item.to_string() == "." || item.to_string() == "#")
            .map(|item| item.to_string().parse())
            .collect::<Result<Vec<Item>, Self::Err>>()?;

        Ok(RowSpec { items })
    }
}

impl FromStr for Item {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(GenericError::new("Cannot use long string".to_string()));
        }

        match s {
            "." => Ok(Item::None),
            "#" => Ok(Item::Tree),
            _ => Err(GenericError::new("Unknown item spec".to_string()))
        }
    }
}

impl Into<Map> for Vec<RowSpec> {
    fn into(self) -> Map {
        Map { rows: self }
    }
}

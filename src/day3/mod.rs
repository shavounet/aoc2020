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

impl IntoIterator for Map {
    type Item = RowSpec;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

impl Map {
    pub fn has_tree(&self, x: usize, y: usize) -> bool {
        if y >= self.rows.len() {
            return false;
        }

        self.rows[y].has_tree(x)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

#[derive(Debug)]
pub struct Path {
    pub x: usize,
    pub y: usize,
    dx: usize,
    dy: usize,
    max_y: usize,
}

impl Iterator for Path {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.x += self.dx;
        self.y += self.dy;
        if self.y > self.max_y {
            return None;
        }

        Some((self.x, self.y))
    }
}

impl Path {
    pub fn new(dx: usize, dy: usize, max_y: usize) -> Self {
        Path {
            x: 0,
            y: 0,
            dx,
            dy,
            max_y,
        }
    }
}

impl RowSpec {
    pub fn has_tree(&self, index: usize) -> bool {
        if self.items.len() == 0 {
            return false;
        }

        match &self.items[index % self.items.len()] {
            Item::Tree => true,
            Item::None => false,
        }
    }
}

impl FromStr for RowSpec {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<Item> = s.chars().into_iter()
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

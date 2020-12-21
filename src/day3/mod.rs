use std::str::FromStr;
use crate::utils::GenericError;
use crate::daily_challenge::DailyChallenge;

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

impl From<Vec<RowSpec>> for Map {
    fn from(rows: Vec<RowSpec>) -> Self {
        Map { rows }
    }
}

#[derive(Default)]
pub struct Day3 {}

impl DailyChallenge for Day3 {
    type Data = RowSpec;
    type Wrapper = Map;

    fn get_day_num(&self) -> usize { 3 }

    fn solve_part_1(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let day3_path = Path::new(3, 1, data.len() - 1);
        let mut day3_count = 0;
        for point in day3_path {
            if data.has_tree(point.0, point.1) {
                day3_count += 1;
            }
        }
        Ok(format!("there is {} trees  in the path", day3_count))
    }

    fn solve_part_2(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let day3_count2 = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].into_iter()
            .map(|(dx, dy)| Path::new(dx, dy, data.len() - 1))
            .map(|path| path.filter(|&(x, y)| data.has_tree(x, y)).collect::<Vec<(usize, usize)>>().len())
            .fold(1, |acc, cur| acc * cur);
        Ok(format!("final count is {}", day3_count2))
    }
}

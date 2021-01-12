use std::str::FromStr;
use crate::utils::GenericError;
use crate::daily_challenge::DailyChallenge;
use std::cmp::{min, max};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Position {
    FreeSeat,
    OccupiedSeat,
    Floor,
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Row {
    positions: Vec<Position>
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Grid {
    rows: Vec<Row>
}

impl Grid {
    pub fn get_position(&self, x: usize, y: usize) -> Option<&Position> {
        if y >= self.rows.len() {
            return None;
        }

        let row = &self.rows[y];

        if x >= row.positions.len() {
            return None;
        }

        let position = &row.positions[x];

        Some(position)
    }

    pub fn mutate(&self) -> Self {
        let mut new_grid = Grid::default();
        for (current_y, row) in (&self.rows).into_iter().enumerate() {
            let mut new_row = Row::default();
            for (current_x, position) in (&row.positions).into_iter().enumerate() {
                let y_range = (max(1, current_y) - 1)..min(self.rows.len(), current_y + 2);
                let x_range = (max(1, current_x) - 1)..min(row.positions.len(), current_x + 2);
                let adjacent_positions = x_range.into_iter()
                    .fold(vec![], |acc, x| {
                        let xy_range = y_range.clone().into_iter()
                            .map(|y| (x, y))
                            .filter(|(i, j)| &current_x != i || &current_y != j)
                            .collect::<Vec<(usize, usize)>>();
                        acc.into_iter().chain(xy_range).collect::<Vec<(usize, usize)>>()
                    })
                    .into_iter()
                    .map(|(i, j)| self.get_position(i, j))
                    .filter(|pos| pos.is_some())
                    .map(|pos| pos.unwrap())
                    .collect::<Vec<&Position>>();

                new_row.positions.push(position.mutate(adjacent_positions));
            }

            new_grid.rows.push(new_row);
        }


        new_grid
    }

    pub fn count_occupied(&self) -> usize {
        (&self.rows).into_iter()
            .map(|row| (&row.positions).into_iter().filter(|pos| **pos == Position::OccupiedSeat).count())
            .sum()
    }
}

impl FromStr for Position {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Position::FreeSeat),
            "#" => Ok(Position::OccupiedSeat),
            "." => Ok(Position::Floor),
            _ => GenericError::throw("Invalid input for position")
        }
    }
}

impl FromStr for Row {
    type Err = GenericError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions: Vec<Position> = s.chars().into_iter().map(|c| c.to_string().parse()).collect::<Result<Vec<Position>, GenericError>>()?;

        Ok(Row { positions })
    }
}

impl From<Vec<Row>> for Grid {
    fn from(rows: Vec<Row>) -> Self {
        Grid { rows }
    }
}

impl Position {
    fn mutate(&self, adjacent_positions: Vec<&Position>) -> Position {
        if let Position::Floor = self {
            return Position::Floor;
        }

        let occupied_seat_count = adjacent_positions.into_iter()
            .filter(|pos| **pos == Position::OccupiedSeat)
            .count();

        return if *self == Position::OccupiedSeat && occupied_seat_count >= 4 {
            Position::FreeSeat
        } else if *self == Position::FreeSeat && occupied_seat_count == 0 {
            Position::OccupiedSeat
        } else {
            self.clone()
        };
    }
}

#[derive(Default)]
pub struct Day11;

impl DailyChallenge for Day11 {
    type Data = Row;
    type Wrapper = Grid;

    fn get_day_num(&self) -> usize { 11 }

    fn solve_part_1(&self, data: &Self::Wrapper) -> Result<String, GenericError> {
        let mut new_data = (data.clone(), data.mutate());

        while new_data.0 != new_data.1 {
            let mutated_data = new_data.1.mutate();
            new_data = (new_data.1, mutated_data);
        }

        Ok(format!("occupied seat count is {}", new_data.1.count_occupied()))
    }

    fn solve_part_2(&self, _data: &Self::Wrapper) -> Result<String, GenericError> {
        Ok(format!("ok"))
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::{Grid, Row};
    use std::str::FromStr;

    #[test]
    fn main_test() {
        let input = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ];


        let rows_result = input.into_iter()
            .map(|s| Row::from_str(s))
            .collect::<Result<Vec<Row>, _>>();

        assert!(rows_result.is_ok());
        let grid: Grid = rows_result.unwrap().into();

        let mut new_data = (grid.clone(), grid.mutate());

        while new_data.0 != new_data.1 {
            let mutated_data = new_data.1.mutate();
            new_data = (new_data.1, mutated_data);
        }

        assert_eq!(new_data.1.count_occupied(), 37);
    }
}


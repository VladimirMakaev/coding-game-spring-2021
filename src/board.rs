use std::{iter::FromIterator, str::FromStr, usize};

use crate::common::ParseError;

pub struct Board {
    cells: Vec<Cell>,
}

impl Board {
    pub fn get_richness(&self, i: u8) -> u8 {
        self.cells[i as usize].richness
    }

    pub fn default() -> Self {
        let default_matrix = vec![
            "0 3 1 2 3 4 5 6",
            "1 3 7 8 2 0 6 18",
            "2 3 8 9 10 3 0 1",
            "3 3 2 10 11 12 4 0",
            "4 3 0 3 12 13 14 5",
            "5 3 6 0 4 14 15 16",
            "6 3 18 1 0 5 16 17",
            "7 2 19 20 8 1 18 36",
            "8 2 20 21 9 2 1 7",
            "9 2 21 22 23 10 2 8",
            "10 2 9 23 24 11 3 2",
            "11 2 10 24 25 26 12 3",
            "12 2 3 11 26 27 13 4",
            "13 2 4 12 27 28 29 14",
            "14 2 5 4 13 29 30 15",
            "15 2 16 5 14 30 31 32",
            "16 2 17 6 5 15 32 33",
            "17 2 35 18 6 16 33 34",
            "18 2 36 7 1 6 17 35",
            "19 1 -1 -1 20 7 36 -1",
            "20 1 -1 -1 21 8 7 19",
            "21 1 -1 -1 22 9 8 20",
            "22 1 -1 -1 -1 23 9 21",
            "23 1 22 -1 -1 24 10 9",
            "24 1 23 -1 -1 25 11 10",
            "25 1 24 -1 -1 -1 26 11",
            "26 1 11 25 -1 -1 27 12",
            "27 1 12 26 -1 -1 28 13",
            "28 1 13 27 -1 -1 -1 29",
            "29 1 14 13 28 -1 -1 30",
            "30 1 15 14 29 -1 -1 31",
            "31 1 32 15 30 -1 -1 -1",
            "32 1 33 16 15 31 -1 -1",
            "33 1 34 17 16 32 -1 -1",
            "34 1 -1 35 17 33 -1 -1",
            "35 1 -1 36 18 17 34 -1",
            "36 1 -1 19 7 18 35 -1",
        ];
        return default_matrix
            .into_iter()
            .map(|x| x.parse::<Cell>().unwrap())
            .collect();
    }
}

impl FromIterator<Cell> for Board {
    fn from_iter<T: IntoIterator<Item = Cell>>(iter: T) -> Self {
        Board {
            cells: iter.into_iter().collect(),
        }
    }
}

type Edge = Option<u8>;

#[derive(PartialEq, Eq, Debug)]
pub struct Cell {
    index: u8,
    richness: u8,
    neig_0: Edge,
    neig_1: Edge,
    neig_2: Edge,
    neig_3: Edge,
    neig_4: Edge,
    neig_5: Edge,
}

impl Cell {
    pub fn new(
        index: u8,
        richness: u8,
        neig_0: Edge,
        neig_1: Edge,
        neig_2: Edge,
        neig_3: Edge,
        neig_4: Edge,
        neig_5: Edge,
    ) -> Self {
        Self {
            index,
            richness,
            neig_0,
            neig_1,
            neig_2,
            neig_3,
            neig_4,
            neig_5,
        }
    }
}

impl FromStr for Cell {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn to_edge(x: i8) -> Edge {
            if x < 0 {
                None
            } else {
                Some(x as u8)
            }
        }

        let s: Vec<i8> = s.split(' ').flat_map(|x| x.parse::<i8>()).collect();

        if s.len() == 8 {
            return Ok(Cell::new(
                s[0] as u8,
                s[1] as u8,
                to_edge(s[2]),
                to_edge(s[3]),
                to_edge(s[4]),
                to_edge(s[5]),
                to_edge(s[6]),
                to_edge(s[7]),
            ));
        }
        return Err(ParseError::UnknownInput);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse() {
        let result = "1 3 7 8 2 0 6 18".parse::<Cell>();
        assert_eq!(
            result,
            Ok(Cell::new(
                1,
                3,
                Some(7),
                Some(8),
                Some(2),
                Some(0),
                Some(6),
                Some(18)
            ))
        );
    }

    #[test]
    fn default_exists() {
        let board = Board::default();
        assert_eq!(board.cells.len(), 37);
    }
}

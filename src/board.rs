use std::{collections::HashMap, iter::FromIterator, str::FromStr, u8, usize};

use itertools::Itertools;

use crate::common::ParseError;

pub struct Delta {
    dx: i8,
    dy: i8,
    dz: i8,
}

impl Delta {
    pub fn new(dx: i8, dy: i8, dz: i8) -> Self {
        Delta { dx, dy, dz }
    }
}

pub fn index_to_coord(index: u8) -> CubeCoord {
    match index {
        0 => CubeCoord::new(0, 0, 0),
        1 => CubeCoord::new(1, -1, 0),
        2 => CubeCoord::new(1, 0, -1),
        3 => CubeCoord::new(0, 1, -1),
        4 => CubeCoord::new(-1, 1, 0),
        5 => CubeCoord::new(-1, 0, 1),
        6 => CubeCoord::new(0, -1, 1),
        7 => CubeCoord::new(2, -2, 0),
        8 => CubeCoord::new(2, -1, -1),
        9 => CubeCoord::new(2, 0, -2),
        10 => CubeCoord::new(1, 1, -2),
        11 => CubeCoord::new(0, 2, -2),
        12 => CubeCoord::new(-1, 2, -1),
        13 => CubeCoord::new(-2, 2, 0),
        14 => CubeCoord::new(-2, 1, 1),
        15 => CubeCoord::new(-2, 0, 2),
        16 => CubeCoord::new(-1, -1, 2),
        17 => CubeCoord::new(0, -2, 2),
        18 => CubeCoord::new(1, -2, 1),
        19 => CubeCoord::new(3, -3, 0),
        20 => CubeCoord::new(3, -2, -1),
        21 => CubeCoord::new(3, -1, -2),
        22 => CubeCoord::new(3, 0, -3),
        23 => CubeCoord::new(2, 1, -3),
        24 => CubeCoord::new(1, 2, -3),
        25 => CubeCoord::new(0, 3, -3),
        26 => CubeCoord::new(-1, 3, -2),
        27 => CubeCoord::new(-2, 3, -1),
        28 => CubeCoord::new(-3, 3, 0),
        29 => CubeCoord::new(-3, 2, 1),
        30 => CubeCoord::new(-3, 1, 2),
        31 => CubeCoord::new(-3, 0, 3),
        32 => CubeCoord::new(-2, -1, 3),
        33 => CubeCoord::new(-1, -2, 3),
        34 => CubeCoord::new(0, -3, 3),
        35 => CubeCoord::new(1, -3, 2),
        36 => CubeCoord::new(2, -3, 1),
        _ => todo!(),
    }
}

pub fn coord_to_index(c: CubeCoord) -> u8 {
    match c {
        CubeCoord { x: 0, y: 0, z: 0 } => 0,
        CubeCoord { x: 1, y: -1, z: 0 } => 1,
        CubeCoord { x: 1, y: 0, z: -1 } => 2,
        CubeCoord { x: 0, y: 1, z: -1 } => 3,
        CubeCoord { x: -1, y: 1, z: 0 } => 4,
        CubeCoord { x: -1, y: 0, z: 1 } => 5,
        CubeCoord { x: 0, y: -1, z: 1 } => 6,
        CubeCoord { x: 2, y: -2, z: 0 } => 7,
        CubeCoord { x: 2, y: -1, z: -1 } => 8,
        CubeCoord { x: 2, y: 0, z: -2 } => 9,
        CubeCoord { x: 1, y: 1, z: -2 } => 10,
        CubeCoord { x: 0, y: 2, z: -2 } => 11,
        CubeCoord { x: -1, y: 2, z: -1 } => 12,
        CubeCoord { x: -2, y: 2, z: 0 } => 13,
        CubeCoord { x: -2, y: 1, z: 1 } => 14,
        CubeCoord { x: -2, y: 0, z: 2 } => 15,
        CubeCoord { x: -1, y: -1, z: 2 } => 16,
        CubeCoord { x: 0, y: -2, z: 2 } => 17,
        CubeCoord { x: 1, y: -2, z: 1 } => 18,
        CubeCoord { x: 3, y: -3, z: 0 } => 19,
        CubeCoord { x: 3, y: -2, z: -1 } => 20,
        CubeCoord { x: 3, y: -1, z: -2 } => 21,
        CubeCoord { x: 3, y: 0, z: -3 } => 22,
        CubeCoord { x: 2, y: 1, z: -3 } => 23,
        CubeCoord { x: 1, y: 2, z: -3 } => 24,
        CubeCoord { x: 0, y: 3, z: -3 } => 25,
        CubeCoord { x: -1, y: 3, z: -2 } => 26,
        CubeCoord { x: -2, y: 3, z: -1 } => 27,
        CubeCoord { x: -3, y: 3, z: 0 } => 28,
        CubeCoord { x: -3, y: 2, z: 1 } => 29,
        CubeCoord { x: -3, y: 1, z: 2 } => 30,
        CubeCoord { x: -3, y: 0, z: 3 } => 31,
        CubeCoord { x: -2, y: -1, z: 3 } => 32,
        CubeCoord { x: -1, y: -2, z: 3 } => 33,
        CubeCoord { x: 0, y: -3, z: 3 } => 34,
        CubeCoord { x: 1, y: -3, z: 2 } => 35,
        CubeCoord { x: 2, y: -3, z: 1 } => 36,
        _ => todo!(),
    }
}

pub fn delta(orientation: u8) -> Delta {
    match orientation {
        0 => Delta::new(1, -1, 0),
        1 => Delta::new(1, 0, -1),
        2 => Delta::new(0, 1, -1),
        3 => Delta::new(-1, 1, 0),
        4 => Delta::new(-1, 0, 1),
        5 => Delta::new(0, -1, 1),
        _ => panic!(
            "Invalid orientation {}. Can only be a value of 0..5",
            orientation
        ),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CubeCoord {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}

impl CubeCoord {
    pub fn new(x: i8, y: i8, z: i8) -> CubeCoord {
        Self { x, y, z }
    }

    pub fn distance_to(&self, point: CubeCoord) -> u8 {
        std::cmp::max(
            (point.x - self.x).abs() as u8,
            std::cmp::max(
                (point.y - self.y).abs() as u8,
                (point.z - self.z).abs() as u8,
            ),
        )
    }

    pub fn at_distance(&self, orientation: u8, distance: u8) -> CubeCoord {
        let delta = delta(orientation);
        Self::new(
            self.x + (delta.dx * (distance as i8)),
            self.y + (delta.dy * (distance as i8)),
            self.z + (delta.dz * (distance as i8)),
        )
    }

    pub fn ring_iter(&self, radius: u8) -> impl Iterator<Item = CubeCoord> {
        let mut result = Vec::<CubeCoord>::new();
        let mut next = self.at_distance(0, radius);
        for orientation_offset in 0..6 {
            for _step in 0..radius {
                result.push(next);
                next = next.at_distance(Self::to_orientation(2 + orientation_offset), 1);
            }
        }
        result.into_iter()
    }

    fn to_orientation(step: u8) -> u8 {
        step % 6
    }
}

pub struct Board {
    by_coord: HashMap<CubeCoord, u8>,
    cells: Vec<Cell>,
    neighbors_1: Vec<Vec<u8>>,
    neighbors_2: Vec<Vec<u8>>,
    neighbors_3: Vec<Vec<u8>>,
}

impl Board {
    fn build_coord_map() -> HashMap<CubeCoord, u8> {
        (0..37u8).map(|i| (index_to_coord(i), i)).collect()
    }

    pub fn new(cells: Vec<Cell>) -> Self {
        Self {
            cells,
            by_coord: Self::build_coord_map(),
            neighbors_1: Self::build_neighbors(1),
            neighbors_2: Self::build_neighbors(2),
            neighbors_3: Self::build_neighbors(3),
        }
    }

    fn build_neighbors(d: u8) -> Vec<Vec<u8>> {
        let mut result = Vec::new();
        for index in 0..37 {
            result.push(Self::get_neighbors_from_indexes(index, d).collect_vec());
        }
        result
    }

    pub fn get_by(&self, coord: CubeCoord) -> &Cell {
        let i = self.by_coord[&coord];
        return self.cells.get(i as usize).unwrap();
    }

    pub fn get_line(
        &self,
        start: CubeCoord,
        size: u8,
        orientation: u8,
    ) -> impl Iterator<Item = &Cell> {
        let center = CubeCoord::new(0, 0, 0);
        (1..size + 1)
            .map(move |distance| start.at_distance(orientation, distance))
            .filter(move |c| c.distance_to(center) <= 3)
            .map(move |c| self.get_by(c))
            .into_iter()
    }

    fn get_neighbors_from_indexes(from_index: u8, distance: u8) -> impl Iterator<Item = u8> {
        let center = CubeCoord::new(0, 0, 0);
        let start = index_to_coord(from_index);
        (1..distance + 1)
            .map(move |r| start.ring_iter(r))
            .flatten()
            .filter(move |c| c.distance_to(center) <= 3)
            .map(|c| coord_to_index(c))
    }

    pub fn get_neighbors_indexes_by_distance(
        &self,
        from_index: u8,
        distance: u8,
    ) -> impl Iterator<Item = &u8> {
        match distance {
            1 => self.neighbors_1[from_index as usize].iter(),
            2 => self.neighbors_2[from_index as usize].iter(),
            3 => self.neighbors_3[from_index as usize].iter(),
            _ => panic!("distance = {}", distance),
        }
    }

    pub fn get_neighbors_from(&self, from_index: u8, distance: u8) -> impl Iterator<Item = &Cell> {
        self.get_neighbors_indexes_by_distance(from_index, distance)
            .map(move |i| &self.cells[*i as usize])
    }

    pub fn get_richness(&self, i: u8) -> u8 {
        self.cells[i as usize].richness
    }

    pub fn default_with_inactive(inactive_cells: impl Iterator<Item = u8>) -> Self {
        let inactive_vec: Vec<_> = inactive_cells.collect();
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
            .map(|mut c| {
                if inactive_vec.contains(&c.index) {
                    c.richness = 0
                }
                c
            })
            .collect();
    }

    pub fn default() -> Self {
        Self::default_with_inactive(Vec::new().into_iter())
    }
}

impl FromIterator<Cell> for Board {
    fn from_iter<T: IntoIterator<Item = Cell>>(iter: T) -> Self {
        Board::new(iter.into_iter().collect())
    }
}

type Edge = Option<u8>;

#[derive(PartialEq, Eq, Debug)]
pub struct Cell {
    pub index: u8,
    pub richness: u8,
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
    use std::cell;

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

    #[test]
    fn test_coordinates_at_distance() {
        let start = CubeCoord::new(0, 0, 0);
        assert_eq!(CubeCoord::new(0, 3, -3), start.at_distance(2, 3));

        let start = CubeCoord::new(2, -3, 1);
        assert_eq!(CubeCoord::new(1, -3, 2), start.at_distance(4, 1));

        let start = CubeCoord::new(-2, 0, 2);
        assert_eq!(CubeCoord::new(0, -2, 2), start.at_distance(0, 2));

        let start = CubeCoord::new(2, 0, -2);
        assert_eq!(CubeCoord::new(1, 1, -2), start.at_distance(3, 1));
    }

    #[test]
    fn test_cicle() {
        let start = CubeCoord::new(0, 0, 0);
        let ring: Vec<CubeCoord> = start.ring_iter(2).collect();
        assert_eq!(
            vec![
                CubeCoord::new(2, -2, 0),
                CubeCoord::new(2, -1, -1),
                CubeCoord::new(2, 0, -2),
                CubeCoord::new(1, 1, -2),
                CubeCoord::new(0, 2, -2),
                CubeCoord::new(-1, 2, -1),
                CubeCoord::new(-2, 2, 0),
                CubeCoord::new(-2, 1, 1),
                CubeCoord::new(-2, 0, 2),
                CubeCoord::new(-1, -1, 2),
                CubeCoord::new(0, -2, 2),
                CubeCoord::new(1, -2, 1),
            ],
            ring
        );
    }

    #[test]
    fn print_mapping() {
        let mut index = 1;
        let start = CubeCoord::new(0, 0, 0);
        for radius in 1..4 {
            for coor in start.ring_iter(radius) {
                println!(
                    "{} => CubeCoord::new({},{},{}),",
                    index, coor.x, coor.y, coor.z
                );
                index += 1;
            }
        }
    }

    #[test]
    fn test_neighbors() {
        let board = Board::default();
        let cells: Vec<_> = board.get_neighbors_from(27, 1).map(|c| c.index).collect();
        assert_eq!(vec![12, 26, 28, 13], cells);

        let cells: Vec<_> = board.get_neighbors_from(33, 2).map(|c| c.index).collect();
        assert_eq!(vec![34, 17, 16, 32, 35, 18, 6, 5, 15, 31], cells);
    }
}

use core::panic;
use std::{collections::HashMap, iter::FromIterator, str::FromStr, string, usize};

use crate::common::ParseError;
use crate::parse::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tree {
    index: u8,
    size: u8,
    is_mine: bool,
    is_dormant: bool,
}

impl Tree {
    pub fn index(&self) -> u8 {
        self.index
    }
    pub fn size(&self) -> u8 {
        self.size
    }
    pub fn is_mine(&self) -> bool {
        self.is_mine
    }
    pub fn is_dormant(&self) -> bool {
        self.is_dormant
    }
}

pub struct TreeCollection {
    trees: HashMap<u8, Tree>,
    trees_by_size: Vec<u8>,
}

impl TreeCollection {
    pub fn empty() -> Self {
        Self::new(HashMap::new())
    }

    pub fn get(&self, index: u8) -> &Tree {
        self.trees.get(&index).unwrap()
    }

    pub fn has_at(&self, index: u8) -> bool {
        self.trees.contains_key(&index)
    }

    pub fn new(map: HashMap<u8, Tree>) -> Self {
        let mut trees_by_size: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
        for (_, t) in &map {
            match (t.is_mine, t.size) {
                (true, x) if x <= 3 => trees_by_size[t.size as usize] += 1,
                (false, y) if y <= 3 => trees_by_size[t.size as usize + 4] += 1,
                _ => panic!("Incorrect size: {} for is_mine: {}", t.size, t.is_mine),
            }
        }

        Self {
            trees: map,
            trees_by_size,
        }
    }

    pub fn len(&self) -> usize {
        self.trees.len()
    }

    pub fn get_amount_of_size(&self, size: u8, is_mine: bool) -> u8 {
        let offset = if is_mine { 0 } else { 4 };
        self.trees_by_size[offset + size as usize]
    }

    pub fn my_trees(&self) -> impl Iterator<Item = &Tree> {
        self.trees.iter().filter(|(i, t)| t.is_mine).map(|(_, t)| t)
    }

    #[cfg(test)]
    pub fn from_strings<'a, T>(strings: T) -> Self
    where
        T: IntoIterator<Item = &'a str>,
    {
        strings
            .into_iter()
            .flat_map(|x| x.parse::<Tree>())
            .collect()
    }
}

impl FromIterator<Tree> for TreeCollection {
    fn from_iter<T: IntoIterator<Item = Tree>>(iter: T) -> Self {
        TreeCollection::new(iter.into_iter().map(|t| (t.index, t)).collect())
    }
}

impl Tree {
    pub fn new(index: u8, size: u8, is_mine: bool, is_dormant: bool) -> Self {
        Self {
            index,
            size,
            is_mine,
            is_dormant,
        }
    }

    pub fn not_dormant(&self) -> bool {
        return !self.is_dormant;
    }
}

impl FromStr for Tree {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inputs: Vec<u8> = Next::read_many_from(s);
        if inputs.len() != 4 {
            return Err(ParseError::InvalidParameters);
        } else {
            return Ok(Tree::new(
                inputs[0],
                inputs[1],
                inputs[2] == 1,
                inputs[3] == 1,
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let t = "29 1 1 0".parse::<Tree>();
        assert_eq!(t, Ok(Tree::new(29, 1, true, false)))
    }
}

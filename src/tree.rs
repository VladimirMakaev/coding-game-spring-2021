use core::panic;
use std::{
    collections::HashMap,
    fmt::{write, Debug},
    iter::FromIterator,
    str::FromStr,
    string, usize,
};

use itertools::Itertools;

use crate::common::ParseError;
use crate::parse::*;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Tree {
    index: u8,
    size: u8,
    is_mine: bool,
    is_dormant: bool,
}

impl Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(index:{}, size:{})", self.index, self.size)
    }
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

    pub fn set_dormant(&mut self, is_dormant: bool) {
        self.is_dormant = is_dormant;
    }

    pub fn grow_size(&mut self) {
        self.size += 1;
        self.set_dormant(true);
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TreeCollection {
    _trees: Vec<Option<Tree>>,
    trees: Vec<Tree>,
    trees_by_size: Vec<u8>,
}

impl Debug for TreeCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "trees: {:?}", self.trees)
    }
}

impl TreeCollection {
    fn size_index(index: u8, is_player: bool) -> usize {
        let offset: usize = if is_player { 0 } else { 4 };
        return offset + index as usize;
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn seed(&mut self, index: u8, is_player: bool) {
        self.trees.push(Tree::new(index, 0, is_player, true));
        self.trees_by_size[Self::size_index(0, is_player)] += 1;
    }

    pub fn remove(&mut self, index: u8) {
        let position = self.trees.iter().find_position(|t| t.index == index);
        if let Some((i, t)) = position {
            let offset: usize = if t.is_mine { 0 } else { 4 };
            self.trees_by_size[offset + t.size as usize] -= 1;
            self.trees.remove(i);
        }
    }

    pub fn get(&self, index: u8) -> &Tree {
        self.trees.iter().find(|t| t.index == index).unwrap()
    }

    pub fn get_mut(&mut self, index: u8) -> &mut Tree {
        self.trees.iter_mut().find(|t| t.index == index).unwrap()
    }

    pub fn has_at(&self, index: u8) -> bool {
        self.trees.iter().any(|t| t.index == index)
    }

    pub fn new(map: Vec<Tree>) -> Self {
        let mut trees_by_size: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
        for t in &map {
            match (t.is_mine, t.size) {
                (true, x) if x <= 3 => trees_by_size[t.size as usize] += 1,
                (false, y) if y <= 3 => trees_by_size[t.size as usize + 4] += 1,
                _ => panic!("Incorrect size: {} for is_mine: {}", t.size, t.is_mine),
            }
        }

        Self {
            _trees: Vec::with_capacity(37),
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
        self.trees.iter().filter(|t| t.is_mine)
    }

    pub fn iter_trees_for(&self, is_player: bool) -> impl Iterator<Item = &Tree> {
        self.trees.iter().filter(move |t| t.is_mine == is_player)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tree> {
        self.trees.iter()
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
        TreeCollection::new(iter.into_iter().collect())
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

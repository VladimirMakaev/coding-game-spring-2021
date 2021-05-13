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
        write!(
            f,
            "(index:{}, size:{}, is_mine:{}, is_dormant:{})",
            self.index, self.size, self.is_mine, self.is_dormant
        )
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
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TreeCollection {
    trees: Vec<Option<Tree>>,
    trees_by_size: Vec<u8>,
}

/*
impl Debug for TreeCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "trees: {:?}", self.trees)
    }
}*/

impl TreeCollection {
    fn size_index(index: u8, is_player: bool) -> usize {
        let offset: usize = if is_player { 0 } else { 4 };
        return offset + index as usize;
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn seed(&mut self, index: u8, is_player: bool) {
        self.trees[index as usize] = Some(Tree::new(index, 0, is_player, true));
        self.trees_by_size[Self::size_index(0, is_player)] += 1;
    }

    pub fn remove(&mut self, index: u8) {
        if let Some(t) = std::mem::replace(&mut self.trees[index as usize], None) {
            self.trees_by_size[Self::size_index(t.size, t.is_mine)] -= 1;
        }
    }

    pub fn get(&self, index: u8) -> &Tree {
        if let Some(Some(ref x)) = self.trees.get(index as usize) {
            return x;
        }
        panic!("Invalid index");
    }

    pub fn get_mut(&mut self, index: u8) -> &mut Tree {
        if let Some(Some(ref mut x)) = self.trees.get_mut(index as usize) {
            return x;
        }
        panic!("Invalid index");
    }

    pub fn grow_size(&mut self, index: u8) {
        if let Some(ref mut x) = self.trees.get_mut(index as usize).unwrap() {
            x.size += 1;
            x.is_dormant = true;
            self.trees_by_size[Self::size_index(x.size, x.is_mine)] += 1;
            self.trees_by_size[Self::size_index(x.size - 1, x.is_mine)] -= 1;
        }
    }

    pub fn wake_up(&mut self) {
        for item in self.trees.iter_mut() {
            if let Some(ref mut t) = item {
                t.set_dormant(false);
            }
        }
    }

    pub fn has_at(&self, index: u8) -> bool {
        if let Some(Some(_)) = self.trees.get(index as usize) {
            return true;
        }
        return false;
    }

    pub fn new(map: Vec<Tree>) -> Self {
        let mut trees_by_size: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0];
        let mut _trees: Vec<Option<Tree>> = (0..37).map(|_| None).collect_vec();

        for t in map {
            match (t.is_mine, t.size) {
                (true, x) if x <= 3 => trees_by_size[t.size as usize] += 1,
                (false, y) if y <= 3 => trees_by_size[t.size as usize + 4] += 1,
                _ => panic!("Incorrect size: {} for is_mine: {}", t.size, t.is_mine),
            }
            let i = t.index();
            _trees[i as usize] = Some(t);
        }

        Self {
            trees: _trees,
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
        self.trees.iter().flatten().filter(|t| t.is_mine)
    }

    pub fn iter_trees_for(&self, is_player: bool) -> impl Iterator<Item = &Tree> {
        self.trees
            .iter()
            .flatten()
            .filter(move |t| t.is_mine == is_player)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tree> {
        self.trees.iter().flatten()
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
    use crate::tree;

    use super::*;

    #[test]
    fn it_parses() {
        let t = "29 1 1 0".parse::<Tree>();
        assert_eq!(t, Ok(Tree::new(29, 1, true, false)))
    }

    #[test]
    fn test_seed_tree() {
        let mut trees: TreeCollection = vec![Tree::new(0, 1, true, false)].into_iter().collect();
        trees.seed(1, true);
        assert_eq!(trees.get(1), &Tree::new(1, 0, true, true));
        assert_eq!(trees.get_amount_of_size(1, true), 1);
        assert_eq!(trees.get_amount_of_size(0, true), 1);
    }

    #[test]
    fn test_complete_tree() {
        let mut trees: TreeCollection = vec![Tree::new(20, 3, true, false)].into_iter().collect();
        trees.remove(20);
        assert_eq!(trees.has_at(3), false);
        assert_eq!(trees.get_amount_of_size(3, true), 0);
    }

    #[test]
    fn it_works() {
        let mut trees: TreeCollection = vec![
            "0 1 1 0", "1 1 1 0", "2 2 1 0", "3 2 1 0", "4 2 0 0", "5 2 0 0", "6 2 1 1",
            "10 0 1 0", "11 0 1 1", "14 1 0 0", "17 1 0 0", "18 1 1 1", "21 3 1 1", "26 3 1 0",
            "30 1 0 0", "35 1 0 0",
        ]
        .into_iter()
        .map(|t| t.parse())
        .flatten()
        .collect();

        assert_eq!(
            trees.my_trees().map(|x| x.index()).collect_vec(),
            vec![0, 1, 2, 3, 6, 10, 11, 18, 21, 26]
        );

        assert_eq!(trees.get_amount_of_size(2, true), 3);
        assert_eq!(trees.get_amount_of_size(2, false), 2);

        trees.grow_size(1);

        assert_eq!(trees.get(1).size(), 2);

        assert_eq!(trees.get_amount_of_size(1, false), 4);

        assert_eq!(
            trees.iter_trees_for(true).map(|x| x.index()).collect_vec(),
            vec![0, 1, 2, 3, 6, 10, 11, 18, 21, 26]
        );
        trees.remove(18);
        assert_eq!(
            trees.iter_trees_for(true).map(|x| x.index()).collect_vec(),
            vec![0, 1, 2, 3, 6, 10, 11, 21, 26]
        );
    }
}

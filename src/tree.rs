use std::{collections::HashMap, iter::FromIterator, str::FromStr};

use crate::common::ParseError;
use crate::parse::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tree {
    index: u8,
    size: u8,
    is_mine: bool,
    is_dormant: bool,
}

pub struct TreeCollection {
    trees: HashMap<u8, Tree>,
}

impl TreeCollection {
    pub fn empty() -> Self {
        Self::new(HashMap::new())
    }

    pub fn new(map: HashMap<u8, Tree>) -> Self {
        Self { trees: map }
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

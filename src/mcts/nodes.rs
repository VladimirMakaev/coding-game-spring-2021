use std::usize;
pub type TreeIndex = usize;
pub type Score = f32;

use crate::engine::actions::Action;

pub struct ActionNode {
    action: Action,
    score: Score,
    wins: u32,
    is_player: bool,
    parent: TreeIndex,
    is_explored: bool,
    child_nodes: Vec<TreeIndex>,
}

pub struct StateNode {
    score: Score,
    wins: u32,
    parent: TreeIndex,
    game: TreeIndex,
    is_explored: bool,
    child_nodes: Vec<TreeIndex>,
}

pub trait GameNode {
    fn parent(&self) -> TreeIndex;
    fn child_nodes(&self) -> Vec<TreeIndex>;
    fn is_expanded(&self) -> bool {
        self.child_nodes().len() > 0
    }
}

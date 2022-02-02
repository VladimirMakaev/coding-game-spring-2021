use std::usize;
pub type TreeIndex = usize;
pub type Score = f32;

use crate::engine::actions::Action;

pub struct PlayerNode {
    action: Action,
    score: Score,
    picks: u32,
    parent_state: TreeIndex,
    child_actions: Vec<TreeIndex>,
}

pub struct EnemyNode {
    action: Action,
    score: Score,
    picks: u32,
    parent_action: TreeIndex,
    child_state: TreeIndex,
}

pub struct StateNode {
    score: Score,
    wins: u32,
    parent: TreeIndex,
    game: TreeIndex,
    is_explored: bool,
    child_nodes: Vec<TreeIndex>,
    merged_parents: Vec<TreeIndex>,
}

pub trait GameNode {
    type TChild;
    type TParent;

    fn parent(&self) -> TreeIndex;

    fn child_nodes(&self) -> Vec<TreeIndex>;

    fn is_expanded(&self) -> bool {
        self.child_nodes().len() > 0
    }
}

impl GameNode for StateNode {
    type TChild = PlayerNode;

    type TParent = EnemyNode;

    fn parent(&self) -> TreeIndex {
        todo!()
    }

    fn child_nodes(&self) -> Vec<TreeIndex> {
        todo!()
    }
}

impl GameNode for PlayerNode {
    type TChild = EnemyNode;

    type TParent = StateNode;

    fn parent(&self) -> TreeIndex {
        todo!()
    }

    fn child_nodes(&self) -> Vec<TreeIndex> {
        todo!()
    }
}

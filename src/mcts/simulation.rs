use std::usize;

use crate::engine::board::*;
use crate::engine::game::*;

use super::nodes::{ActionNode, GameNode, StateNode, TreeIndex};

pub struct MctsSimulation<'a> {
    board: &'a Board,
    games: Vec<Game>,
    nodes: Vec<ActionNode>,
    states: Vec<StateNode>,
}

impl<'a> MctsSimulation<'a> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_or_create_state(game: &Game) -> TreeIndex {
        todo!()
    }

    pub fn iterate(&mut self, state: TreeIndex) {
        let current_state = state;
        loop {
            let ref x = self.states[current_state as usize];
        }
    }

    pub fn ensure_expanded<T: GameNode>(&mut self, node_id: TreeIndex) {
        todo!()
    }
}

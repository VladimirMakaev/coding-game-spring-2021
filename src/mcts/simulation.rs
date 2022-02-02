use std::usize;

use crate::engine::board::*;
use crate::engine::game::*;

use super::nodes::{EnemyNode, GameNode, PlayerNode, StateNode, TreeIndex};

pub struct MctsSimulation<'a> {
    board: &'a Board,
    games: Vec<Game>,
    player_nodes: Vec<PlayerNode>,
    enemy_nodes: Vec<EnemyNode>,
    states: Vec<StateNode>,
    x: Option<StateNode>,
}

impl<'a> MctsSimulation<'a> {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_or_create_state(&mut self, game: &Game) -> TreeIndex {
        if let Some(ref mut x) = self.x {}
        todo!()
    }

    pub fn iterate(&mut self, state: TreeIndex) {
        let current_state = state;
        loop {
            self.ensure_expanded::<StateNode>(current_state);

            for player_node_id in self.states[current_state].child_nodes() {
                for enemy_node_id in self.player_nodes[player_node_id].child_nodes() {
                    self.states.push(Self::create_state_node());
                }
            }
        }
    }

    fn create_state_node() -> StateNode {
        todo!()
    }

    pub fn ensure_expanded<T: GameNode>(&mut self, node_id: TreeIndex) {
        todo!()
    }
}

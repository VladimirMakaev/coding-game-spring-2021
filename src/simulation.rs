use std::{collections::HashMap, usize};

use crate::{actions::Action, game::Game};

pub struct Simulation<'a> {
    free_nodes: Vec<usize>,
    free_games: Vec<usize>,
    nodes: Vec<Node>,
    states: Vec<State>,
    state_by_games: HashMap<&'a Game, u32>,
}

impl<'a> Simulation<'a> {
    pub fn new() -> Self {
        Self {
            free_nodes: Vec::new(),
            nodes: Vec::new(),
            states: Vec::new(),
            state_by_games: HashMap::new(),
            free_games: Vec::new(),
        }
    }

    pub fn set_state(&mut self, game: Game) {}
}

pub struct State {
    game: Game,
    child_nodes: Vec<u32>,
}

pub struct Node {
    action: Action,
    wins: u32,
    picks: u32,
    parent: u32,
    game: u32,
    is_player: bool,
    children: Vec<u32>,
}

pub struct PlayerMove {
    action: Action,
    parent: u32,
    wins: u32,
    picks: u32,
    enemy_moves: Vec<EnemyMove>,
}

pub struct EnemyMove {
    action: Action,
    parent: u32,
    wins: u32,
    picks: u32,
    state: u32,
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, mem::size_of};

    use crate::{
        board::Board,
        game::Game,
        tree::{Tree, TreeCollection},
    };

    #[test]
    pub fn test_tree() {}

    #[test]
    pub fn debug_sizes() {
        let size = size_of::<Game>();

        println!(
            "{}, {}, {}, {}, {}, {}, {}",
            size_of::<Game>(),
            size_of::<&Board>(),
            size_of::<Tree>(),
            size_of::<HashMap<u8, Tree>>(),
            size_of::<TreeCollection>(),
            size_of::<Vec<u8>>(),
            size_of::<HashMap<u8, u8>>()
        );
    }

    pub fn test_allocation() {
        let limit = 1_000_00;
        let mut vec = Vec::with_capacity(limit);
        let trees: TreeCollection = TreeCollection::from_strings(vec!["5 2 1 0"]);
        let board = Board::default();
        let game = Game::new(trees, 20, 10, 20, 10);
        for i in 0..limit {
            vec.push(game.clone());
        }
    }
}

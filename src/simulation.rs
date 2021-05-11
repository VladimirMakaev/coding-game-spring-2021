use std::{cmp::Ordering, collections::HashMap, f64::consts::SQRT_2, u32, usize};

use crate::{actions::Action, board::Board, game::Game};

pub struct Simulation<'a> {
    board: &'a Board,
    free_nodes: Vec<usize>,
    free_games: Vec<usize>,
    nodes: Vec<Node>,
    states: Vec<State>,
    state_by_games: HashMap<&'a Game, u32>,
    current_state: u32,
}

impl<'a> Simulation<'a> {
    pub fn new(board: &'a Board, game: Game) -> Self {
        Self {
            current_state: todo!(),
            board: board,
            free_nodes: Vec::new(),
            nodes: Vec::new(),
            states: Vec::new(),
            state_by_games: HashMap::new(),
            free_games: Vec::new(),
        }
    }

    fn init_state(game: Game) -> u32 {
        todo!()
    }

    fn create_state(&mut self, game: Game) -> u32 {
        let state_id = self.states.len() as u32;
        let enemy_moves = Action::find_next_actions(&game, self.board, false);
        let player_moves = Action::find_next_actions(&game, self.board, true);
        let mut player_moves_ids = Vec::new();
        for i in 0..player_moves.len() {
            self.nodes
                .push(Node::new(state_id, state_id, player_moves[i], true));
            let mut enemy_ids = Vec::new();
            let player_move_id = (self.nodes.len() - 1) as u32;
            player_moves_ids.push(player_move_id);
            for j in 0..enemy_moves.len() {
                self.nodes
                    .push(Node::new(state_id, player_move_id, enemy_moves[j], false));
                enemy_ids.push(self.nodes.len() as u32 - 1);
            }
        }
        // self.state_by_games.insert(&game, state_id);

        self.states.push(State {
            child_nodes: player_moves_ids,
            game: game,
            picks: 0,
            wins: 0,
        });
        return state_id;
    }

    pub fn set_current_state(&mut self, game: Game) {
        if let Some(state_index) = self.state_by_games.get(&game) {
            self.current_state = *state_index;
        } else {
            self.create_state(game);
        }
    }

    fn pick_node_by_ucb(&self, parent_id: u32, is_player: bool) -> (u32, &Node) {
        let (parent_picks, nodes) = if is_player {
            let s = &self.states[parent_id as usize];
            (s.picks, &s.child_nodes)
        } else {
            let s = &self.nodes[parent_id as usize];
            (s.picks, &s.child_nodes)
        };
        let id = nodes
            .into_iter()
            .max_by(|x, y| {
                self.nodes[**x as usize]
                    .ucb(parent_picks)
                    .partial_cmp(&self.nodes[**y as usize].ucb(parent_picks))
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap();
        (*id, &self.nodes[*id as usize])
    }

    pub fn simulate(&mut self, state: u32) {
        let state_node = self.states.get_mut(state as usize).unwrap();
        let (p_id, player_node) = self.pick_node_by_ucb(state, true);
        let (e_id, enemy_node) = self.pick_node_by_ucb(p_id, false);

        let next_state: u32 = todo!();
        let state: &mut State = todo!();

        if state.game.day == 25 {
            //is finished
            if state.game.get_sun_points(true) > 100 { //player won
                 //update_score(1)
            } else {
                //update.score(0)
            }
        } else {
            self.simulate(next_state);
        }
    }
}

pub struct State {
    game: Game,
    child_nodes: Vec<u32>,
    wins: u32,
    picks: u32,
}

pub struct Node {
    action: Action,
    wins: u32,
    picks: u32,
    parent: u32,
    game: u32,
    is_player: bool,
    child_nodes: Vec<u32>,
}

impl Node {
    pub fn new(game_id: u32, parent_action: u32, action: Action, is_player: bool) -> Self {
        Node {
            action,
            wins: 0,
            picks: 0,
            parent: parent_action,
            child_nodes: Vec::new(),
            game: game_id,
            is_player: is_player,
        }
    }
    pub fn visited(&self) -> bool {
        self.picks > 0
    }

    pub fn ucb(&self, parent_picks: u32) -> f64 {
        if self.picks == 0 {
            f64::MAX
        } else {
            (self.wins as f64 / self.picks as f64)
                + SQRT_2 * ((parent_picks as f64).ln() / self.picks as f64).sqrt()
        }
    }
}

trait GameNode {
    type Parent: GameNode;

    fn wins() -> u32;

    fn picks() -> u32;

    fn get_parent(&self, simulation: &Simulation) -> (u32, &Self::Parent);

    fn get_state(&self, simulation: &Simulation) -> (u32, &State);
}

pub struct PlayerNode {
    action: Action,
    parent_state: u32,
    wins: u32,
    picks: u32,
}

pub struct EnemyNode {
    action: Action,
    parent_action: u32,
    wins: u32,
    picks: u32,
    parent_state: u32,
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

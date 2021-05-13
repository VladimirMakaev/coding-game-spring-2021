use std::{cmp::Ordering, collections::HashMap, f64::consts::SQRT_2, u32, usize};

use itertools::Itertools;

use crate::{actions::Action, board::Board, common::random_max, game::Game};

pub struct Simulation<'a> {
    board: &'a Board,
    player_nodes: Vec<PlayerNode>,
    enemy_nodes: Vec<EnemyNode>,
    free_nodes: Vec<usize>,
    free_games: Vec<usize>,
    states: Vec<State>,
    state_by_games: HashMap<&'a Game, u32>,
    current_state: u32,
}

impl<'a> Simulation<'a> {
    pub fn new(board: &'a Board, game: Game) -> Self {
        let mut result = Self {
            current_state: 0,
            board: board,
            free_nodes: Vec::with_capacity(1_000_000),
            states: Vec::with_capacity(1_000_000),
            state_by_games: HashMap::new(),
            free_games: Vec::with_capacity(1_000_000),
            player_nodes: Vec::with_capacity(1_000_000),
            enemy_nodes: Vec::with_capacity(1_000_000),
        };
        result.current_state = result.create_state(game, None).0;
        result.states[0].picks = 1;
        return result;
    }

    pub fn get_moves_summary(&self) -> impl Iterator<Item = &PlayerNode> {
        let state = State::get_node(self.current_state, &self);
        state
            .children()
            .map(move |c| Self::get_node::<State>(self, *c))
            .into_iter()
    }

    fn create_state(&mut self, game: Game, parent: Option<u32>) -> (u32, &State) {
        let state_id = self.states.len() as u32;
        let enemy_moves = Action::find_next_actions(&game, self.board, false);
        let player_moves = Action::find_next_actions(&game, self.board, true);
        let mut player_moves_ids = Vec::new();
        for i in 0..player_moves.len() {
            player_moves_ids.push(PlayerNode::create(
                state_id,
                self,
                player_moves[i],
                enemy_moves.clone(),
            ));
        }
        // self.state_by_games.insert(&game, state_id);

        let value = State {
            child_nodes: player_moves_ids,
            game: game,
            picks: 0,
            wins: 0,
            parent: parent,
        };
        self.states.push(value);
        return (state_id, &self.states[self.states.len() - 1]);
    }

    fn get_node<'b, T: HasChildren>(sim: &'b Simulation, id: u32) -> &'b T::Child {
        <<T as HasChildren>::Child as GameNode>::get_node(id, sim)
    }

    fn compare_by_ucb<T: HasChildren + GameNode>(&self, x: u32, y: u32) -> Ordering {
        let ucb1 = Self::get_node::<T>(self, x).ucb(self);
        let ucb2 = Self::get_node::<T>(self, y).ucb(self);
        ucb1.partial_cmp(&ucb2).unwrap_or(Ordering::Equal)
    }

    fn pick_node_by_ucb_2<T: HasChildren + GameNode>(&self, node: &T) -> (u32, &T::Child) {
        let max_child =
            random_max(node.children(), |x, y| self.compare_by_ucb::<T>(**x, **y)).unwrap();
        (*max_child, Self::get_node::<T>(self, *max_child))
    }

    fn create_next_game(sim: &Simulation, node: &EnemyNode) -> Game {
        let (_, player_node) = node.get_parent(sim);
        let (_, parent_state) = node.get_state(sim);
        let game = parent_state
            .game
            .apply_actions(sim.board, player_node.action, node.action);
        game
    }

    fn get_or_create_next_state(&mut self, enemy_id: u32) -> u32 {
        let ref enemy_node = self.enemy_nodes[enemy_id as usize];

        if let Some(next_state) = enemy_node.next_state {
            return next_state;
        }
        let new_game = Self::create_next_game(self, enemy_node);
        let (state_id, _) = self.create_state(new_game, Some(enemy_id));

        state_id
    }

    pub fn simulate_current(&mut self) {
        self.simulate(self.current_state);
    }

    pub fn simulate(&mut self, state: u32) {
        let mut state_id = state;
        loop {
            let state = State::get_node(state_id, self);

            let (_, player_node) = Self::pick_node_by_ucb_2(&self, state);
            let (enemy_id, _) = Self::pick_node_by_ucb_2(&self, player_node);
            let next_state_id = self.get_or_create_next_state(enemy_id);
            let next_state = State::get_node(next_state_id, self);

            let ref next_game = next_state.game;

            if next_game.day == 24 {
                let player_won = next_game.is_player_won();
                self.on_player_won(next_state_id, player_won);
                break;
            }
            state_id = next_state_id;
        }
    }

    pub fn mark_enemy(&mut self, enemy_id: u32, is_player_won: bool) {
        let node = self.enemy_nodes.get_mut(enemy_id as usize).unwrap();
        node.picks += 1;
        if !is_player_won {
            node.wins += 1;
        }
    }

    pub fn mark_player(&mut self, player_id: u32, is_player_won: bool) {
        let node = self.player_nodes.get_mut(player_id as usize).unwrap();
        node.picks += 1;
        if is_player_won {
            node.wins += 1;
        }
    }

    pub fn mark_state(&mut self, state_id: u32, is_player_won: bool) {
        let node = self.states.get_mut(state_id as usize).unwrap();
        node.picks += 1;

        if is_player_won {
            node.wins += 1;
        }
    }

    pub fn on_player_won(&mut self, node_id: u32, is_player_won: bool) {
        let mut current_node_id = node_id;
        loop {
            if current_node_id == self.current_state {
                break;
            }
            let (enemy_id, _) = State::get_node(current_node_id, self).get_parent(self);
            self.mark_enemy(enemy_id, is_player_won);
            let (player_id, _) = EnemyNode::get_node(enemy_id, self).get_parent(self);
            self.mark_player(player_id, is_player_won);
            let (next_state_id, _) = PlayerNode::get_node(player_id, self).get_parent(self);
            self.mark_state(next_state_id, is_player_won);
            current_node_id = next_state_id;
        }
    }
}

#[derive(Debug)]
pub struct State {
    game: Game,
    child_nodes: Vec<u32>,
    parent: Option<u32>,
    wins: u32,
    picks: u32,
}

pub trait HasChildren {
    type Child: GameNode;

    fn children(&self) -> std::slice::Iter<'_, u32>;
}

pub trait GameNode {
    type Parent: GameNode;

    fn wins(&self) -> u32;

    fn picks(&self) -> u32;

    fn get_node<'a>(node_id: u32, sim: &'a Simulation) -> &'a Self;

    fn get_node_mut<'a>(node_id: u32, sim: &'a mut Simulation) -> &'a mut Self;

    fn get_parent<'a>(&self, simulation: &'a Simulation) -> (u32, &'a Self::Parent);

    fn ucb(&self, simulation: &Simulation) -> f64 {
        if self.picks() == 0 {
            return f64::MAX;
        }

        let (_, parent) = self.get_parent(simulation);
        return (self.wins() as f64 / self.picks() as f64)
            + SQRT_2 * ((parent.picks() as f64).ln() / self.picks() as f64).sqrt();
    }
}

#[derive(Debug)]
pub struct PlayerNode {
    pub action: Action,
    pub parent_state: u32,
    pub enemy_moves: Vec<u32>,
    pub wins: u32,
    pub picks: u32,
}

impl HasChildren for PlayerNode {
    fn children(&self) -> std::slice::Iter<'_, u32> {
        return self.enemy_moves.iter();
    }

    type Child = EnemyNode;
}

impl PlayerNode {
    pub fn new() -> PlayerNode {
        todo!()
    }

    pub fn create(
        state_id: u32,
        sim: &mut Simulation,
        player_move: Action,
        enemy_moves: Vec<Action>,
    ) -> u32 {
        let this_id = sim.player_nodes.len() as u32;

        let result = PlayerNode {
            action: player_move,
            enemy_moves: enemy_moves
                .into_iter()
                .enumerate()
                .map(|(i, enemy_action)| {
                    sim.enemy_nodes
                        .push(EnemyNode::new(this_id, state_id, enemy_action));
                    return sim.enemy_nodes.len() as u32 - 1;
                })
                .collect_vec(),
            parent_state: state_id,
            picks: 0,
            wins: 0,
        };
        sim.player_nodes.push(result);

        return this_id;
    }

    fn get_state<'a>(&self, simulation: &'a Simulation) -> (u32, &'a State) {
        self.get_parent(simulation)
    }
}

impl GameNode for State {
    type Parent = EnemyNode;

    fn wins(&self) -> u32 {
        self.wins
    }

    fn picks(&self) -> u32 {
        self.picks
    }

    fn get_parent<'a>(&self, simulation: &'a Simulation) -> (u32, &'a Self::Parent) {
        let ref node = simulation.enemy_nodes[self.parent.unwrap() as usize];
        (self.parent.unwrap(), node)
    }

    fn get_node<'a>(node_id: u32, sim: &'a Simulation) -> &'a Self {
        let ref state = sim.states[node_id as usize];
        state
    }

    fn get_node_mut<'a>(node_id: u32, sim: &'a mut Simulation) -> &'a mut Self {
        sim.states.get_mut(node_id as usize).unwrap()
    }
}

impl HasChildren for State {
    fn children(&self) -> std::slice::Iter<'_, u32> {
        self.child_nodes.iter()
    }

    type Child = PlayerNode;
}

impl GameNode for PlayerNode {
    type Parent = State;

    fn wins(&self) -> u32 {
        self.wins
    }

    fn picks(&self) -> u32 {
        self.picks
    }

    fn get_node<'a>(node_id: u32, sim: &'a Simulation) -> &'a Self {
        let ref player = sim.player_nodes[node_id as usize];
        player
    }

    fn get_parent<'a>(&self, simulation: &'a Simulation) -> (u32, &'a Self::Parent) {
        let ref state = simulation.states[self.parent_state as usize];
        (self.parent_state, state)
    }

    fn get_node_mut<'a>(node_id: u32, sim: &'a mut Simulation) -> &'a mut Self {
        let node = sim.player_nodes.get_mut(node_id as usize).unwrap();
        node
    }
}

#[derive(Debug)]
pub struct EnemyNode {
    action: Action,
    parent_action: u32,
    wins: u32,
    picks: u32,
    parent_state: u32,
    next_state: Option<u32>,
}

impl EnemyNode {
    pub fn new(parent_id: u32, parent_state: u32, action: Action) -> Self {
        Self {
            parent_action: parent_id,
            parent_state,
            action,
            wins: 0,
            picks: 0,
            next_state: None,
        }
    }

    fn create_next_game<'a>(&self, sim: &'a Simulation) -> Game {
        let (_, parent_state) = self.get_state(sim);
        let (_, player_node) = self.get_parent(sim);

        let game = parent_state
            .game
            .apply_actions(sim.board, player_node.action, self.action);
        game
    }

    fn get_state<'a>(&self, simulation: &'a Simulation) -> (u32, &'a State) {
        (
            self.parent_state,
            &simulation.states[self.parent_state as usize],
        )
    }
}

impl GameNode for EnemyNode {
    type Parent = PlayerNode;

    fn wins(&self) -> u32 {
        self.wins
    }

    fn picks(&self) -> u32 {
        self.picks
    }

    fn get_parent<'a>(&self, simulation: &'a Simulation) -> (u32, &'a Self::Parent) {
        let ref action = simulation.player_nodes[self.parent_action as usize];
        (self.parent_action, action)
    }

    fn get_node<'a>(node_id: u32, sim: &'a Simulation) -> &'a Self {
        &sim.enemy_nodes[node_id as usize]
    }

    fn get_node_mut<'a>(node_id: u32, sim: &'a mut Simulation) -> &'a mut Self {
        let node = sim.enemy_nodes.get_mut(node_id as usize).unwrap();
        node
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cmp::Ordering,
        collections::HashMap,
        mem::size_of,
        time::{Duration, Instant},
    };

    use itertools::Itertools;

    use crate::{
        board::Board,
        game::Game,
        simulation::{GameNode, Simulation},
        tree::{Tree, TreeCollection},
    };

    #[test]
    pub fn test_new_simulation() {
        let board = Board::default_with_inactive(vec![28, 4, 3, 2, 6, 19].into_iter());
        let game = Game::parse_from_strings(vec![
            "0", "20", "2 0", "2 0 0", "4", "24 1 1 0", "27 1 1 0", "33 1 0 0", "36 1 0 0",
        ]);
        let sim = Simulation::new(&board, game);
        assert_eq!(sim.current_state, 0);
        assert_eq!(sim.player_nodes.len(), 8);
        assert_eq!(sim.enemy_nodes.len(), 8 * 8);
    }

    #[test]
    pub fn test_simulation() {
        let board = Board::default();
        let game = Game::parse_from_strings(vec![
            "0", "20", "2 0", "2 0 0", "4", "24 1 1 0", "27 1 1 0", "33 1 0 0", "36 1 0 0",
        ]);
        let mut sim = Simulation::new(&board, game);
        let d = Instant::now();
        for _ in 0..10 {
            sim.simulate(sim.current_state);
        }
        println!("{} ms", Duration::as_millis(&d.elapsed()));
        let moves = sim
            .get_moves_summary()
            .map(|x| {
                (
                    x.action,
                    x.ucb(&sim),
                    std::fmt::format(format_args!("{}/{}", x.wins, x.picks)),
                )
            })
            .collect_vec();
        /*
        let turn = sim.get_moves_summary().max_by(|x, y| {
            x.ucb(&sim)
                .partial_cmp(&y.ucb(&sim))
                .unwrap_or(Ordering::Less)
        });*/
        println!("{:?}", moves);
    }

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
        let game = Game::new(trees, 20, 10, 20, 0, 0, 10, false);
        for i in 0..limit {
            vec.push(game.clone());
        }
    }
}

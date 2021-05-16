use core::f64;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    f64::consts::SQRT_2,
    time::Instant,
    u32, usize,
};

use itertools::Itertools;
use rand::prelude::SliceRandom;

use crate::{
    actions::Action,
    board::{self, Board},
    common::random_max,
    game::{get_next_action_wood, Game},
};

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
        result.states[0].picks = 0;
        return result;
    }

    pub fn print_simulation(sim: &Simulation, state_id: u32, level: usize, max_level: usize) {
        fn indent(level: usize) -> String {
            let mut result = String::new();
            for _ in 0..level {
                result.push_str("  ");
            }
            result
        }

        if level > max_level {
            return;
        }

        let root = State::get_node(state_id, sim);

        eprintln!(
            "{}state: {}. score: {}. picks: {}, max_score: {}, avg_score: {}",
            indent(level),
            state_id,
            root.total_score(),
            root.picks(),
            root.max_score(),
            root.avg_score()
        );
        if level + 1 > max_level {
            return;
        }

        for x in &root.child_nodes {
            let player = PlayerNode::get_node(*x, sim);
            eprintln!(
                "{}player: {}. score: {}. picks: {}, max_score: {}, avg_score: {}",
                indent(level + 1),
                player.action,
                player.total_score(),
                player.picks(),
                player.max_score(),
                player.avg_score()
            );

            for y in player.children() {
                if level + 2 > max_level {
                    break;
                }
                let enemy = EnemyNode::get_node(*y, sim);
                eprintln!(
                    "{}enemy: {}. score: {}. picks: {}, max_score: {}, avg_score: {}",
                    indent(level + 2),
                    enemy.action,
                    enemy.total_score(),
                    enemy.picks(),
                    player.max_score(),
                    enemy.avg_score()
                );
                for s in enemy.next_state {
                    Self::print_simulation(sim, s, level + 3, max_level);
                }
            }
        }
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
        let value = State {
            child_nodes: Vec::new(),
            game: game,
            picks: 0,
            total_score: 0,
            parent: parent,
            max_score: i32::MIN,
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

    fn action_weight(&self, state_id: u32, action: &Action) -> u32 {
        let state = State::get_node(state_id, self);
        let day = state.game.day;
        let board = self.board;

        match *action {
            Action::COMPLETE(x) => 5,
            Action::GROW(x) if day < 20 => state.game.trees().get(x).size() as u32 + 1,
            Action::SEED(_, to) if day < 18 && board.get_richness(to) == 3 => 4,
            Action::WAIT if day <= 1 => 2,
            Action::SEED(_, _) if day > 18 => 0,
            Action::SEED(_, _) if day < 18 => 1,
            _ => 1,
        }
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
        let new_game = Self::create_next_game(self, &enemy_node);
        let (state_id, _) = self.create_state(new_game, Some(enemy_id));

        self.enemy_nodes[enemy_id as usize].next_state = Some(state_id);
        state_id
    }

    pub fn simulate_current(&mut self, cache: &mut HashMap<Game, u32>) {
        self.simulate(self.current_state, cache)
    }

    fn cache_state(&self, cache: &mut HashMap<Game, u32>, state_id: u32) {
        let game = &self.states[state_id as usize].game;
        if !cache.contains_key(game) {
            cache.insert(game.clone(), state_id);
        }
    }

    pub fn set_current(&mut self, state: u32) {
        self.current_state = state;
    }

    fn pick_best_of_n(&self, state_id: u32, x: Action, y: Action) -> Ordering {
        let state = State::get_node(state_id, self);
        let day = state.game.day;

        match (x, y) {
            (Action::SEED(_, 0), _) if day < 18 => Ordering::Greater,

            (_, Action::SEED(_, 0)) if day < 18 => self.pick_best_of_n(state_id, y, x).reverse(),
            _ => todo!(),
        }
    }

    fn ensure_player_best_nodes(&mut self, state_id: u32, width: usize) {
        let ref state = self.states[state_id as usize];

        if state.child_nodes.len() == 0 {
            let find_next_actions = Action::find_next_actions(&state.game, self.board, true);

            let player_moves = find_next_actions
                .into_iter()
                .map(|x| {
                    let next = state.game.apply_actions(self.board, x, Action::WAIT);
                    (x, self.action_weight(state_id, &x))
                })
                .sorted_by(|x, y| x.1.cmp(&y.1).reverse())
                .map(|x| x.0)
                .take(width)
                .collect_vec();

            for action in player_moves {
                self.player_nodes.push(PlayerNode::new(state_id, action));
                self.states[state_id as usize]
                    .child_nodes
                    .push(self.player_nodes.len() as u32 - 1);
            }
        }
    }

    fn ensure_enemy_only_waits(&mut self, state_id: u32, player_node_id: u32, width: usize) {
        let ref player_node = self.player_nodes[player_node_id as usize];
        if player_node.enemy_moves.len() == 0 {
            let find_next_actions = vec![Action::WAIT];
            for action in &find_next_actions {
                self.enemy_nodes.push(EnemyNode::new(
                    player_node_id,
                    player_node.parent_state,
                    *action,
                ));
            }
            for id in 1u32..find_next_actions.len() as u32 + 1 {
                self.player_nodes[player_node_id as usize]
                    .enemy_moves
                    .push(self.enemy_nodes.len() as u32 - id);
            }
        }
    }

    fn ensure_enemy_best_nodes(&mut self, state_id: u32, player_node_id: u32, width: usize) {
        let ref player_node = self.player_nodes[player_node_id as usize];
        let mut rng = rand::thread_rng();
        if player_node.enemy_moves.len() == 0 {
            let state = State::get_node(player_node.parent_state, self);

            let find_next_actions = Action::find_next_actions(&state.game, self.board, false);

            let enemy_moves = find_next_actions
                .into_iter()
                .map(|x| {
                    let next = state.game.apply_actions(self.board, Action::WAIT, x);
                    (x, self.action_weight(state_id, &x))
                })
                .sorted_by(|x, y| x.1.cmp(&y.1).reverse())
                .map(|x| x.0)
                .take(width)
                .collect_vec();

            for action in &enemy_moves {
                self.enemy_nodes.push(EnemyNode::new(
                    player_node_id,
                    player_node.parent_state,
                    *action,
                ));
            }
            for id in 1u32..enemy_moves.len() as u32 + 1 {
                self.player_nodes[player_node_id as usize]
                    .enemy_moves
                    .push(self.enemy_nodes.len() as u32 - id);
            }
        }
    }

    fn ensure_player_nodes(&mut self, state_id: u32, width: usize) {
        let mut rng = rand::thread_rng();
        let ref state = self.states[state_id as usize];
        if state.child_nodes.len() == 0 {
            let find_next_actions = Action::find_next_actions(&state.game, self.board, true);
            let player_moves = find_next_actions
                .choose_multiple_weighted(&mut rng, width, |a| self.action_weight(state_id, a))
                .unwrap();

            for action in player_moves {
                self.player_nodes.push(PlayerNode::new(state_id, *action));
                self.states[state_id as usize]
                    .child_nodes
                    .push(self.player_nodes.len() as u32 - 1);
            }
        }
    }

    fn ensure_enemy_nodes(&mut self, state_id: u32, player_node_id: u32, width: usize) {
        let ref player_node = self.player_nodes[player_node_id as usize];
        let mut rng = rand::thread_rng();
        if player_node.enemy_moves.len() == 0 {
            let state = State::get_node(player_node.parent_state, self);

            let find_next_actions = Action::find_next_actions(&state.game, self.board, false);

            let enemy_moves = find_next_actions
                .choose_multiple_weighted(&mut rng, width, |a| self.action_weight(state_id, a))
                .unwrap()
                .collect_vec();

            for action in &enemy_moves {
                self.enemy_nodes.push(EnemyNode::new(
                    player_node_id,
                    player_node.parent_state,
                    **action,
                ));
            }
            for id in 1u32..enemy_moves.len() as u32 + 1 {
                self.player_nodes[player_node_id as usize]
                    .enemy_moves
                    .push(self.enemy_nodes.len() as u32 - id);
            }
        }
    }

    fn get_points_for_tree(game: &Game, board: &Board, tree: u8, is_player: bool) -> i32 {
        let max_trees = vec![1, 2, 2, 4];
        todo!()
    }

    pub fn get_score(game: &Game, board: &Board, is_player: bool) -> Score {
        fn get_points_for_tree(game: &Game, size: u8, is_player: bool) -> i32 {
            let max_trees = vec![1, 2, 2, 4];
            let amount = game.trees().get_amount_of_size(size, is_player) as i32;
            let days_remaining = 24 - game.day;
            let can_complete = if 4 - size < days_remaining { 1 } else { 0 };
            let within_max = if amount <= max_trees[size as usize] as i32 {
                1
            } else {
                0
            };
            return can_complete * within_max;
        }

        let harvest_by_richness: i32 = game
            .trees()
            .iter_trees_for(is_player)
            .map(|x| {
                board.get_richness(x.index()) as i32
                    * get_points_for_tree(game, x.size(), is_player)
            })
            .sum();

        let nutrients = game.nutrients as i32;
        let t_total: i32 = (0..4)
            .map(|i| {
                get_points_for_tree(game, i, is_player)
                    * game.trees().get_amount_of_size(i, is_player) as i32
            })
            .sum();
        let player_income = game.average_sun_income(board, is_player) as i32;
        let enemy_income = game.average_sun_income(board, !is_player) as i32;

        let mut sun_budget =
            game.get_sun_points(is_player) as i32 + player_income * (23 - game.day as i32);

        let mut potential_harvest = 0;

        let mut n = nutrients;

        for s in 0..4 {
            let s = 3 - s;
            for _ in 0..game.trees().get_amount_of_size(s, is_player) {
                if sun_budget - game.get_harvest_cost_by_size(s, is_player) > 0 {
                    sun_budget -= game.get_harvest_cost_by_size(s, is_player);
                    potential_harvest += n * get_points_for_tree(game, s, is_player);
                    n -= 1 * get_points_for_tree(game, s, is_player);
                } else {
                    break;
                }
            }
        }

        let points = game.get_points(is_player) as i32;

        return Score {
            area_score: 0,
            points_score: 2 * points,
            richness_score: 2 * harvest_by_richness,
            sun_score: 2 * player_income - enemy_income,
            trees_score: potential_harvest,
            win_score: 0,
        };
    }

    pub fn get_score_2(game: &Game, board: &Board, is_player: bool) -> Score {
        fn get_mult_for_tree_score(size: u8, days_rem: u8) -> i32 {
            if 4 - size > days_rem {
                1
            } else {
                (4 - size - days_rem) as i32
            }
        }

        let nutrients = game.nutrients as i32;
        let day = game.day;
        let days_remaining = 24 - game.day;

        let my_trees_0 = game.trees().get_amount_of_size(0, is_player) as i32;
        let my_trees_1 = game.trees().get_amount_of_size(1, is_player) as i32;
        let my_trees_2 = game.trees().get_amount_of_size(2, is_player) as i32;
        let my_trees_3 = game.trees().get_amount_of_size(3, is_player) as i32;

        let player_income = game.average_sun_income(board, is_player) as i32;
        let enemy_income = game.average_sun_income(board, !is_player) as i32;

        let total_richness_by_trees: i32 = game
            .trees()
            .iter_trees_for(is_player)
            .filter(|t| t.time_to_complete() < days_remaining)
            .map(|x| board.get_richness(x.index()) as i32)
            .sum();

        let points = game.get_points(is_player) as i32;
        let score_for_area = 2 * player_income - enemy_income;
        let score_for_trees = my_trees_0 * get_mult_for_tree_score(0, days_remaining)
            + my_trees_1 * 2 * get_mult_for_tree_score(1, days_remaining)
            + my_trees_2 * get_mult_for_tree_score(2, days_remaining) * nutrients / 3
            + my_trees_3 * get_mult_for_tree_score(3, days_remaining) * nutrients / 2;

        let score_for_points = if day < 13 { 5 * points } else { 10 * points };

        let score_for_richness = total_richness_by_trees * 4;

        let win_score = match (game.day >= 23, game.is_player_won()) {
            (true, x) if x == is_player => 10000,
            (true, _) => -5000,
            (false, _) => 0,
        };

        return Score::new(
            score_for_area,
            score_for_points,
            0,
            score_for_trees,
            score_for_richness,
            0,
        );
    }

    fn update_score_node(&mut self, node_id: u32, is_player: bool) {
        if is_player {
            let mut player_node = &mut self.player_nodes[node_id as usize];
            player_node.picks += 1;
        }
        todo!()
    }

    fn propagate_score(&mut self, state_id: u32, player_score: i32, enemy_score: i32) {
        let mut current_node_id = state_id;
        loop {
            if current_node_id == self.current_state {
                break;
            }
            let state_node = &mut self.states[current_node_id as usize];
            state_node.picks += 1;
            state_node.total_score += player_score - enemy_score;
            state_node.max_score = std::cmp::max(state_node.max_score, player_score - enemy_score);

            let (enemy_id, _) = State::get_node(current_node_id, self).get_parent(self);
            let enemy_node = &mut self.enemy_nodes[enemy_id as usize];
            enemy_node.max_score = std::cmp::max(enemy_node.max_score, enemy_score - player_score);
            enemy_node.total_score += enemy_score - player_score;
            enemy_node.picks += 1;

            let (player_id, _) = EnemyNode::get_node(enemy_id, self).get_parent(self);
            let player_node = &mut self.player_nodes[player_id as usize];
            player_node.max_score =
                std::cmp::max(player_node.max_score, player_score - enemy_score);
            player_node.total_score += player_score - enemy_score;
            player_node.picks += 1;

            let (next_state_id, _) = PlayerNode::get_node(player_id, self).get_parent(self);
            current_node_id = next_state_id;
        }
    }

    pub fn simulate3(&mut self, state: u32, depth: u32, width: usize, time_budget: u32) {}

    pub fn simulate2(&mut self, state: u32, depth: u32, width: usize, iterations: u32) {
        //self.cache_state(cache, state);
        for _ in 0..iterations {
            let mut state_id = state;
            for d in 0..depth {
                self.ensure_player_best_nodes(state_id, width);
                let state = State::get_node(state_id, self);

                let (p_id, _) = Self::pick_node_by_ucb_2(&self, state);
                self.ensure_enemy_best_nodes(state_id, p_id, width);
                let (enemy_id, _) =
                    Self::pick_node_by_ucb_2(&self, &self.player_nodes[p_id as usize]);
                let next_state_id = self.get_or_create_next_state(enemy_id);
                let next_state = State::get_node(next_state_id, self);
                let ref next_game = next_state.game;
                //self.cache_state(cache, next_state_id);
                state_id = next_state_id;

                if next_game.day == 24 {
                    break;
                    //let player_won = next_game.is_player_won();
                    //self.on_player_won(next_state_id, player_won);
                    //break;
                }
            }

            let game = &self.states[state_id as usize].game;
            let player_score = Self::get_score(game, &self.board, true).value();
            let enemy_score = Self::get_score(game, &self.board, false).value();
            self.propagate_score(state_id, player_score, enemy_score);
        }
    }

    pub fn simulate(&mut self, state: u32, cache: &mut HashMap<Game, u32>) {
        //self.cache_state(cache, state);
        let mut state_id = state;
        loop {
            self.ensure_player_nodes(state_id, 100);
            let state = State::get_node(state_id, self);

            let (p_id, _) = Self::pick_node_by_ucb_2(&self, state);
            self.ensure_enemy_nodes(state_id, p_id, 100);
            let (enemy_id, _) = Self::pick_node_by_ucb_2(&self, &self.player_nodes[p_id as usize]);
            let next_state_id = self.get_or_create_next_state(enemy_id);
            let next_state = State::get_node(next_state_id, self);
            let ref next_game = next_state.game;
            //self.cache_state(cache, next_state_id);

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
            node.total_score += 1;
        }
    }

    pub fn mark_player(&mut self, player_id: u32, is_player_won: bool) {
        let node = self.player_nodes.get_mut(player_id as usize).unwrap();
        node.picks += 1;
        if is_player_won {
            node.total_score += 1;
        }
    }

    pub fn mark_state(&mut self, state_id: u32, is_player_won: bool) {
        let node = self.states.get_mut(state_id as usize).unwrap();
        node.picks += 1;

        if is_player_won {
            node.total_score += 1;
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
    max_score: i32,
    total_score: i32,
    picks: u32,
}
pub trait HasChildren {
    type Child: GameNode + HasAction;

    fn children(&self) -> std::slice::Iter<'_, u32>;
}

pub trait HasAction {
    fn action(&self) -> Action;
}

pub trait GameNode {
    type Parent: GameNode;

    fn total_score(&self) -> i32;

    fn picks(&self) -> u32;

    fn get_node<'a>(node_id: u32, sim: &'a Simulation) -> &'a Self;

    fn get_node_mut<'a>(node_id: u32, sim: &'a mut Simulation) -> &'a mut Self;

    fn get_parent<'a>(&self, simulation: &'a Simulation) -> (u32, &'a Self::Parent);

    fn avg_score(&self) -> f64 {
        self.total_score() as f64 / self.picks() as f64
    }

    fn max_score(&self) -> i32;

    fn exploration_weight(&self, simulation: &Simulation) -> f64;

    fn ucb(&self, simulation: &Simulation) -> f64 {
        if self.picks() == 0 {
            return 10000000.0 * self.exploration_weight(simulation);
        }

        let (_, parent) = self.get_parent(simulation);
        return self.avg_score()
            + 20.0 * SQRT_2 * ((parent.picks() as f64).ln() / self.picks() as f64).sqrt();
    }
}

#[derive(Debug)]
pub struct PlayerNode {
    pub action: Action,
    pub parent_state: u32,
    pub enemy_moves: Vec<u32>,
    pub total_score: i32,
    pub max_score: i32,
    pub picks: u32,
}

impl HasChildren for PlayerNode {
    fn children(&self) -> std::slice::Iter<'_, u32> {
        return self.enemy_moves.iter();
    }

    type Child = EnemyNode;
}

impl HasAction for PlayerNode {
    fn action(&self) -> Action {
        self.action
    }
}

impl PlayerNode {
    pub fn new(state_id: u32, action: Action) -> PlayerNode {
        Self {
            action: action,
            enemy_moves: Vec::new(),
            parent_state: state_id,
            picks: 0,
            total_score: 0,
            max_score: i32::MIN,
        }
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
            total_score: 0,
            max_score: i32::MIN,
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

    fn picks(&self) -> u32 {
        self.picks
    }

    fn get_node<'a>(node_id: u32, sim: &'a Simulation) -> &'a Self {
        let ref state = sim.states[node_id as usize];
        state
    }

    fn get_node_mut<'a>(node_id: u32, sim: &'a mut Simulation) -> &'a mut Self {
        sim.states.get_mut(node_id as usize).unwrap()
    }

    fn get_parent<'a>(&self, simulation: &'a Simulation) -> (u32, &'a Self::Parent) {
        let ref node = simulation.enemy_nodes[self.parent.unwrap() as usize];
        (self.parent.unwrap(), node)
    }

    fn total_score(&self) -> i32 {
        self.total_score
    }

    fn max_score(&self) -> i32 {
        self.max_score
    }

    fn exploration_weight(&self, simulation: &Simulation) -> f64 {
        1.0
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

    fn total_score(&self) -> i32 {
        self.total_score
    }

    fn picks(&self) -> u32 {
        self.picks
    }

    fn get_node<'a>(node_id: u32, sim: &'a Simulation) -> &'a Self {
        let ref player = sim.player_nodes[node_id as usize];
        player
    }

    fn get_node_mut<'a>(node_id: u32, sim: &'a mut Simulation) -> &'a mut Self {
        let node = sim.player_nodes.get_mut(node_id as usize).unwrap();
        node
    }

    fn get_parent<'a>(&self, simulation: &'a Simulation) -> (u32, &'a Self::Parent) {
        let ref state = simulation.states[self.parent_state as usize];
        (self.parent_state, state)
    }

    fn max_score(&self) -> i32 {
        self.max_score
    }

    fn exploration_weight(&self, simulation: &Simulation) -> f64 {
        simulation.action_weight(self.parent_state, &self.action) as f64
    }
}

#[derive(Debug)]
pub struct EnemyNode {
    action: Action,
    parent_action: u32,
    total_score: i32,
    max_score: i32,
    picks: u32,
    parent_state: u32,
    next_state: Option<u32>,
}

impl HasAction for EnemyNode {
    fn action(&self) -> Action {
        self.action
    }
}

impl EnemyNode {
    pub fn new(parent_id: u32, parent_state: u32, action: Action) -> Self {
        Self {
            parent_action: parent_id,
            parent_state,
            action,
            total_score: 0,
            picks: 0,
            next_state: None,
            max_score: i32::MIN,
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

pub trait HasScore {
    fn value(self) -> i32;
}

#[derive(Debug, Clone, Copy)]
pub struct Score {
    area_score: i32,
    trees_score: i32,
    sun_score: i32,
    points_score: i32,
    win_score: i32,
    richness_score: i32,
}

impl Score {
    pub fn value(self) -> i32 {
        self.into()
    }

    pub fn new(
        area_score: i32,
        points_score: i32,
        sun_score: i32,
        trees_score: i32,
        richness_score: i32,
        win_score: i32,
    ) -> Self {
        Score {
            area_score,
            points_score,
            sun_score,
            trees_score,
            win_score: win_score,
            richness_score,
        }
    }
}

impl HasScore for Score {
    fn value(self) -> i32 {
        self.value()
    }
}

impl Into<i32> for Score {
    fn into(self) -> i32 {
        return self.area_score as i32
            + self.trees_score as i32
            + self.points_score as i32
            + self.sun_score as i32
            + self.win_score as i32
            + self.richness_score as i32;
    }
}

impl GameNode for EnemyNode {
    type Parent = PlayerNode;

    fn total_score(&self) -> i32 {
        self.total_score
    }

    fn picks(&self) -> u32 {
        self.picks
    }

    fn get_node<'a>(node_id: u32, sim: &'a Simulation) -> &'a Self {
        &sim.enemy_nodes[node_id as usize]
    }

    fn get_node_mut<'a>(node_id: u32, sim: &'a mut Simulation) -> &'a mut Self {
        let node = sim.enemy_nodes.get_mut(node_id as usize).unwrap();
        node
    }

    fn get_parent<'a>(&self, simulation: &'a Simulation) -> (u32, &'a Self::Parent) {
        let ref action = simulation.player_nodes[self.parent_action as usize];
        (self.parent_action, action)
    }

    fn max_score(&self) -> i32 {
        self.max_score
    }

    fn exploration_weight(&self, simulation: &Simulation) -> f64 {
        simulation.action_weight(self.parent_state, &self.action) as f64
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cmp::Ordering,
        collections::HashMap,
        mem::size_of,
        ops::Add,
        time::{Duration, Instant},
        usize,
    };

    use itertools::Itertools;

    use crate::{
        actions::Action,
        board::Board,
        game::Game,
        simulation::{EnemyNode, GameNode, HasChildren, PlayerNode, Simulation, State},
        tree::{Tree, TreeCollection},
    };

    #[test]
    pub fn test_new_simulation() {
        let board = Board::default_with_inactive(vec![28, 4, 3, 2, 6, 19].into_iter());
        let game = Game::parse_from_strings(vec![
            "0", "20", "2 0", "2 0 0", "4", "24 1 1 0", "27 1 1 0", "33 1 0 0", "36 1 0 0",
        ]);
        let mut sim = Simulation::new(&board, game);
        let mut cache = HashMap::new();
        sim.simulate_current(&mut cache);
        assert_eq!(sim.current_state, 0);
    }

    pub fn test_simulation() {
        let board = Board::default();
        let game = Game::parse_from_strings(vec![
            "0", "20", "2 0", "2 0 0", "4", "24 1 1 0", "27 1 1 0", "33 1 0 0", "36 1 0 0",
        ]);
        let mut sim = Simulation::new(&board, game);
        let d = Instant::now();
        let mut cache = HashMap::new();

        for _ in 0..10 {
            sim.simulate_current(&mut cache);
        }
        println!("{} ms", Duration::as_millis(&d.elapsed()));
        let moves = sim
            .get_moves_summary()
            .map(|x| {
                (
                    x.action,
                    x.ucb(&sim),
                    std::fmt::format(format_args!("{}/{}", x.total_score, x.picks)),
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

    fn test_bad_simuation() {
        let board = Board::default_with_inactive(vec![28, 4, 3, 2, 6, 19].into_iter());
        let game = Game::parse_from_strings(vec![
            "0", "20", "2 0", "2 0 0", "4", "20 1 0 0", "24 1 0 0", "29 1 1 0", "33 1 1 0",
        ]);
        let mut sim = Simulation::new(&board, game);
        let mut cache = HashMap::new();
        for _ in 0..1000 {
            sim.simulate_current(&mut cache);
        }

        let moves = sim
            .get_moves_summary()
            .map(|x| {
                (
                    x.action,
                    x.ucb(&sim),
                    std::fmt::format(format_args!("{}/{}", x.total_score, x.picks)),
                )
            })
            .collect_vec();
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

    #[test]
    fn test_score_1() {
        let board = Board::default_with_inactive(vec![].into_iter());
        let game = Game::parse_from_strings(vec![
            "7", "20", "5 0", "11 0 0", "14", "1 3 0 0", "3 2 0 0", "5 0 0 0", "9 1 0 0",
            "13 1 1 0", "14 0 1 0", "17 0 1 0", "20 2 0 0", "24 2 0 0", "26 1 0 0", "28 1 1 0",
            "29 1 1 0", "32 0 1 0", "33 1 1 0",
        ]);

        let score_player = Simulation::get_score(&game, &board, true);
        let score_enemy = Simulation::get_score(&game, &board, false);

        println!(
            "player - {}, {} - enemy",
            score_player.value(),
            score_enemy.value()
        );
    }

    #[test]
    fn test_score_start_game() {
        let board = Board::default_with_inactive(vec![].into_iter());
        let game = Game::parse_from_strings(vec![
            "0", "20", "2 0", "2 0 0", "4", "20 1 0 0", "24 1 0 0", "29 1 1 0", "33 1 1 0",
        ]);

        let score_player = Simulation::get_score(&game, &board, true);
        let score_enemy = Simulation::get_score(&game, &board, false);

        print_score(&game, &board);
        assert_eq!(score_enemy.value(), score_player.value());
    }

    fn print_score(game: &Game, board: &Board) {
        let score_player = Simulation::get_score(&game, &board, true);
        let score_enemy = Simulation::get_score(&game, &board, false);
        println!(
            "player - {}, {} - enemy",
            Into::<i32>::into(score_player),
            Into::<i32>::into(score_enemy)
        );
        println!("{:?}", score_player);
        println!("{:?}", score_enemy);
    }

    fn print_simulation(sim: &Simulation, state_id: u32, level: usize, max_level: usize) {
        fn indent(level: usize) -> String {
            let mut result = String::new();
            for _ in 0..level {
                result.push_str("  ");
            }
            result
        }

        if level > max_level {
            return;
        }

        let root = State::get_node(state_id, sim);

        println!(
            "{}state. day: {}. max_score: {}. picks: {}, avg_score: {}, current_score: {}",
            indent(level),
            root.game.day,
            root.max_score(),
            root.picks(),
            root.avg_score(),
            Simulation::get_score(&root.game, &sim.board, true).value()
        );
        if level + 1 > max_level {
            return;
        }

        for x in &root.child_nodes {
            let player = PlayerNode::get_node(*x, sim);
            println!(
                "{}player: {}. max_score: {}. picks: {}, avg_score: {}",
                indent(level + 1),
                player.action,
                player.max_score(),
                player.picks(),
                player.avg_score()
            );

            for y in player.children() {
                if level + 2 > max_level {
                    break;
                }
                let enemy = EnemyNode::get_node(*y, sim);
                println!(
                    "{}enemy: {}. max_score: {}. picks: {}. avg_score: {}",
                    indent(level + 2),
                    enemy.action,
                    enemy.max_score(),
                    enemy.picks(),
                    enemy.avg_score()
                );
                for s in enemy.next_state {
                    print_simulation(sim, s, level + 3, max_level);
                }
            }
        }
    }

    #[test]
    fn test_score_play_game_2() {
        let board = Board::default_with_inactive(vec![].into_iter());
        let game = Game::parse_from_strings(vec![
            "0", "20", "2 0", "2 0 0", "4", "20 1 0 0", "24 1 0 0", "29 1 1 0", "33 1 1 0",
        ]);

        print_score(&game, &board);
        let game = game.apply_actions(&board, Action::WAIT, Action::WAIT);
        print_score(&game, &board);
        let game = game.apply_actions(&board, Action::SEED(33, 16), Action::GROW(20));
        print_score(&game, &board);
    }

    #[test]
    fn test_score_play_game() {
        let board = Board::default_with_inactive(vec![4, 31, 18].into_iter());
        let game = Game::parse_from_strings(vec![
            "0", "20", "2 0", "2 0 0", "4", "21 1 0 0", "24 1 0 0", "30 1 1 0", "33 1 1 0",
        ]);

        print_score(&game, &board);
        let game = game.apply_actions(&board, Action::SEED(33, 17), Action::WAIT);
        let game = game.apply_actions(&board, Action::SEED(30, 29), Action::WAIT);
        let game = game.apply_actions(&board, Action::WAIT, Action::WAIT);
        print_score(&game, &board);
    }

    #[test]
    fn test_win_loose() {
        let board = Board::default_with_inactive(vec![].into_iter());
        let game = Game::parse_from_strings(vec![
            "23", "6", "7 75", "10 142 0", "14", "0 0 1 0", "1 0 1 0", "3 3 0 0", "4 0 1 0",
            "6 1 1 0", "7 0 1 0", "8 1 1 0", "9 2 1 0", "14 0 1 0", "24 2 1 0", "27 2 1 0",
            "31 2 0 0", "33 1 0 0", "36 2 0 0",
        ]);

        let mut sim = Simulation::new(&board, game);
        sim.simulate2(0, 10, 10, 10);
        print_simulation(&sim, 0, 0, 1);
    }

    #[test]
    fn test_score_play_game_simulate() {
        let board = Board::default_with_inactive(vec![].into_iter());
        let game = Game::parse_from_strings(vec![
            "0", "20", "2 0", "2 0 0", "4", "22 1 0 0", "25 1 1 0", "31 1 1 0", "34 1 0 0",
        ]);

        let mut sim = Simulation::new(&board, game);
        sim.simulate2(0, 2, 100, 100);
        print_simulation(&sim, 0, 0, 25);
    }

    #[test]
    fn test_score_of_moves_ahead() {
        let board = Board::default_with_inactive(vec![].into_iter());
        let game = Game::parse_from_strings(vec![
            "7", "20", "8 0", "10 0 0", "12", "1 0 1 0", "2 0 0 0", "3 1 1 0", "4 3 0 0",
            "6 2 0 0", "11 2 1 0", "19 2 1 0", "25 3 1 0", "26 1 0 0", "28 2 0 0", "30 1 0 0",
            "34 2 0 0",
        ]);
        print_score(&game, &board);

        println!("{}", Simulation::get_score(&game, &board, true).value());
    }
}

use core::panic;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::{Debug, Display},
    u8,
};

use itertools::{Iterate, Itertools};

use crate::{
    actions::Action,
    board::{index_to_coord, Board},
    tree::TreeCollection,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shadow {
    index: u8,
    size: u8,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Game {
    //board: &'a board::Board,
    trees: TreeCollection,
    pub nutrients: u16,

    my_sun_points: u16,
    enemy_sun_points: u16,

    my_points: u16,
    enemy_points: u16,

    opponent_waiting: bool,

    pub day: u8,
}
/*
impl Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}*/

impl<'a> Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(day: {}, trees: {:?}, player_sp: {}, enemy_sun_points: {}, nutrients: {}, player_points: {}, enemy_points: {}, op_waiting: {})",
            self.day,
            self.trees,
            self.get_sun_points(true),
            self.get_sun_points(false),
            self.nutrients,
            self.my_points,
            self.enemy_points,
            self.opponent_waiting
        )
    }
}

impl Game {
    pub fn new(
        //board: &'a Board,
        trees: TreeCollection,
        nutrients: u16,
        my_sun_points: u16,
        enemy_sun_points: u16,
        my_score: u16,
        enemy_score: u16,
        day: u8,
        opponent_waiting: bool,
    ) -> Self {
        Self {
            //board,
            trees,
            nutrients,
            my_sun_points,
            enemy_sun_points,
            day,
            enemy_points: enemy_score,
            my_points: my_score,
            opponent_waiting: opponent_waiting,
        }
    }

    pub fn trees(&self) -> &TreeCollection {
        return &self.trees;
    }

    pub fn get_sun_points(&self, is_player: bool) -> u16 {
        if is_player {
            self.my_sun_points
        } else {
            self.enemy_sun_points
        }
    }

    pub fn empty() -> Self {
        Self {
            // board: board,
            day: 0,
            enemy_sun_points: 0,
            my_sun_points: 0,
            nutrients: 0,
            trees: TreeCollection::new(Vec::new()),
            enemy_points: 0,
            my_points: 0,
            opponent_waiting: false,
        }
    }

    fn find_shadows_by<'a, 'b>(
        &self,
        board: &'a Board,
        tree_index: u8,
    ) -> impl Iterator<Item = u8> + 'b
    where
        'a: 'b,
    {
        let tree = self.trees().get(tree_index);
        let sun_orientation = self.day % 6;
        board
            .get_line(index_to_coord(tree.index()), tree.size(), sun_orientation)
            .map(|x| x.index)
            .into_iter()
    }

    pub fn find_shadows(&self, board: &Board) -> HashMap<u8, Shadow> {
        self.trees
            .iter()
            .map(|t| {
                self.find_shadows_by(board, t.index()).map(move |s| {
                    (
                        s,
                        Shadow {
                            index: s,
                            size: t.size(),
                        },
                    )
                })
            })
            .flatten()
            .sorted_by_key(|x| x.1.size)
            .collect()
    }

    fn pay_action_cost(&mut self, board: &Board, action: Action, is_player: bool) {
        let cost = Action::get_action_cost(self, board, action, is_player);
        if is_player {
            self.my_sun_points -= cost as u16;
        } else {
            self.enemy_sun_points -= cost as u16;
        }
    }

    fn increase_points(&mut self, points: u16, is_player: bool) {
        match is_player {
            true => self.my_points += points,
            false => self.enemy_points += points,
        }
    }

    fn richness_to_points(richness: u8) -> u16 {
        match richness {
            1 => 0,
            2 => 2,
            3 => 4,
            _ => panic!(
                "richness of a tree can only be 1, 2 and 3. Got: {}",
                richness
            ),
        }
    }

    fn complete_tree(&mut self, board: &Board, tree_index: u8, is_player: bool) {
        let richness = board.get_richness(tree_index);
        self.trees.remove(tree_index);
        self.increase_points(
            self.nutrients + Self::richness_to_points(richness),
            is_player,
        );
    }

    pub fn apply_single_action(&self, board: &Board, action: Action, is_player: bool) -> Game {
        let mut new_state = self.clone();
        new_state.apply_action_on_clone(board, action, is_player);
        new_state
    }

    fn apply_action_on_clone(&mut self, board: &Board, action: Action, is_player: bool) {
        match (action, is_player) {
            (Action::WAIT, true) => {}
            (Action::WAIT, false) => self.opponent_waiting = true,
            (Action::COMPLETE(t), _) => {
                self.complete_tree(board, t, is_player);
                self.nutrients -= 1;
            }
            (Action::GROW(x), _) => {
                self.trees.grow_size(x);
            }
            (Action::SEED(from, to), is_player) => {
                self.trees.seed(to, is_player);
                self.trees.get_mut(from).set_dormant(true);
            }
        }
    }

    fn apply_seed_collision(&self, player_from: u8, enemy_from: u8) -> Game {
        let mut new_state = self.clone();
        new_state.trees.get_mut(player_from).set_dormant(true);
        new_state.trees.get_mut(enemy_from).set_dormant(true);
        return new_state;
    }

    fn force_wait_when_no_points(&self, board: &Board, action: Action, sun_points: u16) -> Action {
        if Action::get_action_cost(self, board, action, false) as u16 > sun_points {
            Action::WAIT
        } else {
            action
        }
    }

    pub fn apply_actions(&self, board: &Board, player: Action, enemy: Action) -> Game {
        let player = self.force_wait_when_no_points(board, player, self.my_sun_points);
        let enemy = self.force_wait_when_no_points(board, enemy, self.enemy_sun_points);

        match (player, enemy) {
            (Action::SEED(player_from, x), Action::SEED(enemy_from, y)) if x == y => {
                self.apply_seed_collision(player_from, enemy_from)
            }
            (Action::COMPLETE(x), Action::COMPLETE(y)) => {
                let mut new_state = self.clone();
                new_state.pay_action_cost(board, player, true);
                new_state.pay_action_cost(board, enemy, false);
                new_state.complete_tree(board, x, true);
                new_state.complete_tree(board, y, false);
                new_state.nutrients -= 1;
                new_state
            }
            (Action::WAIT, Action::WAIT) => self.apply_new_day(board),
            (player, enemy) => {
                let mut new_state = self.clone();
                new_state.pay_action_cost(board, player, true);
                new_state.pay_action_cost(board, enemy, false);
                new_state.apply_action_on_clone(board, player, true);
                new_state.apply_action_on_clone(board, enemy, false);
                return new_state;
            }
        }
    }

    fn collect_sun_points(&mut self, points: u8, is_player: bool) {
        if is_player {
            self.my_sun_points += points as u16;
        } else {
            self.enemy_sun_points += points as u16;
        }
    }

    fn apply_sun_points_for(&mut self, board: &Board, is_player: bool) {
        let all_shadows = self.find_shadows(board);
        let sun_trees: Vec<_> = self
            .trees
            .iter_trees_for(is_player)
            .filter(|t| match (&all_shadows.get(&t.index()), t.size()) {
                (Some(s), tree_size) if s.size >= tree_size => false,
                (_, _) => true,
            })
            .map(|x| (x.index(), x.size()))
            .collect();
        for (_, size) in sun_trees {
            self.collect_sun_points(size, is_player);
        }
    }

    pub fn apply_new_day(&self, board: &Board) -> Game {
        let mut new_state = self.clone();
        new_state.day += 1;
        new_state.apply_sun_points_for(board, true);
        new_state.apply_sun_points_for(board, false);
        new_state.opponent_waiting = false;
        new_state.trees.wake_up();
        new_state
    }
}

pub fn get_next_action_wood(game: &Game, board: &Board, actions: &Vec<Action>) -> Action {
    fn compare(game: &Game, board: &Board, x: &Action, y: &Action) -> Ordering {
        let can_wait = game.get_sun_points(true) < 3;
        let start_chopping = game.nutrients < 18 || game.day > 18;
        let state_next_day_left = game
            .apply_single_action(board, *x, true)
            .apply_new_day(board);
        let state_next_day_right = game
            .apply_single_action(board, *y, true)
            .apply_new_day(board);

        match (x, y) {
            (Action::COMPLETE(a), Action::COMPLETE(b)) => compare_by_richness(game, board, *a, *b),
            (Action::WAIT, Action::WAIT) => Ordering::Equal,
            (Action::WAIT, Action::GROW(_)) => Ordering::Less,
            (Action::WAIT, _) => {
                if can_wait {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (Action::COMPLETE(_), Action::GROW(_)) => greater_if_true(start_chopping),
            (Action::GROW(x), Action::GROW(y)) => greater_if_true(
                state_next_day_left.enemy_sun_points < state_next_day_right.enemy_sun_points,
            ),
            (Action::COMPLETE(_), Action::SEED(_, _)) => greater_if_true(start_chopping),
            (Action::SEED(_, a), Action::SEED(_, b)) => compare_by_richness(game, board, *a, *b),
            (Action::GROW(_), Action::SEED(_, to)) if board.get_richness(*to) == 3 => {
                Ordering::Less
            }
            (Action::GROW(_), Action::SEED(_, to)) => Ordering::Greater,

            (Action::GROW(_), Action::COMPLETE(_)) => compare(game, board, y, x).reverse(),
            (_, Action::WAIT) => compare(game, board, y, x).reverse(),
            (Action::SEED(_, _), Action::COMPLETE(_)) => compare(game, board, y, x).reverse(),
            (Action::SEED(_, _), Action::GROW(_)) => compare(game, board, y, x).reverse(),
        }
    }

    return actions
        .iter()
        .max_by(|x, y| compare(game, board, x, y))
        .cloned()
        .unwrap();
}

fn compare_by_richness(game: &Game, board: &Board, a: u8, b: u8) -> Ordering {
    board.get_richness(a).cmp(&board.get_richness(b))
}

fn greater_if_earlier(game: &Game, day: u8) -> Ordering {
    if game.day < day {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

fn less_if_earlier(game: &Game, day: u8) -> Ordering {
    if game.day < day {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn greater_if_true(value: bool) -> Ordering {
    if value {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse::Next, tree::Tree};

    use super::*;

    fn parse_from_strings(input: Vec<&str>) -> Game {
        let mut i = 0;
        let day: u8 = Next::read_from(&input, &mut i); // the game lasts 24 days: 0-23
        let nutrients: u16 = Next::read_from(&input, &mut i); // the base score you gain from the next COMPLETE action
        let values: Vec<u16> = Next::read_many_from(input[i]);
        i += 1;
        let sun_points = values[0]; // your sun points
        let score = values[1];
        let values: Vec<u16> = Next::read_many_from(input[i]);
        i += 1;
        let opp_sun = values[0]; // opponent's sun points
        let opp_score = values[1];
        let opp_is_waiting: u16 = values[2];

        let number_of_trees: i32 = Next::read_from(&input, &mut i); // the current amount of trees
        let mut trees = Vec::<Tree>::new();
        for _i in 0..number_of_trees as usize {
            trees.push(Next::read_from(&input, &mut i));
        }
        return Game::new(
            trees.into_iter().collect(),
            nutrients,
            sun_points,
            opp_sun,
            score,
            opp_score,
            day,
            opp_is_waiting == 1,
        );
    }

    #[test]
    fn test_hashmap() {
        let hash: HashMap<u8, u8> = vec![(1, 10), (1, 100)].into_iter().collect();
        assert_eq!(hash.get(&1), Some(&100u8));
    }
    #[test]
    fn get_next_action_wood_sorts_as_expected() {
        let x = get_next_action_wood(
            &Game::new(TreeCollection::empty(), 10, 10, 10, 0, 0, 1, false),
            &Board::default(),
            &vec![Action::WAIT, Action::COMPLETE(20), Action::COMPLETE(1)],
        );

        assert_eq!(Action::COMPLETE(1), x);
    }

    #[test]
    fn test_moves_ahead() {
        let board = Board::default();
        let game_str = vec![
            "10", "20", "3 0", "4 0 1", "15", "0 1 1 0", "1 1 1 0", "2 2 1 0", "3 2 1 0",
            "4 2 0 0", "5 2 0 0", "6 2 1 1", "10 0 1 0", "14 1 0 0", "17 1 0 0", "18 1 1 1",
            "21 3 1 0", "26 3 1 0", "30 1 0 0", "35 1 0 0",
        ];
        let game = parse_from_strings(game_str);

        let game = game.apply_actions(&board, Action::SEED(21, 11), Action::WAIT);
        let game = game.apply_actions(&board, Action::WAIT, Action::WAIT);
        let game = game.apply_actions(&board, Action::GROW(10), Action::GROW(14));
        let game = game.apply_actions(&board, Action::GROW(0), Action::GROW(17));
        let game = game.apply_actions(&board, Action::WAIT, Action::WAIT);
        let game = game.apply_actions(&board, Action::GROW(18), Action::GROW(17));
        let game = game.apply_actions(&board, Action::GROW(11), Action::GROW(4));
        let game = game.apply_actions(&board, Action::WAIT, Action::WAIT);

        let expected_state = parse_from_strings(vec![
            "13", "20", "9 0", "9 0 0", "16", "0 2 1 0", "1 1 1 0", "2 2 1 0", "3 2 1 0",
            "4 2 0 0", "5 2 0 0", "6 2 1 0", "10 1 1 0", "11 1 1 0", "14 2 0 0", "17 2 0 0",
            "18 2 1 0", "21 3 1 0", "26 3 1 0", "30 1 0 0", "35 1 0 0",
        ]);

        assert_eq!(game, expected_state);
    }
}

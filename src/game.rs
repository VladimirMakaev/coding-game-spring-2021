use core::panic;
use std::{cmp::Ordering, collections::HashMap, fmt::Display, u8};

use itertools::{Iterate, Itertools};

use crate::{
    actions::Action,
    board::{self, index_to_coord, Board},
    tree::{self, Tree, TreeCollection},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shadow {
    index: u8,
    size: u8,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Game {
    //board: &'a board::Board,
    trees: TreeCollection,
    pub nutrients: u16,

    my_sun_points: u16,
    enemy_sun_points: u16,

    my_points: u16,
    enemy_points: u16,

    player_waiting: bool,
    opponent_waiting: bool,

    pub day: u8,
}

impl<'a> Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(day: {}, trees: {}, player_sp: {}, enemy_sun_points: {}, nutrients: {})",
            self.day,
            self.trees.len(),
            self.get_sun_points(true),
            self.get_sun_points(false),
            self.nutrients
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
        day: u8,
    ) -> Self {
        Self {
            //board,
            trees,
            nutrients,
            my_sun_points,
            enemy_sun_points,
            day,
            enemy_points: 0,
            my_points: 0,
            opponent_waiting: false,
            player_waiting: false,
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
            player_waiting: false,
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
            self.enemy_points -= cost as u16;
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

    fn apply_action_on_clone(&mut self, board: &Board, action: Action, is_player: bool) {
        match (action, is_player) {
            (Action::WAIT, true) => self.player_waiting = true,
            (Action::WAIT, false) => self.opponent_waiting = true,
            (Action::COMPLETE(t), _) => {
                self.pay_action_cost(board, action, is_player);
                self.complete_tree(board, t, is_player);
                self.nutrients -= 1;
            }
            (Action::GROW(_), true) => {}
            (Action::GROW(_), false) => {}
            (Action::SEED(_, _), true) => {}
            (Action::SEED(_, _), false) => {}
        }
    }

    fn apply_seed_collision(&self, player_from: u8, enemy_from: u8) -> Game {
        let mut new_state = self.clone();
        new_state.trees.get_mut(player_from).set_dormant(true);
        new_state.trees.get_mut(enemy_from).set_dormant(true);
        return new_state;
    }

    pub fn apply_actions(&self, board: &Board, player: Action, enemy: Action) -> Game {
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
        let opponent_shadows = self.find_shadows(board);
        let sun_trees: Vec<_> = self
            .trees
            .iter_trees_for(is_player)
            .filter(|t| match (&opponent_shadows.get(&t.index()), t.size()) {
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
        new_state.apply_sun_points_for(board, true);
        new_state.apply_sun_points_for(board, false);
        new_state
    }
}

pub fn get_next_action_wood(game: &Game, board: &Board, actions: &Vec<Action>) -> Action {
    fn compare(game: &Game, board: &Board, x: &Action, y: &Action) -> Ordering {
        let can_wait = game.get_sun_points(true) < 3;
        let start_chopping = game.nutrients < 18 || game.day > 15;

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
            (_, Action::WAIT) => compare(game, board, y, x).reverse(),
            (Action::COMPLETE(_), Action::GROW(_)) => greater_if_true(start_chopping),
            (Action::GROW(_), Action::COMPLETE(_)) => compare(game, board, y, x).reverse(),
            (Action::GROW(x), Action::GROW(y)) => compare_by_richness(game, board, *x, *y),
            (Action::COMPLETE(_), Action::SEED(_, _)) => greater_if_true(start_chopping),
            (Action::SEED(_, a), Action::SEED(_, b)) => compare_by_richness(game, board, *a, *b),
            (Action::GROW(_), Action::SEED(_, to)) if board.get_richness(*to) == 3 => {
                Ordering::Less
            }
            (Action::GROW(_), Action::SEED(_, to)) => Ordering::Greater,
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
    use super::*;

    #[test]
    fn test_hashmap() {
        let hash: HashMap<u8, u8> = vec![(1, 10), (1, 100)].into_iter().collect();
        assert_eq!(hash.get(&1), Some(&100u8));
    }
    #[test]
    fn get_next_action_wood_sorts_as_expected() {
        let x = get_next_action_wood(
            &Game::new(TreeCollection::empty(), 10, 10, 10, 1),
            &Board::default(),
            &vec![Action::WAIT, Action::COMPLETE(20), Action::COMPLETE(1)],
        );

        assert_eq!(Action::COMPLETE(1), x);
    }

    #[test]
    fn test_game() {}
}

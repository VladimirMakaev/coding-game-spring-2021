use std::{cmp::Ordering, collections::HashMap, fmt::Display, u8};

use crate::{
    actions::Action,
    board::{self, Board},
    tree::TreeCollection,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shadow {
    index: u8,
    size: u8,
}

#[derive(Clone)]
pub struct Game {
    //board: &'a board::Board,
    trees: TreeCollection,
    pub nutrients: u16,
    my_sun_points: u16,
    enemy_sun_points: u16,
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
            trees: TreeCollection::new(HashMap::new()),
        }
    }

    pub fn find_shadows(&self) -> HashMap<u8, Shadow> {
        todo!()
    }

    pub fn apply_action(&self, action: Action) -> Game {
        todo!()
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
    fn get_next_action_wood_sorts_as_expected() {
        let x = get_next_action_wood(
            &Game::new(TreeCollection::empty(), 10, 10, 10, 1),
            &Board::default(),
            &vec![Action::WAIT, Action::COMPLETE(20), Action::COMPLETE(1)],
        );

        assert_eq!(Action::COMPLETE(1), x);
    }
}

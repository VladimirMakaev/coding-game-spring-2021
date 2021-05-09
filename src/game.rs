use std::{cmp::Ordering, fmt::Display, u8};

use crate::{
    actions::Action,
    board::{self, Board},
    tree::{Tree, TreeCollection},
};

pub struct Game<'a> {
    pub board: &'a board::Board,
    trees: TreeCollection,
    pub nutrients: u16,
    pub sun_points: u16,
    pub day: u8,
}

impl<'a> Display for Game<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(day: {}, trees: {}, player_sp: {}, nutrients: {})",
            self.day,
            self.trees.len(),
            self.sun_points,
            self.nutrients
        )
    }
}

impl<'a> Game<'a> {
    pub fn new(
        board: &'a Board,
        trees: TreeCollection,
        nutrients: u16,
        sun_points: u16,
        day: u8,
    ) -> Self {
        Self {
            board,
            trees,
            nutrients,
            sun_points,
            day,
        }
    }

    pub fn trees(&self) -> &TreeCollection {
        return &self.trees;
    }
}

pub fn get_next_action_wood(game: &Game, actions: &Vec<Action>) -> Action {
    let can_wait = game.sun_points < 7;
    let start_chopping = 10;

    return actions
        .iter()
        .max_by(|x, y| match (x, y) {
            (Action::COMPLETE(a), Action::COMPLETE(b)) => compare_by_richness(game, *a, *b),
            (Action::WAIT, Action::WAIT) => Ordering::Equal,
            (Action::WAIT, _) => {
                if can_wait {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            }
            (_, Action::WAIT) => {
                if can_wait {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }
            (Action::COMPLETE(_), Action::GROW(_)) => less_if_earlier(game, start_chopping),
            (Action::GROW(_), Action::COMPLETE(_)) => greater_if_earlier(game, start_chopping),
            (Action::GROW(x), Action::GROW(y)) => compare_by_richness(game, *x, *y),
            (Action::COMPLETE(_), Action::SEED(_, _)) => less_if_earlier(game, start_chopping),
            (Action::SEED(_, a), Action::SEED(_, b)) => compare_by_richness(game, *a, *b),
            (Action::GROW(_), Action::SEED(_, _)) => Ordering::Greater,
            (Action::SEED(_, _), Action::COMPLETE(_)) => greater_if_earlier(game, start_chopping),
            (Action::SEED(_, _), Action::GROW(_)) => Ordering::Less,
        })
        .cloned()
        .unwrap();
}

fn compare_by_richness(game: &Game, a: u8, b: u8) -> Ordering {
    game.board.get_richness(a).cmp(&game.board.get_richness(b))
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_next_action_wood_sorts_as_expected() {
        let x = get_next_action_wood(
            &Game::new(&Board::default(), TreeCollection::empty(), 10, 10, 1),
            &vec![Action::WAIT, Action::COMPLETE(20), Action::COMPLETE(1)],
        );

        assert_eq!(Action::COMPLETE(1), x);
    }
}

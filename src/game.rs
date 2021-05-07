use std::{cmp::Ordering, u8};

use crate::{
    actions::Action,
    board::{self, Board},
};

pub struct Game<'a> {
    board: &'a board::Board,
}

pub struct Player {}

impl<'a> Game<'a> {
    pub fn new(board: &'a Board) -> Self {
        Self { board }
    }
}

pub fn get_next_action_wood(game: &Game, actions: &Vec<Action>) -> Action {
    return actions
        .iter()
        .max_by(|x, y| match (x, y) {
            (Action::WAIT, Action::WAIT) => Ordering::Equal,
            (Action::WAIT, Action::COMPLETE(_)) => Ordering::Less,
            (Action::COMPLETE(_), Action::WAIT) => Ordering::Greater,
            (Action::COMPLETE(a), Action::COMPLETE(b)) => game
                .board
                .get_richness(*a)
                .cmp(&game.board.get_richness(*b)),
        })
        .cloned()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_next_action_wood_sorts_as_expected() {
        let x = get_next_action_wood(
            &Game::new(&Board::default()),
            &vec![Action::WAIT, Action::COMPLETE(20), Action::COMPLETE(1)],
        );

        assert_eq!(Action::COMPLETE(1), x);
    }
}

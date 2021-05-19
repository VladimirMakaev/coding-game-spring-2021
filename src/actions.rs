use crate::{
    board::{Board, Cell},
    common::ParseError,
    game::Game,
    tree::Tree,
};
use core::panic;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Action {
    WAIT,
    COMPLETE(u8),
    GROW(u8),
    SEED(u8, u8),
}

impl Action {
    pub fn find_next_actions(game: &Game, board: &Board, is_player: bool) -> Vec<Action> {
        vec![
            vec![Action::WAIT],
            Self::find_next_complete_actions(game, board, is_player),
            Self::find_next_grow_actions(game, board, is_player),
            Self::find_next_seed_actions(game, board, is_player),
        ]
        .into_iter()
        .flatten()
        .filter(|a| {
            (Self::get_action_cost(game, *a, is_player) as u16) <= game.get_sun_points(is_player)
        })
        .collect()
    }

    pub fn find_next_complete_actions(game: &Game, _board: &Board, is_player: bool) -> Vec<Action> {
        game.trees()
            .iter_trees_for(is_player)
            .filter(|t| t.size() == 3 && !t.is_dormant())
            .map(|t| Action::COMPLETE(t.index()))
            .collect()
    }

    pub fn find_next_grow_actions(game: &Game, _board: &Board, is_player: bool) -> Vec<Action> {
        game.trees()
            .iter_trees_for(is_player)
            .filter(|t| t.size() < 3 && !t.is_dormant())
            .map(|t| Action::GROW(t.index()))
            .collect()
    }

    pub fn get_grow_cost(game: &Game, size: u8, is_player: bool) -> u8 {
        match size {
            0 => 1 + game.trees().get_amount_of_size(1, is_player),
            1 => 3 + game.trees().get_amount_of_size(2, is_player),
            2 => 7 + game.trees().get_amount_of_size(3, is_player),
            _ => panic!("Can't grow a tree of size {}", size),
        }
    }

    pub fn get_action_cost(game: &Game, action: Action, is_player: bool) -> u8 {
        match action {
            Self::WAIT => 0,
            Self::SEED(_, _) => game.trees().get_amount_of_size(0, is_player),
            Self::GROW(tree_index) => {
                let tree = game.trees().get(tree_index);
                Self::get_grow_cost(game, tree.size(), is_player)
            }
            Action::COMPLETE(_) => 4,
        }
    }

    fn get_seedable_neighbors<'a>(
        game: &'a Game,
        board: &'a Board,
        tree: &Tree,
    ) -> impl Iterator<Item = &'a Cell> {
        board
            .get_neighbors_from(tree.index(), tree.size())
            .filter(move |cell| cell.richness > 0 && !game.trees().has_at(cell.index))
    }

    pub fn find_next_seed_actions(game: &Game, board: &Board, is_player: bool) -> Vec<Action> {
        let trees = game.trees();
        return trees
            .iter_trees_for(is_player)
            .filter(|x| x.not_dormant() && x.size() > 0)
            .map(|tree| {
                (
                    tree.index(),
                    tree.size(),
                    Self::get_seedable_neighbors(game, board, tree),
                )
            })
            .flat_map(|(from, _, to_list)| to_list.map(move |to| Action::SEED(from, to.index)))
            .collect();
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::WAIT => write!(f, "WAIT"),
            Action::COMPLETE(x) => write!(f, "COMPLETE {}", x),
            Action::GROW(x) => write!(f, "GROW {}", x),
            Action::SEED(x, y) => write!(f, "SEED {} {}", x, y),
        }
    }
}

impl FromStr for Action {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let params: Vec<&str> = s.split(' ').collect();
        match (params[0], params.len()) {
            ("WAIT", 1) => Ok(Action::WAIT),
            ("WAIT", _) => Err(ParseError::InvalidParameters),
            ("COMPLETE", 2) => Ok(Action::COMPLETE(
                params[1]
                    .parse::<u8>()
                    .map_err(|_| ParseError::InvalidParameters)?,
            )),
            ("GROW", 2) => Ok(Action::GROW(
                params[1]
                    .parse::<u8>()
                    .map_err(|_| ParseError::InvalidParameters)?,
            )),
            ("SEED", 3) => Ok(Action::SEED(
                params[1]
                    .parse::<u8>()
                    .map_err(|_| ParseError::InvalidParameters)?,
                params[2]
                    .parse::<u8>()
                    .map_err(|_| ParseError::InvalidParameters)?,
            )),
            _ => Err(ParseError::UnknownInput),
        }
    }
}

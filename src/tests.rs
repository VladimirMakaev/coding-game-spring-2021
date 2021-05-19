pub fn main() {}
mod actions;
mod board;
pub mod common;
mod game;
pub mod parse;
pub mod simulation;
mod tree;

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{board::Board, tree::TreeCollection};

    use super::*;

    use actions::*;
    use game::*;

    #[test]
    fn action_parses() {
        let result = "COMPLETE 12".parse::<Action>();
        assert_eq!(Ok(Action::COMPLETE(12)), result);
        let result = "WAIT".parse::<Action>();
        assert_eq!(Ok(Action::WAIT), result);
        let result = "GROW 32".parse::<Action>();
        assert_eq!(Ok(Action::GROW(32)), result);
        let result = "SEED 32 1".parse::<Action>();
        assert_eq!(Ok(Action::SEED(32, 1)), result);
    }

    #[test]
    fn available_actions_seed_start_game() {
        let trees =
            TreeCollection::from_strings(vec!["21 1 1 0", "27 1 0 0", "30 1 0 0", "36 1 1 0"]);
        let board = Board::default();
        let game = Game::new(trees, 20, 2, 10, 0, 0, 0, false);
        let expected_actions: Vec<Action> = vec![
            "WAIT",
            "SEED 36 7",
            "SEED 21 8",
            "SEED 21 9",
            "SEED 36 19",
            "SEED 36 35",
            "SEED 21 22",
            "SEED 21 20",
            "SEED 36 18",
        ]
        .into_iter()
        .flat_map(|x| x.parse::<Action>())
        .sorted()
        .collect();

        let mut actions = Action::find_next_actions(&game, &board, true);
        actions.sort();

        assert_eq!(actions, expected_actions);
    }

    #[test]
    fn available_actions_seed_middle_game() {
        let trees = TreeCollection::from_strings(vec![
            "5 2 0 0", "6 1 0 0", "13 0 0 0", "15 2 0 0", "21 3 1 0", "27 1 0 0", "30 1 0 0",
            "36 2 1 0",
        ]);
        let board = Board::default_with_inactive(vec![25, 23, 32, 34].into_iter());
        let game = Game::new(trees, 20, 11, 10, 0, 0, 7, false);
        let expected_actions: Vec<Action> = vec![
            "WAIT",
            "COMPLETE 21",
            "GROW 36",
            "SEED 21 11",
            "SEED 36 18",
            "SEED 21 22",
            "SEED 36 8",
            "SEED 36 17",
            "SEED 21 3",
            "SEED 21 1",
            "SEED 21 2",
            "SEED 21 9",
            "SEED 21 0",
            "SEED 21 8",
            "SEED 21 18",
            "SEED 21 20",
            "SEED 36 35",
            "SEED 36 7",
            "SEED 36 20",
            "SEED 21 10",
            "SEED 21 7",
            "SEED 36 1",
            "SEED 21 24",
            "SEED 21 19",
            "SEED 36 19",
        ]
        .into_iter()
        .flat_map(|x| x.parse::<Action>())
        .sorted()
        .collect();

        let mut actions = Action::find_next_actions(&game, &board, true);
        actions.sort();

        assert_eq!(actions, expected_actions);
    }

    #[test]
    fn available_actions_seed_middle_game_2() {
        let trees = TreeCollection::from_strings(vec![
            "0 2 0 0", "1 1 1 1", "2 0 1 1", "3 0 0 1", "4 1 0 1", "5 1 0 0", "6 2 0 1",
            "12 0 1 1", "15 2 0 0", "16 0 0 0", "17 1 0 0", "22 3 1 1", "25 3 1 1", "30 0 0 0",
            "31 1 0 0", "34 1 0 0",
        ]);
        let board = Board::default_with_inactive(vec![25, 23, 32, 34].into_iter());
        let game = Game::new(trees, 20, 10, 10, 0, 0, 9, false);
        let expected_actions: Vec<Action> = vec!["WAIT"]
            .into_iter()
            .flat_map(|x| x.parse::<Action>())
            .sorted()
            .collect();

        let mut actions = Action::find_next_actions(&game, &board, true);
        actions.sort();

        assert_eq!(actions, expected_actions);
    }

    #[test]
    fn available_actions_real_test() {
        let trees = TreeCollection::from_strings(vec![
            "0 0 1 0", "1 0 0 0", "2 0 1 0", "3 2 1 0", "4 1 0 0", "5 2 1 0", "6 2 0 0", "7 2 0 0",
            "9 1 0 0", "18 0 0 0", "22 1 0 0", "36 1 0 0",
        ]);
        let board = Board::default_with_inactive(vec![26, 10, 21, 30, 16, 35].into_iter());
        let game = Game::new(trees, 17, 7, 2, 0, 0, 11, false);
        let expected_actions: Vec<Action> = vec![
            "WAIT",
            "GROW 5",
            "GROW 0",
            "GROW 2",
            "GROW 3",
            "SEED 3 25",
            "SEED 5 14",
            "SEED 3 12",
            "SEED 3 8",
            "SEED 5 33",
            "SEED 3 24",
            "SEED 5 13",
            "SEED 5 12",
            "SEED 3 11",
            "SEED 5 17",
            "SEED 3 27",
            "SEED 5 31",
            "SEED 3 14",
            "SEED 3 13",
            "SEED 5 29",
            "SEED 5 15",
            "SEED 5 32",
            "SEED 3 23",
        ]
        .into_iter()
        .flat_map(|x| x.parse::<Action>())
        .sorted()
        .collect();

        let actions = Action::find_next_actions(&game, &board, true)
            .into_iter()
            .sorted()
            .collect_vec();

        assert_eq!(actions, expected_actions);
    }

    #[test]
    fn available_actions_real_test_2() {
        let trees = TreeCollection::from_strings(vec![
            "0 1 0 1", "1 1 0 0", "2 1 0 0", "3 2 0 0", "4 1 1 0", "5 3 1 1", "7 0 0 0", "8 2 0 0",
            "14 0 1 0", "16 0 1 0", "18 1 0 0", "20 1 0 0", "35 1 0 0",
        ]);
        let board = Board::default_with_inactive(vec![25, 11, 27, 26, 17, 34].into_iter());
        let game = Game::new(trees, 17, 2, 2, 0, 0, 11, false);
        let expected_actions = vec!["WAIT", "GROW 16", "GROW 14", "SEED 4 13", "SEED 4 12"]
            .into_iter()
            .flat_map(|x| x.parse::<Action>())
            .sorted()
            .collect_vec();

        let actions = Action::find_next_actions(&game, &board, true)
            .into_iter()
            .sorted()
            .collect_vec();

        assert_eq!(actions, expected_actions);
    }

    #[test]
    fn available_actions_real_test_2_enemy_actions() {
        let trees = TreeCollection::from_strings(vec![
            "0 1 0 1", "1 1 0 0", "2 1 0 0", "3 2 0 0", "7 0 0 0", "8 2 0 0", "18 1 0 0",
            "20 1 0 0", "35 1 0 0", "5 3 1 1", "14 0 1 0", "16 0 1 0", "4 1 1 0",
        ]);
        let board = Board::default_with_inactive(vec![25, 11, 27, 26, 17, 34].into_iter());
        let game = Game::new(trees, 17, 2, 6, 0, 0, 11, false);
        let expected_grow = vec![
            "GROW 1", "GROW 2", "GROW 3", "GROW 7", "GROW 8", "GROW 18", "GROW 20", "GROW 35",
        ]
        .into_iter()
        .flat_map(|x| x.parse::<Action>())
        .sorted()
        .collect_vec();

        let _trees = game.trees().iter_trees_for(false).collect_vec();

        let actions = Action::find_next_grow_actions(&game, &board, false)
            .into_iter()
            .sorted()
            .collect_vec();

        assert_eq!(actions, expected_grow);
    }
}

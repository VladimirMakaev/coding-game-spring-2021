pub mod engine;
use engine::{game::search_next_action, game_loop::*};
use std::env;

pub struct BeamSearch {}

impl BeamSearch {
    pub fn new() -> BeamSearch {
        BeamSearch {}
    }
}

impl Strategy for BeamSearch {
    fn on_start(&mut self, board: &engine::board::Board) {}

    fn get_next(
        &mut self,
        game: &engine::game::Game,
        board: &engine::board::Board,
        time_limit: u128,
    ) -> engine::actions::Action {
        let (d, action) = search_next_action(game, board, 5, 100, time_limit);
        eprintln!("depth is {}", d);
        action
    }
}

fn main() {
    //let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);
    play_game(BeamSearch::new(), GameSettings::new());
}

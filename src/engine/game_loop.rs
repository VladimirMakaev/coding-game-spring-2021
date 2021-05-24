use std::time::Instant;

use crate::engine::{game::search_next_action, tree::Tree};

use super::{
    actions::Action,
    board::{Board, Cell},
    game::Game,
    parse::Next,
};

pub trait Strategy {
    fn on_start(&mut self, board: &Board);

    fn get_next(&mut self, game: &Game, board: &Board, time_left: u128) -> Action;
}

pub struct GameLoop {
    game: Game,
    board: Board,
    turn: usize,
    time_limit: usize,
}

pub struct GameSettings {}

impl GameSettings {
    pub fn new() -> GameSettings {
        GameSettings {}
    }
}

pub fn play_game<TStrategy>(mut strat: TStrategy, settings: GameSettings)
where
    TStrategy: Strategy,
{
    let number_of_cells: i32 = Next::read(); // 37
    let mut cells = Vec::new();
    for _i in 0..number_of_cells as usize {
        let cell: Cell = Next::read();
        cells.push(cell);
    }

    let board: Board = cells.into_iter().collect();
    strat.on_start(&board);
    let mut time_limit = 1000;
    // game loop
    loop {
        let day: u8 = Next::read(); // the game lasts 24 days: 0-23
        let nutrients: u16 = Next::read(); // the base score you gain from the next COMPLETE action
        let inputs: Vec<u16> = Next::read_many();
        let sun_points = inputs[0]; // your sun points
        let score = inputs[1]; // your current score
        let inputs: Vec<u16> = Next::read_many();
        let opp_sun = inputs[0]; // opponent's sun points
        let opp_score = inputs[1]; // opponent's score
        let opp_is_waiting = inputs[2]; // whether your opponent is asleep until the next day
        let number_of_trees: i32 = Next::read(); // the current amount of trees
        let mut trees = Vec::<Tree>::new();

        for _i in 0..number_of_trees as usize {
            trees.push(Next::read());
        }

        let number_of_possible_moves: i32 = Next::read(); //test

        let mut actions = Vec::<Action>::new();
        for _i in 0..number_of_possible_moves as usize {
            let possible_move: Action = Next::read();
            //eprintln!("{:?}", possible_move);
            actions.push(possible_move);
        }

        let game = Game::new(
            trees.into_iter().collect(),
            nutrients,
            sun_points,
            opp_sun,
            score,
            opp_score,
            day,
            opp_is_waiting == 1,
        );
        let start = Instant::now();
        let action = strat.get_next(&game, &board, time_limit);
        let finish = Instant::now();
        eprintln!("elapsed: {} ms", finish.duration_since(start).as_millis(),);
        // Simulation::print_simulation(&sim, 0, 0, 1);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>
        /*let player_move = sim
        .get_moves_summary()
        .max_by(|x, y| {
            x.avg_score()
                .partial_cmp(&y.avg_score())
                .unwrap_or(Ordering::Less)
        })
        .unwrap();*/

        println!("{}", action);
        time_limit = 100;
    }
}

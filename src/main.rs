pub mod actions;
pub mod board;
pub mod common;
pub mod game;
pub mod parse;
pub mod simulation;
pub mod tree;
use std::time::Instant;

use actions::*;
use board::{Board, Cell};
use parse::*;

use crate::{game::Game, tree::Tree};

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let number_of_cells: i32 = Next::read(); // 37
    let mut cells = Vec::new();
    for _i in 0..number_of_cells as usize {
        let cell: Cell = Next::read();
        cells.push(cell);
    }

    let board: Board = cells.into_iter().collect();

    /*
        let new = Instant::now();
        let limit = 5_000_000;
        let mut states = Vec::with_capacity(limit);
        for i in 0..limit {
            states.push(Game::empty());
        }
        eprintln!("Init state in {}ms ", new.elapsed().as_millis());
    */
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
        /*
        let mut sim = Simulation::new(&board, game);
        let mut c = 0;
        loop {
            sim.simulate2(0, 5, 20, 1);
            c += 1;
            let finish = Instant::now();

            if finish.duration_since(start).as_millis() > 90 {
                break;
            }
        }*/
        let start = Instant::now();
        let (d, action) = game::search_next_action(&game, &board, 5, 100);
        let finish = Instant::now();

        eprintln!(
            "elapsed: {} ms. {} depth",
            finish.duration_since(start).as_millis(),
            d
        );
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
    }
}

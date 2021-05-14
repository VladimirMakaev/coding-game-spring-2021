mod actions;
mod board;
pub mod common;
mod game;
pub mod parse;
mod simulation;
mod tree;
use core::time;
use std::{
    cmp::Ordering,
    collections::HashMap,
    time::{Duration, Instant},
};

use actions::*;
use board::{Board, Cell};
use itertools::{assert_equal, Itertools};
use parse::*;
use simulation::*;

use crate::{game::Game, tree::Tree};

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let number_of_cells: i32 = Next::read(); // 37
    let mut cells = Vec::new();
    for i in 0..number_of_cells as usize {
        let cell: Cell = Next::read();
        cells.push(cell);
    }

    let mut time_allocation = 950;
    let board: Board = cells.into_iter().collect();
    let mut cache = HashMap::new();
    let mut sim = Simulation::new(&board, Game::empty());

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
        let timer = Instant::now();
        if let Some(state_id) = cache.get(&game) {
            sim.set_current(*state_id);
        } else {
            sim = Simulation::new(&board, game);
        }
        let mut counter = 0;
        while Duration::as_millis(&timer.elapsed()) < time_allocation {
            sim.simulate_current(&mut cache);
            counter += 1;
        }

        eprintln!("{} simulations", counter);

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>
        let player_move = sim
            .get_moves_summary()
            .max_by(|x, y| {
                x.mean_win()
                    .partial_cmp(&y.mean_win())
                    .unwrap_or(Ordering::Less)
            })
            .unwrap();

        println!("{}", player_move.action);

        time_allocation = 80;
    }
}

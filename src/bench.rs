use std::{
    mem::size_of_val,
    time::{Duration, Instant},
};

use itertools::Itertools;

use crate::{board::Board, game::Game, simulation::Simulation, simulation::*};

mod actions;
mod board;
pub mod common;
mod game;
pub mod parse;
mod simulation;
mod tree;

pub fn main() {
    let board = Board::default();
    let game = Game::parse_from_strings(vec![
        "0", "20", "2 0", "2 0 0", "4", "24 1 1 0", "27 1 1 0", "33 1 0 0", "36 1 0 0",
    ]);
    let mut sim = Simulation::new(&board, game);
    let d = Instant::now();
    for _ in 0..10000 {
        sim.simulate_current();
    }

    println!("{} ms.", Duration::as_millis(&d.elapsed()),);
    let moves = sim
        .get_moves_summary()
        .map(|x| {
            (
                x.action,
                x.ucb(&sim),
                std::fmt::format(format_args!("{}/{}", x.wins, x.picks)),
            )
        })
        .collect_vec();
    println!("{:?}", moves);
}

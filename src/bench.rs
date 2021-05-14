use std::{
    collections::HashMap,
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
    let board = Board::default_with_inactive(vec![26, 30, 14, 0, 8, 21, 35].into_iter());
    let game = Game::parse_from_strings(vec![
        "0", "20", "2 0", "2 0 0", "4", "19 1 0 0", "24 1 0 0", "28 1 1 0", "33 1 1 0",
    ]);
    let mut sim = Simulation::new(&board, game);
    let d = Instant::now();
    let mut cache = HashMap::new();

    let for_lookup = Game::parse_from_strings(vec![
        "1", "20", "4 0", "4 0 0", "4", "19 1 0 0", "24 1 0 0", "28 1 1 0", "33 1 1 0",
    ]);
    for _ in 0..1000 {
        sim.simulate_current(&mut cache);
    }

    println!("{} ms.", Duration::as_millis(&d.elapsed()),);

    if cache.contains_key(&for_lookup) {
        println!("Found in cache. Cache Size: {}", cache.len());
        sim.set_current(*cache.get(&for_lookup).unwrap());
    }

    let moves = sim
        .get_moves_summary()
        .map(|x| {
            (
                x.action,
                x.ucb(&sim),
                std::fmt::format(format_args!(
                    "{}/{} = {}",
                    x.wins,
                    x.picks,
                    x.wins as f64 / x.picks as f64
                )),
            )
        })
        .collect_vec();
    println!("{:?}", moves);
}

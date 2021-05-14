#![feature(test)]

use std::{
    collections::HashMap,
    mem::size_of_val,
    time::{Duration, Instant},
};

use itertools::Itertools;
use rand::{prelude::SliceRandom, Rng};

use crate::{actions::Action, board::Board, game::Game, simulation::Simulation, simulation::*};

mod actions;
mod board;
pub mod common;
mod game;
pub mod parse;
mod simulation;
mod tree;

pub fn main() {}

pub fn play(board: &Board, game: Game, day: u8) -> Game {
    let mut r1 = rand::thread_rng();
    let mut game = game;
    loop {
        if game.day == day {
            return game;
        }
        let p_actions = Action::find_next_actions(&game, &board, true);
        let e_actions = Action::find_next_actions(&game, &board, false);
        let act_1 = p_actions.choose(&mut r1).unwrap();
        let act_2 = e_actions.choose(&mut r1).unwrap();
        game = game.apply_actions(&board, *act_1, *act_2);
    }
}

pub fn main5() {
    let board = Board::default_with_inactive(vec![26, 30, 14, 0, 8, 21, 35].into_iter());
    let game = Game::parse_from_strings(vec![
        "0", "20", "2 0", "2 0 0", "4", "19 1 0 0", "24 1 0 0", "28 1 1 0", "33 1 1 0",
    ]);
    let game = play(&board, game, 24);
    let p_actions = Action::find_next_actions(&game, &board, true);
    let e_actions = Action::find_next_actions(&game, &board, false);
    let mut r1 = rand::thread_rng();

    let act_1 = p_actions.choose(&mut r1).unwrap();
    let act_2 = e_actions.choose(&mut r1).unwrap();
    let d = Instant::now();
    for _ in 0..100000 {
        let p_actions = Action::find_next_actions(&game, &board, true);
        let e_actions = Action::find_next_actions(&game, &board, false);

        let act_1 = p_actions.choose(&mut r1).unwrap();
        let act_2 = e_actions.choose(&mut r1).unwrap();
        //let _ = game.apply_actions(&board, *act_1, *act_2);
    }

    println!("{} ms.", Duration::as_millis(&d.elapsed()));
}

pub fn main2() {
    let board = Board::default_with_inactive(vec![26, 30, 14, 0, 8, 21, 35].into_iter());
    let game = Game::parse_from_strings(vec![
        "0", "20", "2 0", "2 0 0", "4", "19 1 0 0", "24 1 0 0", "28 1 1 0", "33 1 1 0",
    ]);
    let p_actions = Action::find_next_actions(&game, &board, true);
    let e_actions = Action::find_next_actions(&game, &board, false);
    let mut r1 = rand::thread_rng();
    let d = Instant::now();
    let mut x = Vec::new();
    for _ in 0..500000 {
        let p_actions = Action::find_next_actions(&game, &board, true);
        let e_actions = Action::find_next_actions(&game, &board, false);
        let act_1 = p_actions.choose(&mut r1).unwrap();
        let act_2 = e_actions.choose(&mut r1).unwrap();
        let next = game.apply_actions(&board, *act_1, *act_2);
        x.push(next.day);
    }

    println!("{} ms.", Duration::as_millis(&d.elapsed()),);
}

pub fn main4() {
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
    for _ in 0..10 {
        sim.simulate_current(&mut cache);
    }

    println!("{} ms.", Duration::as_millis(&d.elapsed()),);

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
#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use super::*;
    use crate::board::*;

    #[bench]
    fn bench_play(b: &mut Bencher) {
        let board = Board::default_with_inactive(vec![26, 30, 14, 0, 8, 21, 35].into_iter());
        let game = Game::parse_from_strings(vec![
            "0", "20", "2 0", "2 0 0", "4", "19 1 0 0", "24 1 0 0", "28 1 1 0", "33 1 1 0",
        ]);

        b.iter(|| {
            let _ = play(&board, game.clone(), 25);
        })
    }

    #[bench]
    fn bench_grow(b: &mut Bencher) {
        let board = Board::default_with_inactive(vec![26, 30, 14, 0, 8, 21, 35].into_iter());
        let game = Game::parse_from_strings(vec![
            "0", "20", "2 0", "2 0 0", "4", "19 1 0 0", "24 1 0 0", "28 1 1 0", "33 1 1 0",
        ]);

        let sample_board = play(&board, game, 15);

        b.iter(move || {
            let _ = Action::find_next_actions(&sample_board, &board, true);
        });
    }

    #[bench]
    fn bench_seed(b: &mut Bencher) {
        let board = Board::default_with_inactive(vec![26, 30, 14, 0, 8, 21, 35].into_iter());
        let game = Game::parse_from_strings(vec![
            "0", "20", "2 0", "2 0 0", "4", "19 1 0 0", "24 1 0 0", "28 1 1 0", "33 1 1 0",
        ]);

        let sample_board = play(&board, game, 15);

        b.iter(move || {
            let _ = Action::find_next_seed_actions(&sample_board, &board, true);
        });
    }
}

use std::{cell, io};
mod actions;
mod board;
mod game;
mod parse;
use actions::*;
use board::{Board, Cell};
use parse::*;

use game::get_next_action_wood;

use crate::game::Game;

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

    let board: Board = cells.into_iter().collect();

    // game loop
    loop {
        let day: i32 = Next::read(); // the game lasts 24 days: 0-23
        let nutrients: i32 = Next::read(); // the base score you gain from the next COMPLETE action
        let inputs: Vec<i32> = Next::read_many();
        let sun = inputs[0]; // your sun points
        let score = inputs[1]; // your current score
        let inputs: Vec<i32> = Next::read_many();
        let opp_sun = inputs[0]; // opponent's sun points
        let opp_score = inputs[1]; // opponent's score
        let opp_is_waiting = inputs[2]; // whether your opponent is asleep until the next day
        let number_of_trees: i32 = Next::read(); // the current amount of trees
        for i in 0..number_of_trees as usize {
            let inputs: Vec<i32> = Next::read_many();
            let cell_index = inputs[0]; // location of this tree
            let size = inputs[1]; // size of this tree: 0-3
            let is_mine = inputs[2]; // 1 if this is your tree
            let is_dormant = inputs[3]; // 1 if this tree is dormant
        }

        let number_of_possible_moves: i32 = Next::read(); //test
        let mut actions = Vec::<Action>::new();
        for i in 0..number_of_possible_moves as usize {
            let possible_move: Action = Next::read();
            //eprintln!("{:?}", possible_move);
            actions.push(possible_move);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>
        println!(
            "{}",
            game::get_next_action_wood(&Game::new(&board), &actions)
        );
    }
}

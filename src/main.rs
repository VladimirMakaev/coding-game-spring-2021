use std::io;
pub mod parse;
use parse::*;

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let number_of_cells: i32 = Next::read(); // 37

    for i in 0..number_of_cells as usize {
        let inputs: Vec<i32> = Next::read_many();
        let index = inputs[0]; // 0 is the center cell, the next cells spiral outwards
        let richness = inputs[1]; // 0 if the cell is unusable, 1-3 for usable cells
        let neigh_0 = inputs[2]; // the index of the neighbouring cell for each direction
        let neigh_1 = inputs[3];
        let neigh_2 = inputs[4];
        let neigh_3 = inputs[5];
        let neigh_4 = inputs[6];
        let neigh_5 = inputs[7];
    }

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
        let number_of_possible_moves: i32 = Next::read();

        for i in 0..number_of_possible_moves as usize {
            let possible_move: String = Next::read();
            eprint!("{}", possible_move);
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");

        // GROW cellIdx | SEED sourceIdx targetIdx | COMPLETE cellIdx | WAIT <message>
        println!("WAIT");
    }
}

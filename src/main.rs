mod sudoku_solver;
mod cell;
mod point;
mod sudoku_errors;
mod sudoku;

use crate::sudoku::Sudoku;

fn main() {
    let mut sudoku = Sudoku::new_puzzle(60);
    sudoku.solve().unwrap();

    println!("{}", sudoku);
    println!("Is sudoku correct: {}", sudoku.check());
}

// fn main() {
//     let mut sudoku = Sudoku::new_puzzle(60);
//     sudoku.set_cell(Point::new(0, 0), NonZeroU8::new(1).unwrap());
//     sudoku.set_cell(Point::new(1, 0), NonZeroU8::new(1).unwrap());
//
//     if let Err(e) = sudoku.solve() {
//         eprintln!("Error: {}", e);
//         return;
//     }
//
//     println!("{}", sudoku);
//     println!("Is sudoku correct: {}", sudoku.check());
// }

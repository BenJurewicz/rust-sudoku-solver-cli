mod sudoku;
mod cell;
mod point;

use crate::sudoku::SudokuSolver;

fn main() {
    // let mut sudoku = SudokuSolver::new([
    //                           [1, 0, 0, 0, 0, 0, 0, 0, 0],
    //                           [0, 0, 0, 0, 0, 0, 0, 0, 0],
    //                           [0, 0, 0, 0, 0, 0, 0, 0, 0],
    //                           [0, 9, 0, 0, 0, 0, 0, 0, 0],
    //                           [0, 0, 0, 0, 0, 0, 0, 0, 0],
    //                           [0, 8, 0, 0, 0, 0, 0, 0, 0],
    //                           [0, 7, 0, 0, 0, 0, 0, 0, 0],
    //                           [0, 6, 0, 0, 0, 0, 0, 0, 0],
    //                           [0, 5, 0, 0, 0, 0, 0, 0, 0]
    // ]);
    let sudoku = SudokuSolver::new([
        [0, 0, 0, 0, 0, 0, 0, 8, 0],
        [6, 8, 0, 4, 7, 0, 0, 2, 0],
        [0, 1, 9, 5, 0, 8, 6, 4, 7],
        [0, 6, 0, 9, 0, 0, 0, 0, 4],
        [3, 4, 2, 6, 8, 0, 0, 0, 0],
        [1, 9, 0, 0, 5, 0, 8, 3, 0],
        [0, 0, 0, 7, 2, 0, 4, 0, 3],
        [0, 0, 6, 0, 0, 5, 0, 1, 0],
        [0, 0, 3, 8, 9, 1, 5, 0, 0]
    ]);
    match sudoku {
        Ok(mut sudoku) => {
            sudoku.solve();
            println!("Is sudoku correct: {}", sudoku.check_if_correct());
            println!("{}", sudoku);
        },
        Err(e) => println!("Error: {}", e)
    }
}
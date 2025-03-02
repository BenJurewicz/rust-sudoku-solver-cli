use std::array::from_fn;
use std::collections::HashSet;
use std::num::NonZeroU8;

use crate::cell::Cell;
use crate::point::Point;
pub use crate::sudoku_errors::*;

// type Sudoku = Vec<Vec<Cell>>;
type Sudoku = [[Cell; 9]; 9];

#[derive(Debug, Clone)]
pub struct SudokuSolver {
    board: Sudoku,
    previous_states: Vec<Sudoku>,
}

impl SudokuSolver {
    pub fn new_empty() -> Self {
        SudokuSolver {
            board: from_fn(|_| from_fn(|_| Cell::new_empty())),
            previous_states: Vec::with_capacity(81), // sudoku is 9x9 so there is 81 max moves on a totally empty board
        }
    }

    pub fn new(starting_state: [[Option<NonZeroU8>; 9]; 9]) -> Result<Self, ErrorNoSolution> {
        let mut sudoku = SudokuSolver::new_empty();

        for (y, row) in starting_state.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Some(cell) = cell {
                    sudoku.board[y][x] = Cell::new_filled(u8::from(*cell));
                    sudoku.propagate_collapse(Point::new(x, y), u8::from(*cell)).map_err(|_| ErrorNoSolution)?;
                }
            }
        }

        Ok(sudoku)
    }

    fn get_cell_mut(&mut self, cell_coords: &Point<usize>) -> &mut Cell {
        &mut self.board[cell_coords.y][cell_coords.x]
    }

    fn board_to_option_array(self) -> [[Option<NonZeroU8>; 9]; 9] {
        self.board.map(|row| row.map(
            |cell| {
                match cell {
                    Cell::Collapsed(n) => Some(NonZeroU8::try_from(n).unwrap()),
                    Cell::Uncollapsed(_) => None
                }
            }))
    }

    pub fn solve(mut self) -> Result<[[Option<NonZeroU8>; 9]; 9], ErrorNoSolution>{
        let mut solved = false;

        while !solved {
            match self.solve_iteration() {
                Ok(true) => solved = true,
                Ok(false) => continue,
                Err(_) => match self.previous_states.pop() {
                    Some(previous_state) => self.board = previous_state,
                    None => break
                }
            }
        }

        if solved {
            Ok(self.board_to_option_array())
        } else {
            Err(ErrorNoSolution)
        }
    }

    // returns true if sudoku is solved, false if not and Err if there is a contradiction
    fn solve_iteration(&mut self) -> Result<bool, ()> {
        match self.get_coords_of_uncollapsed_cell_with_lowest_entropy() {
            Some(cell_coords) => { self.collapse_cell_and_save_state(cell_coords)?; Ok(false) },
            None => Ok(true) // sudoku is solved
        }
    }

    fn collapse_cell_and_save_state(&mut self, cell_coords: Point<usize>) -> Result<(), ()> {
        let cell = self.get_cell_mut(&cell_coords);
        let should_save = cell.get_entropy() > 1;
        let value_with_collapsed_num_removed = cell.collapse();
        let Cell::Collapsed(collapsed_to_num) = *cell else { unreachable!() };

        if should_save {
            let mut board = self.board.clone();
            board[cell_coords.y][cell_coords.x] = value_with_collapsed_num_removed;
            self.previous_states.push(board);
        }

        self.propagate_collapse(cell_coords, collapsed_to_num)?;
        Ok(())
    }

    fn propagate_collapse(&mut self, cell_coords: Point<usize>, value: u8) -> Result<(), ()> {
        let relatives_coords = self.get_relatives(cell_coords);
        for relative_cords in relatives_coords {
            self.get_cell_mut(&relative_cords).remove(value)?;
        }
        Ok(())
    }

    fn get_relatives(&self, cell_coords: Point<usize>) -> Vec<Point<usize>> {
       // row + column + small square - repetitions = 3*8-4 = 20
        let mut relatives = HashSet::with_capacity(20);
        relatives.extend(self.get_row(cell_coords.y));
        relatives.extend(self.get_column(cell_coords.x));
        relatives.extend(self.get_region(cell_coords));
        relatives.remove(&cell_coords);
        relatives.into_iter().collect()
    }

    /// Return the coordinates of the top left corner of the region that the cell belongs to
    fn get_region_coords(&self, cell_coords: Point<usize>) -> Point<usize> {
        Point::new(cell_coords.x / 3, cell_coords.y / 3) * 3
    }

    fn get_coords_of_uncollapsed_cell_with_lowest_entropy(&self) -> Option<Point<usize>> {
        let mut cell = None::<Point<usize>>;
        let mut lowest_entropy = u8::MAX;

        for (y ,row) in self.board.iter().enumerate() {
            for (x, current_cell) in row.iter().enumerate() {
                if let Cell::Collapsed(_) = current_cell {
                    continue;
                }
                let current_entropy = current_cell.get_entropy();
                if current_entropy < lowest_entropy {
                    lowest_entropy = current_entropy;
                    cell = Some(Point::new(x, y));
                }
            }
        }

        cell
    }

    fn get_region(&self, point: Point<usize>) -> HashSet<Point<usize>> {
        let mut relatives = HashSet::with_capacity(9);
        let region_coords = self.get_region_coords(point);
        for y in region_coords.y..region_coords.y + 3 {
            for x in region_coords.x..region_coords.x + 3 {
                relatives.insert(Point::new(x, y));
            }
        }
        relatives
    }

    fn get_row(&self, y : usize) -> HashSet<Point<usize>> {
        let mut relatives = HashSet::with_capacity(9);
        for x in 0..9 {
            relatives.insert(Point::new(x, y));
        }
        relatives
    }

    fn get_column(&self, x : usize) -> HashSet<Point<usize>> {
        let mut relatives = HashSet::with_capacity(9);
        for y in 0..9 {
            relatives.insert(Point::new(x, y));
        }
        relatives
    }
}
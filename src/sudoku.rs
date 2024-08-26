use crate::cell::Cell;
use crate::point::Point;

use std::collections::HashSet;

type Sudoku = Vec<Vec<Cell>>;

#[derive(Debug, Clone)]
pub struct SudokuSolver {
    board: Sudoku,
    previous_states: Vec<Sudoku>,
    debug_view: String
}

#[derive(Debug, Clone)]
pub struct ErrorSudokuContainsAContradiction;
impl std::fmt::Display for ErrorSudokuContainsAContradiction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The sudoku cannot be solved because it contains a contradiction in the initial state")
    }
}

#[derive(Debug, Clone)]
pub struct SudokuIsUnsolvable;
impl std::fmt::Display for SudokuIsUnsolvable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The sudoku contains a contradiction that could not be detected when initializing the SudokuSolver")
    }
}

// sudokuBuilder would be nice
impl SudokuSolver {
    pub fn new(starting_state: [[u8; 9]; 9]) -> Result<Self, ErrorSudokuContainsAContradiction> {
        let mut sudoku = SudokuSolver {
            board: vec![vec![Cell::new_empty(); 9]; 9],
            previous_states: Vec::with_capacity(81), // sudoku is 9x9 so there is 81 max moves on a totally empty board
            debug_view: String::new()
        };

        for (y, row) in starting_state.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == 0  { continue; }
                sudoku.board[y][x] = Cell::new_filled(*cell);
                sudoku.propagate_collapse(Point::new(x, y), *cell).map_err(|_| ErrorSudokuContainsAContradiction)?;
            }
        }

        Ok(sudoku)
    }

    fn get_cell(&self, cell_coords: &Point<usize>) -> &Cell {
        &self.board[cell_coords.y][cell_coords.x]
    }

    fn get_cell_mut(&mut self, cell_coords: &Point<usize>) -> &mut Cell {
        &mut self.board[cell_coords.y][cell_coords.x]
    }

    pub fn solve(&mut self) -> Result<(), SudokuIsUnsolvable>{
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
            self.debug_view = self.to_string();
        }

        if solved {
            Ok(())
        } else {
            Err(SudokuIsUnsolvable)
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
        // let mut relatives = HashSet::with_capacity(20); // row + column + small square - repetitions = 3*8-4 = 20
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

    fn get_coords_of_uncollapsed_cell_with_lowest_entropy(& self) -> Option<Point<usize>> {
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

    pub fn check_if_correct(&self) -> bool {
        self.check_rows() && self.check_columns() && self.check_regions()
    }

    fn check_rows(&self) -> bool {
        for y in 0..9 {
            let row : HashSet<_> = HashSet::from_iter(self.get_row(y));
            if !self.check_if_points_have_all_digits(&row) {
                return false;
            }
        }
        true
    }

    fn check_columns(&self) -> bool {
        for x in 0..9 {
            let column: HashSet<_> = HashSet::from_iter(self.get_column(x));
            if !self.check_if_points_have_all_digits(&column) {
                return false;
            }
        }
        true
    }

    // region is the small 3x3 square (according to some site with sudoku terminology)
    fn check_regions(&self) -> bool {
        for y in [0, 3, 6]{
            for x in [0, 3, 6]{
                let region = self.get_region(Point::new(x, y));
                if !self.check_if_points_have_all_digits(&region) {
                    return false;
                }
            }
        }
        true
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

    fn check_if_points_have_all_digits(&self, hash: &HashSet<Point<usize>>) -> bool {
        self.check_if_hash_has_all_digits(self.points_to_digits(hash))
    }


    fn points_to_digits(&self, points: &HashSet<Point<usize>>) -> HashSet<u8> {
        let mut digits = HashSet::with_capacity(points.len());
        for point in points {
            if let Cell::Collapsed(value) = self.get_cell(point) {
                digits.insert(*value);
            } else {
                digits.insert(0);
            }
        }

        digits
    }

    fn check_if_hash_has_all_digits(&self, hash: HashSet<u8>) -> bool {
        let mut digits: HashSet<u8> = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        for digit in hash.iter() {
            if !digits.remove(digit) {
                return false;
            }
        }
        digits.is_empty()
    }

}

impl std::fmt::Display for SudokuSolver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Collapsed(value) => write!(f, "{}", value)?,
                    Cell::Uncollapsed(_) => write!(f, " ")?
                }
                write!(f, " ")?;
                if x % 3 == 2 && x != row.len() - 1 {
                    write!(f, "| ")?;
                }
            }

            write!(f, "\n")?;
            if y % 3 == 2 && y != self.board.len() - 1 {
                for x in 0..(2*row.len() + 3) {
                    if x == 6 || x == 14 {
                        write!(f, "+")?;
                    } else {
                        write!(f, "-")?;
                    }
                }
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
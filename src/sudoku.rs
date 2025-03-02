use rand::prelude::SliceRandom;
use std::collections::HashSet;
use std::num::NonZeroU8;

use crate::point::Point;
use crate::sudoku_errors::ErrorNoSolution;
use crate::sudoku_solver::SudokuSolver;

type SudokuBoard = [[Option<NonZeroU8>; 9]; 9];

pub struct Sudoku {
    board: SudokuBoard,
    read_only: [[bool; 9]; 9]
}

impl Sudoku {
    pub fn new_empty() -> Self {
        Sudoku {
            board: [[None; 9]; 9],
            read_only: [[false; 9]; 9]
        }
    }

    /// Creates a new Sudoku puzzle with the given difficulty.
    /// # Arguments
    /// * `difficulty` - a number between 1 and 81 (inclusive) that represents the number of empty cells in the puzzle
    /// # Returns
    /// * a new Sudoku puzzle with the given difficulty
    pub fn new_puzzle(difficulty: u8) -> Self {
        let puzzle = Self::create_puzzle(difficulty);
        Sudoku {
            board: puzzle,
            read_only: Self::infer_read_only(puzzle)
        }
    }

    fn create_puzzle(difficulty: u8) -> SudokuBoard {
        let difficulty = difficulty.min(81);

        let mut rng = rand::rng();
        let mut points: Vec<(usize, usize)> = Vec::with_capacity(81);
        for y in 0..9 {
            for x in 0..9 {
                points.push((x, y));
            }
        }
        points.shuffle(&mut rng);

        let mut puzzle = SudokuSolver::new_empty().solve().unwrap();
        for _ in 0..difficulty {
            let (x, y) = points.pop().unwrap();
            puzzle[y][x] = None;
        }
        puzzle
    }

    fn infer_read_only(board: SudokuBoard) -> [[bool; 9]; 9] {
        let mut read_only = [[false; 9]; 9];
        for y in 0..9 {
            for x in 0..9 {
                if let Some(_) = board[y][x] {
                    read_only[y][x] = true;
                }
            }
        }
        read_only
    }

    pub fn get_cell(&self, point: Point<usize>) -> Option<NonZeroU8> {
        self.board[point.y][point.x]
    }

    pub fn set_cell(&mut self, point: Point<usize>, value: NonZeroU8) {
        self.board[point.y][point.x] = Some(value);
    }

    pub fn is_read_only(&self, point: Point<usize>) -> bool {
        self.read_only[point.y][point.x]
    }

    pub fn solve(&mut self) -> Result<(), ErrorNoSolution> {
        let solved_sudoku= SudokuSolver::new(self.board).and_then(|s| s.solve());
        match solved_sudoku {
            Ok(solved_sudoku) => {
                self.board = solved_sudoku;
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    pub fn check(&self) -> bool {
        // Early return if not full
        self.is_full() && self.check_rows() && self.check_columns() && self.check_regions()
    }

    fn is_full(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|cell| cell.is_some()))
    }

    fn check_rows(&self) -> bool {
        for y in 0..9 {
            let row = HashSet::from_iter(self.get_row(y));
            if !self.check_if_points_have_all_digits(&row) {
                return false;
            }
        }
        true
    }

    fn get_row(&self, y : usize) -> HashSet<Point<usize>> {
        let mut relatives = HashSet::with_capacity(9);
        for x in 0..9 {
            relatives.insert(Point::new(x, y));
        }
        relatives
    }

    fn check_columns(&self) -> bool {
        for x in 0..9 {
            let column = HashSet::from_iter(self.get_column(x));
            if !self.check_if_points_have_all_digits(&column) {
                return false;
            }
        }
        true
    }

    fn get_column(&self, x : usize) -> HashSet<Point<usize>> {
        let mut relatives = HashSet::with_capacity(9);
        for y in 0..9 {
            relatives.insert(Point::new(x, y));
        }
        relatives
    }

    /// region is the small 3x3 square (according to some site with sudoku terminology)
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

    fn get_region_coords(&self, cell_coords: Point<usize>) -> Point<usize> {
        Point::new(cell_coords.x / 3, cell_coords.y / 3) * 3
    }

    fn check_if_points_have_all_digits(&self, points: &HashSet<Point<usize>>) -> bool {
        self.check_if_set_has_all_digits(self.points_to_digits(points))
    }

    fn points_to_digits(&self, points: &HashSet<Point<usize>>) -> HashSet<u8> {
        let mut digits = HashSet::with_capacity(points.len());
        for point in points {
            if let Some(value) = self.get_cell(*point) {
                digits.insert(value.into());
            } else {
                // Insert a number not from range 1-9 for empty cells
                digits.insert(0);
            }
        }
        digits
    }

    /// Check if a given set has exactly all digits from 1-9
    fn check_if_set_has_all_digits(&self, set: HashSet<u8>) -> bool {
        let mut digits: HashSet<u8> = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        for digit in set.iter() {
            if !digits.remove(digit) {
                return false;
            }
        }
        digits.is_empty()
    }
}

impl std::fmt::Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Some(value) => write!(f, "{}", value)?,
                    None => write!(f, " ")?
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

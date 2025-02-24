#[derive(Debug, Clone)]
pub enum Error {
    ContainsAContradiction,
    IsUnsolvable
}

impl Error {
    pub fn contains_a_contradiction() -> Self {
        Error::ContainsAContradiction
    }

    pub fn is_unsolvable() -> Self {
        Error::IsUnsolvable
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ContainsAContradiction => write!(
                f, "The sudoku cannot be solved because it contains a contradiction in the initial state"),
            Error::IsUnsolvable => write!(
                f, "The sudoku contains a contradiction that could not be detected when initializing the SudokuSolver")
        }
    }
}
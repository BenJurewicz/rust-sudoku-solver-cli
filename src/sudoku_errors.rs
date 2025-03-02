#[derive(Debug, Clone)]
pub struct ErrorNoSolution;

impl std::fmt::Display for ErrorNoSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sudoku has no valid solution")
    }
}

impl std::error::Error for ErrorNoSolution {}


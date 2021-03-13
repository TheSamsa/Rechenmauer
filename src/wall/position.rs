use std::default::Default;
use std::fmt::Display;

pub struct Position {
    pub row: u8,
    pub col: u8,
}

impl Position {
    pub fn new(row: u8, col: u8) -> Self {
        Self { row, col }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { row: 0, col: 0 }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

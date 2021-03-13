use super::position::Position;
use std::fmt::Display;

pub struct Cell {
    pos: Position,
    nck: usize,
    value: Option<isize>,
}

impl Cell {
    pub fn new(pos: Position, value: Option<isize>) -> Self {
        let n = pos.row as usize;
        let k = pos.col as usize;
        let nck = super::n_choose_k(n, k);

        Self { pos, nck, value }
    }

    pub fn set(&mut self, value: isize) -> Result<(), isize> {
        if let Some(actual_value) = self.value {
            if actual_value != value {
                return Err(actual_value);
            }
        } else {
            self.value = Some(value);
        }

        Ok(())
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            pos: Position::default(),
            nck: 1,
            value: None,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(value) = self.value {
            write!(f, " {:^5} ", value)
        } else {
            write!(f, "       ")
        }
    }
}

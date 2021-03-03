use block_map::BlockMap;
use std::fmt;
use std::fmt::Display;

pub use block_map::Position;

mod block_map;

pub struct Mauer {
    rows: usize,
    blocks: BlockMap,
}

impl Mauer {
    pub fn new(rows: usize) -> Self {
        Self {
            rows,
            blocks: BlockMap::new(rows),
        }
    }

    pub fn set(&mut self, pos: Position, value: isize) {
        self.blocks.set(pos, value);
    }

    pub fn solve(&mut self) {
        self.calc_possible();

        // if not all values are known here, we have to try and use some maths.
        if !self.blocks.solved() {
            // first check bottom line if it has only one missing value
            let bottom_lane = self.blocks.bottom_lane();

            if self.blocks.bottom_lane_value_count() == 1 {
                // only one value missing
                let missing_pos =
                    bottom_lane
                        .iter()
                        .fold(
                            Position::new(0, 0),
                            |ret, (pos, val)| {
                                if val.is_none() {
                                    **pos
                                } else {
                                    ret
                                }
                            },
                        );

                // now we look for a existing value above it and use it as top of a (possible smaller) block_map so we can use an equation
                if let (result_pos, Some(result)) = self.blocks.bottom_equation(missing_pos) {
                    self.blocks.set(result_pos, result);
                }
            }

            self.calc_possible();
            //        let left_lane = self.blocks.iter().filter(|(pos, _)| pos.1 == 1);
            //        let right_lane = self.blocks.iter().filter(|(pos, _)| pos.0 == pos.1);
        }

        // TODO do integrity check afterwards
    }

    fn calc_possible(&mut self) {
        let mut changes = true;
        while changes {
            let value_count = self.blocks.value_count();

            self.blocks.calc_all();

            let new_value_count = self.blocks.value_count();

            if value_count == new_value_count {
                changes = false;
            }
        }
    }
}

impl Display for Mauer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut format = String::from("");

        for row in 1..=self.rows {
            let identation = self.rows - row;
            let mut top_line = " ".chars().cycle().take(4 * identation).collect::<String>();
            let mut line = " ".chars().cycle().take(4 * identation).collect::<String>();
            for pos in 1..=row {
                let block = self.blocks.get(&Position::new(row, pos));

                top_line = format!("{}+-------", top_line);
                if let Some(block) = block {
                    if let Some(value) = block {
                        line = format!("{}| {:^5} ", line, value);
                    } else {
                        line = format!("{}|       ", line);
                    }
                } else {
                    line = format!("{}|       ", line);
                }
                if pos == row {
                    top_line = format!("{}+\n", top_line);
                    line = format!("{}|\n", line);
                }
            }

            format = format!("{}{}{}", format, top_line, line);
        }
        format = format!(
            "{}{}+\n",
            format,
            "+-------"
                .chars()
                .cycle()
                .take(8 * self.rows)
                .collect::<String>()
        );
        write!(f, "{}", format)
    }
}

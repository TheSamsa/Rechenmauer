use block::Block;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};

mod block;
mod block_map;

type Position = (usize, usize);

pub struct Mauer<T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> {
    rows: usize,
    size: usize,
    blocks: BTreeMap<Position, Option<T>>,
}

impl<T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> Mauer<T> {
    pub fn new(rows: usize) -> Self {
        let mut size = 1;
        let mut blocks = BTreeMap::new();

        for i in 1..=rows {
            for j in 1..=i {
                size += 1;
                blocks.insert((i, j), None);
            }
        }

        Self { rows, size, blocks }
    }

    pub fn set(&mut self, pos: Position, value: T) {
        self.blocks.insert(pos, Some(value));
    }

    pub fn solve(&self) -> Option<Mauer<T>> {
        let blocks = self.calc_possible();

        // if not all values are known here, we have to try and use some maths.
        if blocks.iter().filter(|(_, val)| val.is_some()).count() < self.size {
            // first check bottom line if it has only one missing value
            let bottom_lane: BTreeMap<&Position, &Option<T>> = blocks
                .iter()
                .filter(|(pos, _)| pos.0 == self.rows)
                .collect();

            if (bottom_lane
                .iter()
                .filter(|(_, value)| value.is_none())
                .count())
                == 1
            {
                // only one value missing
                // look upwards from missing value if we find one
            }
            //        let left_lane = self.blocks.iter().filter(|(pos, _)| pos.1 == 1);
            //        let right_lane = self.blocks.iter().filter(|(pos, _)| pos.0 == pos.1);
        }

        // TODO do integrity check afterwards

        Some(Self {
            rows: self.rows,
            size: self.size,
            blocks,
        })
    }

    fn calc_possible(&self) -> BTreeMap<Position, Option<T>> {
        let mut new_blocks = self.blocks.clone();
        let mut changes = true;
        while changes {
            let value_count = new_blocks
                .iter()
                .filter(|(_, block)| block.is_some())
                .count();

            for (pos, _) in self.blocks.iter() {
                let block = Block::new(&pos, &new_blocks);
                let value = block.calc();
                new_blocks.insert(*pos, value);
            }

            let new_value_count = new_blocks
                .iter()
                .filter(|(_, block)| block.is_some())
                .count();

            if value_count == new_value_count {
                changes = false;
            }
        }

        new_blocks
    }
}

impl<T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> Display for Mauer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut format = String::from("");

        for row in 1..=self.rows {
            let identation = self.rows - row;
            let mut top_line = " ".chars().cycle().take(4 * identation).collect::<String>();
            let mut line = " ".chars().cycle().take(4 * identation).collect::<String>();
            for pos in 1..=row {
                let block = self.blocks.get(&(row, pos));

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

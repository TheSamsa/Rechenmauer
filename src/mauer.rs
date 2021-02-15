use block_map::BlockMap;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};

mod block;
mod block_map;

type Position = (usize, usize);

pub struct Mauer<'a, T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> {
    size: usize,
    blocks: BlockMap<'a, T>,
}

impl<'a, T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> Mauer<'a, T> {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            blocks: BlockMap::new(size),
        }
    }

    pub fn set(&'a mut self, pos: Position, value: T) {
        if let Some(block) = self.blocks.get_mut(pos) {
            block.set(value);
        }
    }

    pub fn solve(&self) -> Option<Mauer<'a, T>> {
        None
    }

    fn calc_possible(&self) -> BlockMap<'a, T> {
        let new_blocks = self.blocks.clone();
        let mut changes = true;
        while changes {
            let value_count = new_blocks
                .iter()
                .filter(|(_, block)| block.get().is_some())
                .count();

                //

            let new_value_count = new_blocks
                .iter()
                .filter(|(_, block)| block.get().is_some())
                .count();

            if value_count == new_value_count {
                changes = false;
            }
        }

        new_blocks
    }

    //    pub fn solve(&mut self) -> Option<Mauer> {
    //        // first calculate every possible combination of adjacent blocks for bottom to top
    //        let new_blocks = self.calc_possible();
    //
    //        // first check bottom line if it has only one missing value
    //        let bottom_lane: HashMap<&Position, &usize> = self
    //            .blocks
    //            .iter()
    //            .filter(|(pos, _)| pos.0 == self.size)
    //            .collect();
    //        if (bottom_lane.iter().count() - self.size) == 1 {
    //            // only one value missing
    //            // look upwards from missing value if we find one
    //            bottom_lane
    //                .keys()
    //                .map(|(_, pos)| *pos)
    //                .collect::<Vec<usize>>()
    //                .sort();
    //        }
    //        //        let left_lane = self.blocks.iter().filter(|(pos, _)| pos.1 == 1);
    //        //        let right_lane = self.blocks.iter().filter(|(pos, _)| pos.0 == pos.1);
    //
    //        Some(Mauer {
    //            size: self.size,
    //            blocks: new_blocks,
    //        })
    //    }
    //
    //    fn calc_possible(&self) -> HashMap<Position, usize> {
    //        let mut new_blocks = self.blocks.clone();
    //        for (row, pos) in self.walker() {
    //            if row == 1 || row == pos {
    //                continue;
    //            }
    //
    //            if let Some(block_value) = self.blocks.get(&(row, pos)) {
    //                if let Some(adjacent_value) = self.blocks.get(&(row, pos + 1)) {
    //                    let new_pos = (row - 1, pos);
    //                    let new_value = block_value + adjacent_value;
    //                    if let Some(existing_value) = new_blocks.get(&new_pos) {
    //                        if *existing_value != new_value {
    //                            println!("want to overwrite {} with {}", existing_value, new_value);
    //                        }
    //                    } else {
    //                        new_blocks.insert((row - 1, pos), block_value + adjacent_value);
    //                    }
    //                }
    //            }
    //        }
    //
    //        new_blocks
    //    }
    //
    //    fn walker(&self) -> Vec<(usize, usize)> {
    //        let walker_rows = (1..=self.size).fold(Vec::new(), |mut walker, i| {
    //            (1..=i).for_each(|_| walker.push(i));
    //
    //            walker
    //        });
    //        let walker_blocks = (1..=self.size).fold(Vec::new(), |mut walker, i| {
    //            (1..=i).for_each(|j| walker.push(j));
    //
    //            walker
    //        });
    //
    //        walker_rows
    //            .into_iter()
    //            .zip(walker_blocks.into_iter())
    //            .collect()
    //    }
}

impl<'a, T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> Display
    for Mauer<'_, T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut format = String::from("");

        for row in 1..=self.size {
            let identation = self.size - row;
            let mut top_line = " ".chars().cycle().take(4 * identation).collect::<String>();
            let mut line = " ".chars().cycle().take(4 * identation).collect::<String>();
            for pos in 1..=row {
                let block = self.blocks.get((row, pos));

                top_line = format!("{}+-------", top_line);
                if let Some(block) = block {
                    //dbg!(block);
                    if let Some(value) = block.get() {
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
                .take(8 * self.size)
                .collect::<String>()
        );
        write!(f, "{}", format)
    }
}

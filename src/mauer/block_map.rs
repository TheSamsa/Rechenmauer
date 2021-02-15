use super::block::Block;
use std::collections::{BTreeMap, btree_map};
use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};

type Position = (usize, usize);

#[derive(Debug, Clone)]
pub struct BlockMap<'a, T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> {
    size: usize,
    inner: BTreeMap<Position, Block<'a, T>>,
}

impl<'a, T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> BlockMap<'a, T> {
    pub fn new(size: usize) -> Self {
        let mut inner = BTreeMap::new();

        for i in 1..=size {
            for j in 1..=i {
                let pos = (i, j);
                let block = Block::new(pos, None);
                inner.insert((i, j), block);
            }
        }

        Self { size, inner }
    }

    pub fn iter(&self) -> btree_map::Iter<Position, Block<'a, T>> {
        self.inner.iter()
    }

    pub fn get(&mut self, position: Position) -> Option<&'a Block<T>> {
        let block = self.inner.get(&position);

        if let Some(block) = block {
            let top_left = self.inner.get(&(position.0 - 1, position.1 - 1));
            let top_right = self.inner.get(&(position.0 - 1, position.1));
            let left = self.inner.get(&(position.0, position.1 - 1));
            let right = self.inner.get(&(position.0, position.1 + 1));
            let bottom_left = self.inner.get(&(position.0 + 1, position.1));
            let bottom_right = self.inner.get(&(position.0 + 1, position.1 + 1));

            block.set_adjacent(top_left, top_right, left, right, bottom_left, bottom_right);
        }

        block
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&'a mut Block<T>> {
        let block = self.inner.get_mut(&position);

        if let Some(block) = block {
            let top_left = self.inner.get(&(position.0 - 1, position.1 - 1));
            let top_right = self.inner.get(&(position.0 - 1, position.1));
            let left = self.inner.get(&(position.0, position.1 - 1));
            let right = self.inner.get(&(position.0, position.1 + 1));
            let bottom_left = self.inner.get(&(position.0 + 1, position.1));
            let bottom_right = self.inner.get(&(position.0 + 1, position.1 + 1));

            block.set_adjacent(top_left, top_right, left, right, bottom_left, bottom_right);
        }

        block
       
    }
}


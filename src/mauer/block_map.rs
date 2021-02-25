use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Position(usize, usize);

impl Position {
    pub fn top_left(&self) -> Self {
        Self(self.0.saturating_sub(1), self.1.saturating_sub(1))
    }

    pub fn top_right(&self) -> Self {
        Self(self.0.saturating_sub(1), self.1)
    }

    pub fn left(&self) -> Self {
        Self(self.0, self.1.saturating_sub(1))
    }

    pub fn right(&self) -> Self {
        Self(self.0, self.1.saturating_add(1))
    }

    pub fn bottom_left(&self) -> Self {
        Self(self.1.saturating_add(1), self.1)
    }

    pub fn bottom_right(&self) -> Self {
        Self(self.0.saturating_add(1), self.1.saturating_add(1))
    }
}

struct Block<'t, T> {
    pos: Position,
    value: Option<T>,
    top_left: Option<&'t Block<'t, T>>,
    top_right: Option<&'t Block<'t, T>>,
    left: Option<&'t Block<'t, T>>,
    right: Option<&'t Block<'t, T>>,
    bottom_left: Option<&'t Block<'t, T>>,
    bottom_right: Option<&'t Block<'t, T>>,
}

impl<'t, T> Block<'t, T> {
    pub fn new(pos: Position, value: Option<T>) -> Self {
        Self {
            pos,
            value,
            top_left: None,
            top_right: None,
            left: None,
            right: None,
            bottom_left: None,
            bottom_right: None,
        }
    }

    fn set_adjacent(
        &mut self,
        top_left: Option<&'t Block<T>>,
        top_right: Option<&'t Block<T>>,
        left: Option<&'t Block<T>>,
        right: Option<&'t Block<T>>,
        bottom_left: Option<&'t Block<T>>,
        bottom_right: Option<&'t Block<T>>,
    ) {
        self.top_left = top_left;
        self.top_right = top_right;
        self.left = left;
        self.right = right;
        self.bottom_left = bottom_left;
        self.bottom_right = bottom_right;
    }
}

struct BlockMap<'a, T> {
    inner: BTreeMap<Position, Block<'a, T>>,
}

impl<'a, T> BlockMap<'a, T> {
    pub fn new(input: BTreeMap<Position, Option<T>>) -> Self {
        let mut inner = BTreeMap::new();
        for (pos, val) in input.into_iter() {
            let block = Block::new(pos, val);
            inner.insert(pos, block);
        }

        for (pos, val) in &mut inner {
            let top_left = inner
                .get(&pos.top_left())
                .map_or(None, |tl| Some(tl));
            let top_right = inner
                .get(&pos.top_right())
                .map_or(None, |tr| Some(tr));
            let left = inner
                .get(&pos.left())
                .map_or(None, |l| Some(l));
            let right = inner
                .get(&pos.right())
                .map_or(None, |r| Some(r));
            let bottom_left = inner
                .get(&pos.bottom_left())
                .map_or(None, |bl| Some(bl));
            let bottom_right = inner
                .get(&pos.bottom_right())
                .map_or(None, |br| Some(br));

            val.set_adjacent(top_left, top_right, left, right, bottom_left, bottom_right);
        }

        Self { inner }
    }
}

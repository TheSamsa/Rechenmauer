use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::BTreeMap;
use std::fmt::{Debug, Display};
use std::iter::Iterator;
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Self(row, col)
    }

    fn spire() -> Self {
        Self(1, 1)
    }

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
        Self(self.0.saturating_add(1), self.1)
    }

    pub fn bottom_right(&self) -> Self {
        Self(self.0.saturating_add(1), self.1.saturating_add(1))
    }
}

#[derive(Clone)]
pub struct BlockMap<T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> {
    rows: usize,
    size: usize,
    positions: Vec<Position>,
    inner: BTreeMap<Position, Option<T>>,
}

impl<T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> BlockMap<T> {
    pub fn new(rows: usize) -> Self {
        let mut size = 1;
        let mut positions = Vec::new();
        let mut inner = BTreeMap::new();

        for i in 1..=rows {
            for j in 1..=i {
                size += 1;
                let pos = Position::new(i, j);
                inner.insert(pos, None);
                positions.push(pos);
            }
        }

        Self {
            rows,
            size,
            positions,
            inner,
        }
    }

    pub fn get(&self, pos: &Position) -> Option<&Option<T>> {
        self.inner.get(pos)
    }

    pub fn set(&mut self, pos: Position, value: T) {
        self.inner.insert(pos, Some(value));
    }

    pub fn solved(&self) -> bool {
        self.value_count() == self.size
    }

    pub fn value_count(&self) -> usize {
        self.inner.iter().filter(|(_, val)| val.is_some()).count()
    }

    pub fn bottom_lane(&self) -> BTreeMap<&Position, &Option<T>> {
        self.inner
            .iter()
            .filter(|(pos, _)| pos.0 == self.rows)
            .collect()
    }

    pub fn bottom_lane_value_count(&self) -> usize {
        self.bottom_lane()
            .iter()
            .filter(|(_, value)| value.is_none())
            .count()
    }

    pub fn left_lane(&self) -> BTreeMap<&Position, &Option<T>> {
        self.inner.iter().filter(|(pos, _)| pos.1 == 1).collect()
    }

    pub fn right_lane(&self) -> BTreeMap<&Position, &Option<T>> {
        self.inner
            .iter()
            .filter(|(pos, _)| pos.0 == pos.1)
            .collect()
    }

    pub fn is_top(&self, pos: &Position) -> bool {
        self.inner.get(&pos.top_left()).is_none()
            && self.inner.get(&pos.top_right()).is_none()
            && self.inner.get(&pos.left()).is_none()
            && self.inner.get(&pos.right()).is_none()
    }

    pub fn is_left(&self, pos: &Position) -> bool {
        self.inner.get(&pos.top_left()).is_none() && self.inner.get(&pos.left()).is_none()
    }

    pub fn is_right(&self, pos: &Position) -> bool {
        self.inner.get(&pos.top_right()).is_none() && self.inner.get(&pos.right()).is_none()
    }

    pub fn is_bottom(&self, pos: &Position) -> bool {
        self.inner.get(&pos.bottom_left()).is_none()
            && self.inner.get(&pos.bottom_right()).is_none()
    }

    pub fn bottom_lane_spire(&self, missing_pos: Position) -> Option<(&Position, &Option<T>)> {
        self.inner
            .iter()
            .rev()
            .filter(|(pos, _)| {
                if pos.0 == missing_pos.0 {
                    return false;
                }

                let lower = missing_pos.1.saturating_sub(missing_pos.0 - pos.0);
                let lower = if lower < 1 { 1 } else { lower };
                let upper = if missing_pos.1 <= pos.0 { missing_pos.1 } else { pos.0 };

                if lower <= pos.1 && pos.1 <= upper {
                    true
                } else {
                    false
                }
            })
            .find(|(_, val)| val.is_some())
    }

    pub fn calc_all(&mut self) {
        for pos in self.positions.clone().iter() {
            let value = self.calc(pos);
            self.inner.insert(*pos, value);
        }
    }

    fn calc(&mut self, pos: &Position) -> Option<T> {
        // skip calculating because we already have a value
        if let Some(value) = self.inner.get(pos) {
            if value.is_some() {
                return value.as_ref().copied();
            }

            self.calc_from_bottom(pos).or(self
                .calc_from_left(pos)
                .or(self.calc_from_right(pos).or(None)))
        } else {
            None
        }
    }

    fn calc_from_bottom(&self, pos: &Position) -> Option<T> {
        if self.is_bottom(pos) {
            // TODO get rid of unwrap, works for now, because this function is only called via calc
            return self.inner.get(pos).unwrap().as_ref().copied();
        }

        // safe to unwrap because we check with is_bottom before
        let bottom_left = self.get(&pos.bottom_left()).unwrap();
        let bottom_right = self.get(&pos.bottom_right()).unwrap();

        if let Some(bl_value) = bottom_left.as_ref() {
            if let Some(br_value) = bottom_right.as_ref() {
                return Some(*bl_value + *br_value);
            }
        }

        None
    }

    fn calc_from_left(&self, pos: &Position) -> Option<T> {
        if self.is_left(pos) {
            // TODO get rid of unwrap, works for now, because this function is only called via calc
            return self.inner.get(pos).unwrap().as_ref().copied();
        }

        // safe to unwrap because we check with is_left before
        let top_left = self.get(&pos.top_left()).unwrap();
        let left = self.get(&pos.left()).unwrap();

        if let Some(tl_left) = top_left.as_ref() {
            if let Some(l_value) = left.as_ref() {
                return Some(*tl_left - *l_value);
            }
        }

        None
    }

    fn calc_from_right(&self, pos: &Position) -> Option<T> {
        if self.is_right(pos) {
            // TODO get rid of unwrap, works for now, because this function is only called via calc
            return self.inner.get(pos).unwrap().as_ref().copied();
        }

        // safe to unwrap because we check with is_right before
        let top_right = self.get(&pos.top_right()).unwrap();
        let right = self.get(&pos.right()).unwrap();

        if let Some(tr_value) = top_right.as_ref() {
            if let Some(r_value) = right.as_ref() {
                return Some(*tr_value - *r_value);
            }
        }

        None
    }
}

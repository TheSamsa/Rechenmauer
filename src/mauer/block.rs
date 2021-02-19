use std::collections::BTreeMap;
use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};

use super::Position;

// this is a newtrait way to alias the trait bounds.
//pub trait Integer: Debug + Display + Copy +  Eq + Add + Sub {}
//impl<T: Debug + Display + Copy +  Eq + Add<Output = T> + Sub<Output = T>> Integer for T {}

#[derive(Debug, Clone)]
pub struct Block<'a, T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> {
    value: Option<&'a T>,
    top_left: Option<Option<&'a T>>,
    top_right: Option<Option<&'a T>>,
    left: Option<Option<&'a T>>,
    right: Option<Option<&'a T>>,
    bottom_left: Option<Option<&'a T>>,
    bottom_right: Option<Option<&'a T>>,
}

impl<'a, T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> Block<'a, T> {
    pub fn new(position: &Position, blocks: &'a BTreeMap<Position, Option<T>>) -> Self {
        let value = blocks.get(position);
        if let Some(value) = value {
            let top_left = blocks
                .get(&(position.0 - 1, position.1 - 1))
                .map(|val| val.as_ref());
            let top_right = blocks
                .get(&(position.0 - 1, position.1))
                .map(|val| val.as_ref());
            let left = blocks
                .get(&(position.0, position.1 - 1))
                .map(|val| val.as_ref());
            let right = blocks
                .get(&(position.0, position.1 + 1))
                .map(|val| val.as_ref());
            let bottom_left = blocks
                .get(&(position.0 + 1, position.1))
                .map(|val| val.as_ref());
            let bottom_right = blocks
                .get(&(position.0 + 1, position.1 + 1))
                .map(|val| val.as_ref());

            return Self {
                value: value.as_ref(),
                top_left,
                top_right,
                left,
                right,
                bottom_left,
                bottom_right,
            };
        }

        Self {
            value: None,
            top_left: None,
            top_right: None,
            left: None,
            right: None,
            bottom_left: None,
            bottom_right: None,
        }
    }

    pub fn get(&self) -> Option<&'a T> {
        self.value
    }

    pub fn is_top(&self) -> bool {
        self.top_left.is_none()
            && self.top_right.is_none()
            && self.left.is_none()
            && self.right.is_none()
    }

    pub fn is_left(&self) -> bool {
        self.top_left.is_none() && self.left.is_none()
    }

    pub fn is_right(&self) -> bool {
        self.top_right.is_none() && self.right.is_none()
    }

    pub fn is_bottom(&self) -> bool {
        self.bottom_left.is_none() && self.bottom_right.is_none()
    }

    pub fn calc(&self) -> Option<T> {
        // skip calculating because we already have a value
        if self.value.is_some() {
            return self.value.copied();
        }

        self.calc_from_bottom()
            .or(self.calc_from_left().or(self.calc_from_right().or(None)))
    }

    fn calc_from_bottom(&self) -> Option<T> {
        if self.is_bottom() {
            return self.get().copied();
        }

        // safe to unwrap because we check with is_bottom before
        let bottom_left = self.bottom_left.unwrap();
        let bottom_right = self.bottom_right.unwrap();

        if let Some(bl_value) = bottom_left {
            if let Some(br_value) = bottom_right {
                return Some(*bl_value + *br_value);
            }
        }

        None
    }

    fn calc_from_left(&self) -> Option<T> {
        if self.is_left() {
            return self.get().copied();
        }

        // safe to unwrap because we check with is_left before
        let top_left = self.top_left.unwrap();
        let left = self.left.unwrap();
        if let Some(tl_left) = top_left {
            if let Some(l_value) = left {
                return Some(*tl_left - *l_value);
            }
        }

        None
    }

    fn calc_from_right(&self) -> Option<T> {
        if self.is_right() {
            return self.get().copied();
        }

        // safe to unwrap because we check with is_right before
        let top_right = self.top_right.unwrap();
        let right = self.right.unwrap();

        if let Some(tr_value) = top_right {
            if let Some(r_value) = right {
                return Some(*tr_value - *r_value);
            }
        }

        None
    }
}

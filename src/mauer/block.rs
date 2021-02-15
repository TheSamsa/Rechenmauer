use std::fmt::{Debug, Display};
use std::ops::{Add, Sub};

// this is a newtrait way to alias the trait bounds.
//pub trait Integer: Debug + Display + Copy +  Eq + Add + Sub {}
//impl<T: Debug + Display + Copy +  Eq + Add<Output = T> + Sub<Output = T>> Integer for T {}

#[derive(Debug, Clone)]
pub struct Block<'a, T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> {
    position: (usize, usize),
    value: Option<T>,
    top_left: Option<&'a Block<'a, T>>,
    top_right: Option<&'a Block<'a, T>>,
    left: Option<&'a Block<'a, T>>,
    right: Option<&'a Block<'a, T>>,
    bottom_left: Option<&'a Block<'a, T>>,
    bottom_right: Option<&'a Block<'a, T>>,
}

impl<'a, T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> Block<'a, T> {
    pub fn new(position: (usize, usize), value: Option<T>) -> Self {
        Self {
            position,
            value,
            top_left: None,
            top_right: None,
            left: None,
            right: None,
            bottom_left: None,
            bottom_right: None,
        }
    }

    pub fn set_adjacent(
        &mut self,
        top_left: Option<&'a Block<T>>,
        top_right: Option<&'a Block<T>>,
        left: Option<&'a Block<T>>,
        right: Option<&'a Block<T>>,
        bottom_left: Option<&'a Block<T>>,
        bottom_right: Option<&'a Block<T>>,
    ) {
        self.top_left = top_left;
        self.top_right = top_right;
        self.left = left;
        self.right = right;
        self.bottom_left = bottom_left;
        self.bottom_right = bottom_right;
    }

    pub fn get(&self) -> Option<T> {
        self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = Some(value)
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
            return self.value;
        }

        self.calc_from_bottom()
            .or(self.calc_from_left().or(self.calc_from_right().or(None)))
    }

    fn calc_from_bottom(&self) -> Option<T> {
        if self.is_bottom() {
            return self.get();
        }

        // safe to unwrap because we check with is_bottom before
        let bottom_left = self.bottom_left.unwrap();
        let bottom_right = self.bottom_right.unwrap();

        bottom_left + bottom_right
    }

    fn calc_from_left(&self) -> Option<T> {
        if self.is_left() {
            return self.get();
        }

        // safe to unwrap because we check with is_left before
        let top_left = self.top_left.unwrap();
        let left = self.left.unwrap();

        top_left - left
    }

    fn calc_from_right(&self) -> Option<T> {
        if self.is_right() {
            return self.get();
        }

        // safe to unwrap because we check with is_right before
        let top_right = self.top_right.unwrap();
        let right = self.right.unwrap();

        top_right - right
    }
}

impl<'a, T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> Add for Block<'a, T> {
    type Output = Option<T>;

    fn add(self, other: Self) -> Option<T> {
        if let Some(self_value) = self.get() {
            if let Some(other_value) = other.get() {
                Some(self_value + other_value)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a, T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> Add
    for &'a Block<'a, T>
{
    type Output = Option<T>;

    fn add(self, other: Self) -> Option<T> {
        if let Some(self_value) = self.get() {
            if let Some(other_value) = other.get() {
                Some(self_value + other_value)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a, T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> Sub for Block<'a, T> {
    type Output = Option<T>;

    fn sub(self, other: Self) -> Option<T> {
        if let Some(self_value) = self.get() {
            if let Some(other_value) = other.get() {
                Some(self_value - other_value)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a, T: Debug + Display + Copy + Eq + Add<Output = T> + Sub<Output = T>> Sub
    for &'a Block<'a, T>
{
    type Output = Option<T>;

    fn sub(self, other: Self) -> Option<T> {
        if let Some(self_value) = self.get() {
            if let Some(other_value) = other.get() {
                Some(self_value - other_value)
            } else {
                None
            }
        } else {
            None
        }
    }
}

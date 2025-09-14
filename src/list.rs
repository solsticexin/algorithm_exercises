///! Sequential list module providing SqList implementation.
///
/// This module defines a sequential list using a fixed-size array.

pub mod sq_list;

/// Maximum capacity of the SqList.
const MAX_SIZE: usize = 50;

/// Sequential list structure implemented with a fixed-size array.
///
/// Supports generic type T where T is PartialOrd, Copy, and Default.
pub struct SqList<T> {
    data: [T; MAX_SIZE],
    len: usize,
}

impl<T> SqList<T>
where
    T: PartialOrd + Copy + Default,
{
    /// Creates a new empty SqList.
    ///
    /// Initializes with default values and length 0.
    pub fn new() -> Self {
        Self {
            data: [T::default(); MAX_SIZE],
            len: 0,
        }
    }

    /// Inserts an element at the specified position.
    ///
    /// Position i starts from 1. Returns true if successful, false if invalid position or full.
    pub fn insert(&mut self, i: usize, e: T) -> bool {
        if i == 0 || i > self.len + 1 {
            return false;
        }
        if self.len >= MAX_SIZE {
            return false;
        }
        for j in (i..=self.len).rev() {
            self.data[j] = self.data[j - 1];
        }
        self.data[i - 1] = e;
        self.len = self.len + 1;
        true
    }

    /// Deletes the element at the specified position.
    ///
    /// Position i starts from 1. Returns the deleted element if successful, None otherwise.
    pub fn delete(&mut self, i: usize) -> Option<T> {
        if i == 0 || i > self.len {
            return None;
        }
        let result = Some(self.data[i - 1]);
        for j in i..self.len {
            self.data[j - 1] = self.data[j];
        }
        self.len = self.len - 1;
        result
    }

    /// Locates the position of the element.
    ///
    /// Returns the 1-based position if found, 0 otherwise.
    pub fn locate_elem(&self, elem: T) -> usize {
        for i in 0..self.len {
            if self.data[i] == elem {
                return i + 1;
            }
        }
        0
    }
}

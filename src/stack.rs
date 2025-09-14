///! Stack module providing Stack implementation and utility functions.
///
/// This module implements a dynamic stack using Vec, along with functions for
/// parenthesis checking, binary conversion, and base conversion.
use std::fmt::Debug; // Assuming Debug for potential use, but not strictly needed

/// Stack structure implemented with a dynamic Vec.
///
/// Supports generic type T.
#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new empty Stack.
    ///
    /// Initializes with size 0 and empty Vec.
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    /// Checks if the stack is empty.
    ///
    /// Returns true if size is 0.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the current size of the stack.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Clears the stack.
    ///
    /// Resets size to 0 and clears the underlying Vec.
    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    /// Pushes a value onto the top of the stack.
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    /// Pops and returns the top value from the stack.
    ///
    /// Returns None if the stack is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let del = self.data.pop();
        self.size -= 1;
        del
    }

    /// Returns a reference to the top value without removing it.
    ///
    /// Returns None if the stack is empty.
    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        self.data.get(self.size - 1)
    }

    /// Returns a mutable reference to the top value without removing it.
    ///
    /// Returns None if the stack is empty.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            return None;
        }
        Some(&mut self.data[self.size - 1])
    }
}

/// Checks if two parentheses characters match.
///
/// Returns true if open and close form a valid pair.
fn par_match(open: char, close: char) -> bool {
    let opens = "([{"; // Fixed string without ;
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

/// Checks if parentheses in a string are balanced and correctly matched.
///
/// Uses a stack to validate nesting. Returns true if balanced.
pub fn par_checker(par: &str) -> bool {
    let mut char_list = Vec::new();
    for char_ in par.chars() {
        char_list.push(char_);
    }
    let mut index = 0;
    let mut balance = true;
    let mut stack: Stack<char> = Stack::new();
    while index < char_list.len() && balance {
        let c = char_list[index];

        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
        }
        if c == ')' || c == ']' || c == '}' {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }
        index += 1;
    }
    balance && stack.is_empty()
}

/// Converts a decimal number to binary string using stack.
///
/// Pops remainders from stack to build the binary representation.
pub fn divide_by_two(mut dec_num: u32) -> String {
    let mut rem_stack = Stack::new();
    while dec_num > 0 {
        let rem = dec_num % 2;
        rem_stack.push(rem);
        dec_num /= 2;
    }
    let mut bin_str = String::new();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap().to_string();
        bin_str += &rem;
    }
    bin_str
}

/// Converts a decimal number to a string in the given base (2-16).
///
/// Uses digits 0-9, A-F for bases >10. Builds result by popping from stack.
pub fn base_converter(mut dec_num: u32, base: u32) -> String {
    let digits: [char; 16] = [
        '0', '1', '2', '3', '4', '5', '6', '7',
        '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let mut rem_stack = Stack::new();
    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }
    let mut base_str = String::new();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap() as usize;
        base_str.push(digits[rem]);
    }
    base_str
}

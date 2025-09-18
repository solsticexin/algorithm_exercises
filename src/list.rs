///! 顺序表模块提供SqList实现
///
/// 该模块使用固定大小的数组定义顺序表。
///
/// 实现思想：使用固定大小的数组存储元素，维护长度字段跟踪当前元素数量，
/// 提供基于数组的顺序表操作接口。

pub mod sq_list;

/// Maximum capacity of the SqList.
const MAX_SIZE: usize = 50;

/// 使用固定大小数组实现的顺序表结构
///
/// 支持泛型类型T，其中T需要实现PartialOrd、Copy和Default trait。
///
/// 实现思想：使用固定大小的数组存储数据，维护长度字段len跟踪有效元素数量，
/// 提供基于数组的插入、删除、查找等操作。
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

//! 顺序表（数组/Vec）相关的小工具函数
//!
//! 该模块提供了一个原地删除最小元素的简单算法：把最后一个元素移动到最小元素的位置，
//! 然后弹出最后一个元素。时间复杂度 O(n)，额外空间 O(1)。

use std::fmt::Debug;

/// 从可变 `Vec` 中删除最小值：用最后一个元素覆盖最小值位置并弹出最后一个元素。
///
/// 行为要点：
/// - 如果向量为空，什么也不做。
/// - 如果向量只有一个元素，函数会把它弹出，结果为空向量。
/// - 当最小值存在重复时，函数只删除第一个遇到的最小值（最小索引最小的那个）。
///
/// # 示例
///
/// ```
/// let mut v = vec![3, 1, 4, 2];
/// // 调用位于库的路径：algorithm_exercises::list::sq_list::remove_min_and_move_last
/// algorithm_exercises::list::sq_list::remove_min_and_move_last(&mut v);
/// assert_eq!(v.len(), 3);
/// assert!(!v.contains(&1));
/// ```
pub fn remove_min_and_move_last<T>(vec: &mut Vec<T>)
where
    T: PartialOrd + Copy + Debug,
{
    if vec.is_empty() {
        return;
    }

    let mut min_index = 0usize;
    for i in 1..vec.len() {
        if vec[i] < vec[min_index] {
            min_index = i;
        }
    }

    // 将最后一个元素移动到最小元素的位置（如果有必要），然后弹出最后一个元素
    if let Some(last_element) = vec.pop() {
        if min_index < vec.len() {
            vec[min_index] = last_element;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut v: Vec<i32> = vec![];
        remove_min_and_move_last(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn test_single() {
        let mut v = vec![42i32];
        remove_min_and_move_last(&mut v);
        assert!(v.is_empty());
    }

    #[test]
    fn test_unique_min() {
        let mut v = vec![5, 2, 7, 3]; // min = 2 at index 1, last = 3
        remove_min_and_move_last(&mut v);
        assert_eq!(v.len(), 3);
        // 最小值 2 应该被移除
        assert!(!v.contains(&2));
        // 被移入的位置应为原先的最小值位置（index 1）
        assert_eq!(v[1], 3);
    }

    #[test]
    fn test_duplicate_min() {
        let mut v = vec![6, 3, 9, 3, 8]; // 两个 3，最小索引为 1
        remove_min_and_move_last(&mut v);
        assert_eq!(v.len(), 4);
        // 仍然会有一个 3（因为原来有两个），但至少删除了一个
        let count_3 = v.iter().filter(|&&x| x == 3).count();
        assert_eq!(count_3, 1);
    }
}
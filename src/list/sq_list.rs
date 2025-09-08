//! 顺序表（数组/Vec）相关的小工具函数
//!
//! 该模块提供了一个原地删除最小元素的简单算法：把最后一个元素移动到最小元素的位置，
//! 然后弹出最后一个元素。时间复杂度 O(n)，额外空间 O(1)。

use std::{fmt::Debug};

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

pub fn reverse<T>(list:&mut Vec<T>)
where T:Copy+Debug
{
    for i in 0..list.len()/2 {
        let j=list.len()-i-1;
        list.swap(i, j);
    }
}

pub fn del_x<T>(list:&mut Vec<T>,x:T)
where T:PartialEq
{
    list.retain(|elem| *elem!=x);
}

pub fn del_x_t<T>(list:&mut Vec<T>,s:T,t:T)->bool
where T:PartialOrd
{
 if s>=t{
    return false
 }
 list.retain(|elem| *elem<s || *elem >t);
 true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reverse(){
        let mut val=vec![1,2,3,4,5,6];
        reverse(&mut val);
        assert_eq!(val,vec![6,5,4,3,2,1]);
    }
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
    
    #[test]
     fn test_del_x_t() {
        // 测试正常情况
        let mut vec1 = vec![1, 2, 3, 4, 5];
        assert_eq!(del_x_t(&mut vec1, 2, 4), true);
        assert_eq!(vec1, vec![1, 5]);

        // 测试边界值
        let mut vec2 = vec![1, 2, 3, 4, 5];
        assert_eq!(del_x_t(&mut vec2, 1, 5), true);
        assert_eq!(vec2, vec![]);

        // 测试空向量
        let mut vec3: Vec<i32> = vec![];
        assert_eq!(del_x_t(&mut vec3, 2, 4), true);
        assert_eq!(vec3, vec![]);

        // 测试s>=t的情况
        let mut vec4 = vec![1, 2, 3, 4, 5];
        assert_eq!(del_x_t(&mut vec4, 4, 2), false);
        assert_eq!(vec4, vec![1, 2, 3, 4, 5]);

        // 测试浮点数
        let mut vec5 = vec![1.0, 2.5, 3.5, 4.0, 5.0];
        assert_eq!(del_x_t(&mut vec5, 2.0, 4.0), true);
        assert_eq!(vec5, vec![1.0, 5.0]);

        // 测试所有元素都在范围内
        let mut vec6 = vec![2, 3, 4];
        assert_eq!(del_x_t(&mut vec6, 1, 5), true);
        assert_eq!(vec6, vec![]);

        // 测试所有元素都不在范围内
        let mut vec7 = vec![1, 5];
        assert_eq!(del_x_t(&mut vec7, 2, 4), true);
        assert_eq!(vec7, vec![1, 5]);

        // 测试重复元素
        let mut vec8 = vec![1, 2, 2, 3, 3, 3, 4, 5];
        assert_eq!(del_x_t(&mut vec8, 2, 4), true);
        assert_eq!(vec8, vec![1, 5]);
    }
}
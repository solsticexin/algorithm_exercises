///! 栈模块提供栈实现和实用函数
///
/// 该模块使用Vec实现动态栈，包含括号检查、二进制转换和进制转换等功能。
///
/// 实现思想：使用Rust的Vec作为底层存储，提供O(1)时间复杂度的push/pop操作，
/// 并通过栈结构解决括号匹配、进制转换等经典算法问题。
use std::fmt::Debug; // Assuming Debug for potential use, but not strictly needed

/// 使用动态Vec实现的栈结构
///
/// 支持泛型类型T。
///
/// 实现思想：使用Vec作为底层存储，维护size字段跟踪当前元素数量，
/// 提供类型安全的栈操作接口。
#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    /// 创建一个新的空栈
    ///
    /// 初始化size为0，使用空的Vec。
    ///
    /// 实现思想：使用Vec::new()创建空向量，size初始化为0。
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    /// 检查栈是否为空
    ///
    /// 如果size为0则返回true。
    ///
    /// 实现思想：通过检查size字段是否为0来判断栈是否为空，时间复杂度O(1)。
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// 返回栈的当前大小
    ///
    /// 实现思想：直接返回size字段的值，时间复杂度O(1)。
    pub fn len(&self) -> usize {
        self.size
    }

    /// 清空栈
    ///
    /// 将size重置为0并清空底层Vec。
    ///
    /// 实现思想：调用Vec::clear()方法清空向量，同时将size设为0。
    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    /// 将值压入栈顶
    ///
    /// 实现思想：使用Vec::push()方法将值添加到向量末尾，同时递增size计数器。
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    /// 弹出并返回栈顶的值
    ///
    /// 如果栈为空则返回None。
    ///
    /// 实现思想：使用Vec::pop()方法移除并返回最后一个元素，同时递减size计数器。
    pub fn pop(&mut self) -> Option<T> {
        if self.size == 0 {
            return None;
        }
        let del = self.data.pop();
        self.size -= 1;
        del
    }

    /// 返回栈顶值的引用而不移除它
    ///
    /// 如果栈为空则返回None。
    ///
    /// 实现思想：使用Vec::get()方法获取最后一个元素的引用，时间复杂度O(1)。
    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }
        self.data.get(self.size - 1)
    }

    /// 返回栈顶值的可变引用而不移除它
    ///
    /// 如果栈为空则返回None。
    ///
    /// 实现思想：通过索引直接访问最后一个元素的可变引用，时间复杂度O(1)。
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            return None;
        }
        Some(&mut self.data[self.size - 1])
    }
}

/// 检查两个括号字符是否匹配
///
/// 如果开括号和闭括号形成有效对则返回true。
///
/// 实现思想：通过预定义的括号字符串查找字符位置，比较位置是否相同来判断是否匹配。
fn par_match(open: char, close: char) -> bool {
    let opens = "([{"; // Fixed string without ;
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

/// 检查字符串中的括号是否平衡且正确匹配
///
/// 使用栈验证嵌套。如果平衡则返回true。
///
/// 实现思想：遍历字符串字符，遇到开括号压栈，遇到闭括号时弹出栈顶元素检查是否匹配，
/// 最终检查栈是否为空来判断括号是否完全匹配。
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

/// 使用栈将十进制数转换为二进制字符串
///
/// 从栈中弹出余数来构建二进制表示。
///
/// 实现思想：通过不断除以2获取余数并压栈，然后依次弹出余数构建二进制字符串，
/// 实现十进制到二进制的转换。
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

use std::collections::HashMap;

use crate::utils::token;

fn get_precedence(op: &str) -> i32 {
    let mut precedence = HashMap::new();
    precedence.insert("+", 1);
    precedence.insert("-", 1);
    precedence.insert("*", 2);
    precedence.insert("/", 2);
    *precedence.get(op).unwrap_or(&0)
}

pub fn infix_to_postfix(infix:&str)->String{
    let tokens=token(infix);
    let mut postfix=String::new();
    let mut op_stack:Vec<String>=Vec::new();
    for token in tokens {
        match token.as_str() {
            "(" => op_stack.push(token),
            ")" => {
                while let Some(top) = op_stack.last() {
                    if *top == "(" {
                        break;
                    }
                    postfix.push_str(&op_stack.pop().unwrap());
                    postfix.push(' '); // 在 token 之间添加空格
                }
                op_stack.pop(); // 弹出 '('
            }
            "+" | "-" | "*" | "/" => {
                while let Some(top) = op_stack.last() {
                    if *top == "(" || get_precedence(top) < get_precedence(&token) {
                        break;
                    }
                    postfix.push_str(&op_stack.pop().unwrap());
                    postfix.push(' ');
                }
                op_stack.push(token);
            }
            _ => {
                // 假设是操作数（数字或变量）
                postfix.push_str(&token);
                postfix.push(' ');
            }
        }
    }

    // 将栈中剩余的运算符全部弹出
    while let Some(op) = op_stack.pop() {
        postfix.push_str(&op);
        if !op_stack.is_empty() {
            postfix.push(' ');
        }
    }
    // 去除末尾多余的空格
    postfix.trim().to_string()
}

//计算后缀表达式
pub fn evaluate_postfix(postfix: &str) -> Option<f64> {
    let tokens: Vec<&str> = postfix.split_whitespace().collect();
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            // 如果 token 是运算符
            "+" | "-" | "*" | "/" => {
                let right = stack.pop()?;
                let left = stack.pop()?;

                let result = match token {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => {
                        if right == 0.0 {
                            eprintln!("Error: Division by zero");
                            return None;
                        }
                        left / right
                    },
                    _ => unreachable!(),
                };
                stack.push(result);
            },
            // 如果 token 是数字
            _ => {
                let number = token.parse::<f64>().ok()?;
                stack.push(number);
            }
        }
    }

    // 栈中应只剩一个结果
    if stack.len() == 1 {
        stack.pop()
    } else {
        eprintln!("Error: Invalid postfix expression");
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infix_to_postfix_basic() {
        assert_eq!(infix_to_postfix("1 + 2"), "1 2 +");
        assert_eq!(infix_to_postfix("1 + 2 * 3"), "1 2 3 * +");
        assert_eq!(infix_to_postfix("(1 + 2) * 3"), "1 2 + 3 *");
        assert_eq!(infix_to_postfix("1 * 2 + 3"), "1 2 * 3 +");
    }

    #[test]
    fn test_infix_to_postfix_complex() {
        assert_eq!(infix_to_postfix("(1 + 2) * (3 + 4)"), "1 2 + 3 4 + *");
        assert_eq!(infix_to_postfix("1 + 2 * 3 - 4 / 5"), "1 2 3 * + 4 5 / -");
        assert_eq!(infix_to_postfix("(1 + (2 * 3)) - 4"), "1 2 3 * + 4 -");
    }

    #[test]
    fn test_evaluate_postfix_basic() {
        assert_eq!(evaluate_postfix("1 2 +"), Some(3.0));
        assert_eq!(evaluate_postfix("1 2 3 * +"), Some(7.0));
        assert_eq!(evaluate_postfix("1 2 + 3 *"), Some(9.0));
        assert_eq!(evaluate_postfix("1 2 * 3 +"), Some(5.0));
    }

    #[test]
    fn test_evaluate_postfix_complex() {
        assert_eq!(evaluate_postfix("1 2 + 3 4 + *"), Some(21.0));
        assert_eq!(evaluate_postfix("1 2 3 * + 4 5 / -"), Some(6.2));
        assert_eq!(evaluate_postfix("1 2 3 * + 4 -"), Some(3.0));
    }

    #[test]
    fn test_evaluate_postfix_division_by_zero() {
        assert_eq!(evaluate_postfix("1 0 /"), None);
        assert_eq!(evaluate_postfix("5 2 - 0 /"), None);
    }

    #[test]
    fn test_evaluate_postfix_invalid_expressions() {
        assert_eq!(evaluate_postfix("1 2 + 3"), None); // 多余的操作数
        assert_eq!(evaluate_postfix("1 +"), None); // 操作符不足
        assert_eq!(evaluate_postfix("1 2"), None); // 缺少操作符
    }

    #[test]
    fn test_integration() {
        // 测试中缀转后缀再求值的完整流程
        let infix = "(1 + 2) * (3 + 4)";
        let postfix = infix_to_postfix(infix);
        assert_eq!(postfix, "1 2 + 3 4 + *");
        assert_eq!(evaluate_postfix(&postfix), Some(21.0));
    }

    #[test]
    fn test_decimal_numbers() {
        assert_eq!(infix_to_postfix("1.5 + 2.3"), "1.5 2.3 +");
        assert_eq!(evaluate_postfix("1.5 2.3 +"), Some(3.8));
        assert_eq!(evaluate_postfix("3.0 1.5 *"), Some(4.5));
    }

    #[test]
    fn test_negative_numbers() {
        // 测试负数的中缀到后缀转换
        assert_eq!(infix_to_postfix("-1 + 2"), "-1 2 +");
        assert_eq!(infix_to_postfix("1 + -2"), "1 -2 +");
        assert_eq!(infix_to_postfix("-1 * -2"), "-1 -2 *");
        assert_eq!(infix_to_postfix("(-1 + 2) * 3"), "-1 2 + 3 *");
        
        // 测试负数的后缀表达式求值
        assert_eq!(evaluate_postfix("-1 2 +"), Some(1.0));
        assert_eq!(evaluate_postfix("1 -2 +"), Some(-1.0));
        assert_eq!(evaluate_postfix("-1 -2 *"), Some(2.0));
        assert_eq!(evaluate_postfix("-5 2 /"), Some(-2.5));
        assert_eq!(evaluate_postfix("10 -3 *"), Some(-30.0));
    }

    #[test]
    fn test_negative_decimal_numbers() {
        // 测试负小数的综合运算
        assert_eq!(infix_to_postfix("-1.5 + 2.3"), "-1.5 2.3 +");
        
        // 使用浮点数近似比较来处理精度问题
        assert!((evaluate_postfix("-1.5 2.3 +").unwrap() - 0.8).abs() < 1e-10);
        assert_eq!(evaluate_postfix("-3.0 -1.5 *"), Some(4.5));
        assert_eq!(evaluate_postfix("-4.5 1.5 /"), Some(-3.0));
    }
}

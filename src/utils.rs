//分离字符串，正确处理整数、小数、运算符和括号的实现，它将数字和运算符正确地分离开来。
// 支持负数：负号在表达式开头或运算符/括号后时作为数字的一部分
pub fn token(s: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        // 如果字符是数字或小数点，添加到当前 token
        if c.is_ascii_digit() || c == '.' {
            current_token.push(c);
        }
        // 处理负号
        else if c == '-' {
            // 检查是否是负数：表达式开头，或者前一个 token 是运算符或左括号
            let is_negative = current_token.is_empty() && 
                (tokens.is_empty() || 
                 matches!(tokens.last().map(|s: &String| s.as_str()), Some("(" | "+" | "-" | "*" | "/")));
            
            if is_negative {
                current_token.push(c); // 负号作为数字的一部分
            } else {
                // 先处理可能存在的上一个数字 token
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
                // 将减号作为运算符 token
                tokens.push(c.to_string());
            }
        }
        // 如果字符是其他运算符或括号
        else if "+*/()".contains(c) {
            // 先处理可能存在的上一个数字 token
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
            // 将当前运算符或括号作为独立的 token
            tokens.push(c.to_string());
        } else if c.is_whitespace() {
            // 遇到空格，将当前 token 推入结果并清空
            if !current_token.is_empty() {
                tokens.push(current_token.clone());
                current_token.clear();
            }
        } else {
            // 其他字符（如字母等）也作为 token 的一部分
            current_token.push(c);
        }
    }

    // 处理循环结束时可能残留的最后一个 token
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

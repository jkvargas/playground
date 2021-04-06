struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        Self::backtrack(&mut result, "".to_string(), 0, 0, n);
        result
    }

    fn backtrack(list: &mut Vec<String>, cur: String, open: i32, close: i32, max: i32) {
        if cur.len() == (max * 2) as usize {
            list.push(cur);
            return;
        }

        if open < max {
            Self::backtrack(list, format!("{}(", cur), open + 1, close, max);
        }

        if close < open {
            Self::backtrack(list, format!("{})", cur), open, close + 1, max);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn generate_parenthesis_1() {
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                "((()))".to_string(),
                "(()())".to_string(),
                "(())()".to_string(),
                "()(())".to_string(),
                "()()()".to_string()
            ]
        )
    }
}

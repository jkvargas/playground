struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let word : Vec<char> = s.chars().collect();

    }

    fn lp(w: &Vec<char>, mut pos: usize, mut open: bool) -> i32 {
        if !open {
            while w[pos] == ')' {
                if pos == w.len() {
                    return;
                }

                pos += 1;
            }
        } else {
            if w[pos] == ')' {
                return 1;
            }
            if pos == '(' {
                Self::lp(w, pos + 1, true);
            }
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_valid_parentheses_1() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
    }

    #[test]
    fn longest_valid_parentheses_2() {
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
    }

    #[test]
    fn longest_valid_parentheses_3() {
        assert_eq!(Solution::longest_valid_parentheses("))()()))(()()())".to_string()), 8);
    }

    #[test]
    fn longest_valid_parentheses_4() {
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
    }
}
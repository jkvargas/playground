// https://leetcode.com/problems/valid-palindrome-iv/?envType=study-plan-v2&envId=amazon-spring-23-high-frequency

use log::error;

struct Solution;

impl Solution {
    pub fn make_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }

        let letters = s.chars().collect::<Vec<char>>();
        let mut ops = 2;

        let mut beg = 0;
        let mut end = letters.len() as i32 - 1;

        loop {
            if beg > end {
                return true;
            }

            if letters[beg as usize] == letters[end as usize] {
                beg += 1;
                end -= 1;
            } else {
                if ops > 0 {
                    beg += 1;
                    end -= 1;
                    ops -= 1;
                } else {
                    return false;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::palindrome_iv::Solution;

    #[test]
    fn test_one() {
        assert!(Solution::make_palindrome("abcdba".to_string()));
    }

    #[test]
    fn test_two() {
        assert_eq!(false, Solution::make_palindrome("abcdef".to_string()));
    }

    #[test]
    fn test_three() {
        assert!(Solution::make_palindrome("k".to_string()));
    }
}

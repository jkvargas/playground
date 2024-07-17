// https://leetcode.com/problems/palindrome-partitioning/description/

use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        calc(
            &mut result,
            s.chars().collect::<Vec<char>>().as_slice(),
            0,
            &mut vec![],
        );

        result
    }
}

fn calc(result: &mut Vec<Vec<String>>, s: &[char], start: usize, current_list: &mut Vec<String>) {
    if start >= s.len() {
        result.push(current_list.clone());
        return;
    }

    for end in start..s.len() {
        if is_palindrome(s, start, end) {
            current_list.push(s[start..(end + 1)].iter().collect());
            calc(result, s, end + 1, current_list);
            current_list.pop();
        }
    }
}

fn is_palindrome(text: &[char], mut low: usize, mut high: usize) -> bool {
    while low < high {
        if text[low] != text[high] {
            return false;
        }

        low += 1;
        high -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::palindrome_partitioning::Solution;

    #[test]
    fn test_one() {
        let compare_to = vec![
            vec!["a".to_string(), "a".to_string(), "b".to_string()],
            vec!["aa".to_string(), "b".to_string()],
        ];
        let result = Solution::partition("aab".to_string());
        assert_eq!(compare_to, result);
    }
}

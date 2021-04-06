use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut max_count = 0;
        let letters = s.chars().collect::<Vec<char>>();
        let mut j = 0;

        for i in 0..letters.len() {
            if map.contains_key(&letters[i]) {
                j = std::cmp::max(map[&letters[i]], j)
            }

            max_count = std::cmp::max(max_count, i - j + 1);
            map.insert(letters[i], i + 1);
        }

        max_count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_of_longest_substring_1() {
        let result = Solution::length_of_longest_substring("abcabcbb".to_string());

        assert_eq!(3, result);
    }

    #[test]
    fn length_of_longest_substring_2() {
        let result = Solution::length_of_longest_substring("bbbbb".to_string());

        assert_eq!(1, result);
    }
}

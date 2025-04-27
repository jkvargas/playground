// https://leetcode.com/explore/interview/card/facebook/5/array-and-strings/3017/

use std::cmp::max;
use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        if k == 0 { return 0; }
        let letters = s.chars().collect::<Vec<char>>();
        
        let mut a = 0;
        let mut b = 0;
        let mut cur_max = 0;
        let mut max_over = 0;
        let mut set = HashMap::new();
        
        
        while b < letters.len() {
            if set.contains_key(&letters[b]) || set.len() < k as usize {
                set.entry(letters[b]).or_insert(HashSet::new()).insert(b);
                b += 1;
                cur_max += 1;
                max_over = max(max_over, cur_max);
            } else {
                let mut temp = set.get_mut(&letters[a]).unwrap();
                temp.remove(&a);
                if temp.is_empty() { set.remove(&letters[a]); }
                cur_max -= 1;
                a += 1;
            }
        }
        
        max_over
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_one() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring_k_distinct("eceba".to_string(), 2)
        );
    }

    #[test]
    pub fn two() {
        assert_eq!(
            1,
            Solution::length_of_longest_substring_k_distinct("aba".to_string(), 1)
        );
    }

    #[test]
    pub fn three() {
        assert_eq!(
            4,
            Solution::length_of_longest_substring_k_distinct("abaccc".to_string(), 2)
        );
    }
}

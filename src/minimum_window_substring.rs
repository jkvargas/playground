// https://leetcode.com/explore/interview/card/facebook/5/array-and-strings/285/

use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let letters = s.chars().collect::<Vec<char>>();
        let map = t.chars().collect::<HashSet<char>>();
        let mut resulting_map = HashMap::new();
        
        for i in 0..letters.len() {
            if map.contains(&letters[i]) {
                resulting_map.entry(letters[i]).or_insert(Vec::new()).push(i);
            }
        }
        
        
        
        "oo".to_string()
    }
}


// "ADOBECODEBANC", t = "ABC"

A - 0, 10,
B - 3, 9,
C - 5, 12
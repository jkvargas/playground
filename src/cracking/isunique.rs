use std::collections::HashMap;

struct Solution;

impl Solution {
    // 1.1 PG 90
    pub fn is_unique(word: String) -> bool{
        let mut map : HashMap<char, bool> = HashMap::new();

        for i in word.chars() {
            if map.contains_key(&i) {
                return false;
            } else {
                map.insert(i, false);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn is_unique_1() {
        assert_eq!(Solution::is_unique("jhonny".to_string()), false);
    }

    #[test]
    pub fn is_unique_2() {
        assert_eq!(Solution::is_unique("abcdefg".to_string()), true);
    }
}
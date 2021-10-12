use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        let mut map = HashMap::new();
        let nums : Vec<char> = num.chars().collect();

        map.insert('1', '1');
        map.insert('0', '0');
        map.insert('8', '8');
        map.insert('6', '9');
        map.insert('9', '6');

        for i in 0..nums.len() {
            let l = nums[i];
            let r = nums[nums.len() - 1 - i];

            if let Some(mr) = map.get(&r) {
                if l != *mr {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::strobogrammaticnumber::Solution;

    #[test]
    fn test_one() {
        assert!(Solution::is_strobogrammatic("69".to_string()));
    }

    #[test]
    fn test_two() {
        assert!(Solution::is_strobogrammatic("88".to_string()));
    }

    #[test]
    fn test_three() {
        assert!(!Solution::is_strobogrammatic("962".to_string()));
    }
}
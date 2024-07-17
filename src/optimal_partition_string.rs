use std::collections::HashMap;

//https://leetcode.com/problems/optimal-partition-of-string/description/?envType=study-plan-v2&envId=amazon-spring-23-high-frequency
struct Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut res = 1;
        let mut mask = 0;
        for c in s.chars() {
            let cur = 1 << (c as u8 - b'a');
            if (mask & cur) == cur {
                res += 1;
                mask = 0;
            }
            mask |= cur;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::optimal_partition_string::Solution;

    #[test]
    fn test_one() {
        let temp = Solution::partition_string("abacaba".to_string());

        assert!(temp > 0);
    }
}

// https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/?envType=study-plan-v2&envId=amazon-spring-23-high-frequency

use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {

    }
}

#[cfg(test)]
mod tests {
    use crate::max_sum_distinct_subarrays::Solution;

    #[test]
    fn test_one() {
        assert_eq!(15, Solution::maximum_subarray_sum(vec![1,5,4,2,9,9,9], 3));
    }

    #[test]
    fn test_two() {
        assert_eq!(12, Solution::maximum_subarray_sum(vec![9,9,9,1,2,3], 3));
    }

    #[test]
    fn test_three() {
        assert_eq!(0, Solution::maximum_subarray_sum(vec![5,3,3,1,1], 3));
    }
}
// https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/?envType=study-plan-v2&envId=amazon-spring-23-high-frequency

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut beg = 0;
        let mut end = 0;
        let mut set = HashMap::new();
        let mut sum = 0;
        let mut max = 0;

        loop {
            if end - beg < k as usize {
                *set.entry(&nums[end]).or_insert(0) += 1;
                sum += nums[end] as i64;
                end += 1;
            } else {
                set.entry(&nums[beg]).and_modify(|x| *x -= 1);
                if let Some(from_get) = set.get(&nums[beg]) {
                    if *from_get < 1 {
                        set.remove(&nums[beg]);
                    }
                }
                sum -= nums[beg] as i64;
                beg += 1;

                *set.entry(&nums[end]).or_insert(0) += 1;
                sum += nums[end] as i64;
                end += 1;
            }

            if set.len() == k as usize {
                max = max.max(sum);
            }

            if end == nums.len() {
                break;
            }
        }

        max
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

    #[test]
    fn test_four() {
        assert_eq!(21, Solution::maximum_subarray_sum(vec![14,7,7,7,12,7], 2));
    }
}
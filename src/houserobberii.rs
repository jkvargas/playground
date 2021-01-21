use std::cmp::max;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        if nums.len() == 1 {
            return nums[0];
        }

        let max1 = Self::rob_simple(&nums, 0, nums.len() - 2);
        let max2 = Self::rob_simple(&nums, 1, nums.len() - 1);

        max(max1, max2)
    }

    fn rob_simple(nums: &Vec<i32>, start: usize, end: usize) -> i32 {
        let mut t1 = 0;
        let mut t2 = 0;

        for i in start..end + 1 {
            let temp = t1;
            let current = nums[i];
            t1 = max(current + t2, t1);
            t2 = temp;
        }

        t1
    }

    pub fn rob_leet(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut dp = (0, 0);

        for n in nums.iter().take(nums.len() - 1) {
            dp = (dp.1, std::cmp::max(dp.0 + n, dp.1));
        }
        let max = std::cmp::max(dp.0, dp.1);
        dp = (0, 0);
        for n in nums.iter().skip(1) {
            dp = (dp.1, std::cmp::max(dp.0 + n, dp.1));
        }
        std::cmp::max(max, std::cmp::max(dp.0, dp.1))
    }
}

#[test]
pub fn example1() {
    assert_eq!(Solution::rob(vec![7, 4, 1, 9, 3, 8, 6]), 24);
}

#[test]
pub fn example2() {
    assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
}

#[test]
pub fn example3() {
    assert_eq!(Solution::rob(vec![200, 3, 140, 20, 10]), 340);
}

#[test]
pub fn example4() {
    assert_eq!(Solution::rob(vec![1]), 1);
}

#[test]
pub fn example5() {
    assert_eq!(Solution::rob(vec![1, 2]), 2);
}

#[test]
pub fn example6() {
    assert_eq!(Solution::rob(vec![1, 2, 1, 1]), 3);
}

#[test]
pub fn example7() {
    assert_eq!(Solution::rob(vec![1, 1, 1, 2]), 3);
}

use std::cmp::min;

struct Solution;

impl Solution {
    // O(k*nums^2)
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let mut dp = vec![vec![-1.0; nums.len() + 1]; k as usize + 1];

        Solution::f(&nums, k as usize, 0, &mut dp)
    }

    fn f(nums: &Vec<i32>, k: usize, start: usize, dp: &mut Vec<Vec<f64>>) -> f64 {
        if k <= 0 {
            return 0.0;
        }

        if dp[k][start] != -1.0 {
            return dp[k][start];
        }

        let mut res = 0.0;
        let mut sum = 0.0;

        for i in start..nums.len() {
            sum += nums[i] as f64;

            if k != 1 || i == nums.len() - 1 {
                let component = sum / (i - start + 1) as f64 + Solution::f(nums, k - 1, i + 1, dp);
                res = if res > component { res } else { component };
            }
        }

        dp[k][start] = res;
        dp[k][start]
    }
}

#[test]
fn test_one() {
    assert_eq!(
        Solution::largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3),
        20.0
    );
}

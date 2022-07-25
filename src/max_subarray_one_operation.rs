struct Solution;

impl Solution {
    pub fn max_sum_after_operation(nums: Vec<i32>) -> i32 {
        let mut dp = vec![vec![i32::MIN; 2]; nums.len()];

        dp[0][0] = nums[0];
        dp[0][1] = nums[0] * nums[0];

        let mut ans = std::cmp::max(dp[0][0], dp[0][1]);

        for i in 1..nums.len() {
            dp[i][0] = std::cmp::max(nums[i] + dp[i - 1][0], nums[i]);
            let squared = nums[i] * nums[i];
            dp[i][1] = std::cmp::max(
                std::cmp::max(squared, squared + dp[i - 1][0]),
                nums[i] + dp[i - 1][1],
            );
            ans = std::cmp::max(ans, std::cmp::max(dp[i][0], dp[i][1]));
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::max_subarray_one_operation::Solution;

    #[test]
    fn test_one() {
        let vec = vec![2, -1, -4, -3];
        assert_eq!(17, Solution::max_sum_after_operation(vec));
    }

    #[test]
    fn test_two() {
        let vec = vec![1, -1, 1, 1, -1, -1, 1];
        assert_eq!(4, Solution::max_sum_after_operation(vec));
    }
}

struct Solution;

// https://leetcode.com/problems/merge-operations-to-turn-array-into-a-palindrome/description/?envType=study-plan-v2&envId=amazon-spring-23-high-frequency

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = nums.len() - 1;
        let mut left_sum = nums[start];
        let mut right_sum = nums[end];
        let mut operations = 0;

        while start < end {
            if left_sum == right_sum {
                start += 1;
                end -= 1;

                left_sum = nums[start];
                right_sum = nums[end];
            } else if left_sum < right_sum {
                operations += 1;
                start += 1;
                left_sum += nums[start];
            } else {
                operations += 1;
                end -= 1;
                right_sum += nums[end];
            }
        }

        operations
    }
}

#[cfg(test)]
mod test {
    use crate::merge_operations_palindrome::Solution;

    #[test]
    fn test_one() {
        let result = Solution::minimum_operations(vec![4,3,2,1,2,3,1]);
        assert_eq!(2, result);
    }
}
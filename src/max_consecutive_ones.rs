// https://leetcode.com/problems/max-consecutive-ones-ii/

use std::cmp::max;
use std::convert::TryInto;

struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_global = 0;
        let mut max_iter = 0;
        let mut sum_so_far = Vec::new();

        for num in &nums {
            if *num == 1 {
                max_iter += 1;

                for i in &mut sum_so_far {
                    *i += 1;

                    if *i > max_global {
                        max_global = *i;
                    }
                }

                if max_iter > max_global {
                    max_global = max_iter;
                }
            } else {
                sum_so_far.clear();

                sum_so_far.push(max_iter + 1);

                if max_iter + 1 > max_global {
                    max_global = max_iter + 1;
                }

                max_iter = 0;
            }
        }

        max_global
    }

    pub fn find_max_consecutive_ones_leetcode(nums: Vec<i32>) -> i32 {
        let mut longestSequence = 0;
        let mut left = 0;
        let mut right = 0;
        let mut numZeroes = 0;

        while right < nums.len() {
            // add the right most element into our window
            if nums[right] == 0 {
                numZeroes += 1;
            }

            //if our window is invalid, contract our window
            while numZeroes == 2 {
                if nums[left] == 0 {
                    numZeroes -= 1;
                }
                left += 1;
            }

            // update our longest sequence answer
            longestSequence = max(longestSequence, right - left + 1);

            // expand our window
            right += 1;
        }

        return longestSequence.try_into().unwrap();
    }
}

#[cfg(test)]
mod test {
    use crate::max_consecutive_ones::Solution;

    #[test]
    fn test_one() {
        assert_eq!(4, Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0]));
    }

    #[test]
    fn test_two() {
        assert_eq!(
            4,
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1])
        );
    }
}

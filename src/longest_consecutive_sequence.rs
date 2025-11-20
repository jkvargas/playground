// https://leetcode.com/problems/longest-consecutive-sequence/description/?envType=study-plan-v2&envId=top-interview-150

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::new();

        for i in &nums {
            set.insert(*i);
        }

        let mut longest_streak = 0;
        for &i in set.iter() {
            if !set.contains(&(i - 1)) {
                let mut current_num = i;
                let mut current_strak = 1;

                while set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_strak += 1;
                }

                longest_streak = std::cmp::max(longest_streak, current_strak);
            }
        }

        longest_streak
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_consecutive_sequence::Solution;

    #[test]
    fn it_works() {
        assert_eq!(4, Solution::longest_consecutive(vec![400, 4, 200, 1, 3, 2]));
    }
}

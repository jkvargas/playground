// https://leetcode.com/problems/minimum-swaps-to-group-all-1s-together/?envType=study-plan-v2&envId=amazon-spring-23-high-frequency

use std::cmp::max;

struct Solution;

impl Solution {
    pub fn min_swaps(data: Vec<i32>) -> i32 {
        let ones = data.iter().filter(|&&x| x == 1).count();

        let mut pos_x = 0;
        let mut pos_y = 0;

        let mut best_so_far = 0;
        let mut max_ones_found = 0;

        while pos_y < data.len() {
            if pos_y - pos_x == ones {
                if data[pos_x] == 1 {
                    max_ones_found -= 1;
                }

                pos_x += 1;

                if data[pos_y] == 1 {
                    max_ones_found += 1;
                    best_so_far = best_so_far.max(max_ones_found);
                }

                pos_y += 1;
            } else {
                if data[pos_y] == 1 {
                    max_ones_found += 1;
                    best_so_far = best_so_far.max(max_ones_found);
                }
                pos_y += 1;
            }
        }

        ones as i32 - best_so_far as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::minimum_swaps::Solution;

    #[test]
    fn test_one() {
        assert_eq!(1, Solution::min_swaps(vec![1, 0, 1, 0, 1]));
    }

    #[test]
    fn test_two() {
        assert_eq!(
            3,
            Solution::min_swaps(vec![1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1])
        );
    }
}

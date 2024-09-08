// https://leetcode.com/problems/count-almost-equal-pairs-i/

use std::{collections::HashMap, f32::MAX_EXP};
use std::cmp::max;
use std::collections::VecDeque;
use log::Level;

struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>) -> i32 {
        let mut maps = Vec::new();
        let mut deques = Vec::new();
        let mut max_count = 0;
        for i in &nums {
            let mut count = 0;
            let mut deque = VecDeque::new();
            let mut map = HashMap::new();
            i.to_string().chars().for_each(|letter| {
                count += 1;
                *map.entry(letter).or_insert(0) += 1;
                deque.push_front(letter);
            });
            maps.push(map);
            deques.push(deque);
            max_count = max_count.max(count);
        }

        for map in &mut maps {
            let sum = map.values().sum::<i32>();
            if sum < max_count {
                *map.entry('0').or_insert(0) += max_count - sum;

            }
        }

        dbg!(&maps);

        2
    }
}

#[cfg(test)]
mod test {
    use crate::count_almost_equals_pairs_i::Solution;

    #[test]
    fn test_one() {
        assert_eq!(2, Solution::count_pairs(vec![3,12,30,17,21]))
    }
}
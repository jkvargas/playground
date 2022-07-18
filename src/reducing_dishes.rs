// https://leetcode.com/problems/reducing-dishes/

use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_satisfaction(mut satisfaction: Vec<i32>) -> i32 {
        satisfaction.sort();

        let mut memo = HashMap::new();

        Self::cook(&satisfaction, 0, 1, &mut memo)
    }

    fn cook(satisfaction: &Vec<i32>, i: usize, time: i32, memo: &mut HashMap<(usize, i32), i32>) -> i32 {
        if i >= satisfaction.len() {
            return 0;
        }

        if memo.contains_key(&(i, time)) {
            return *memo.get(&(i, time)).unwrap();
        }

        let best_value = max(satisfaction[i] * time + Self::cook(satisfaction, i + 1, time + 1, memo), Self::cook(satisfaction, i + 1, time, memo));

        memo.insert((i, time), best_value);

        best_value
    }
}

#[cfg(test)]
mod tests {
    use crate::reducing_dishes::Solution;

    #[test]
    fn test_one() {
        assert_eq!(14, Solution::max_satisfaction(vec![-1,-8,0,5,-9]))
    }

    #[test]
    fn test_two() {
        assert_eq!(20, Solution::max_satisfaction(vec![4,3,2]))
    }

    #[test]
    fn test_three() {
        assert_eq!(0, Solution::max_satisfaction(vec![-1,-4,-5]))
    }
}
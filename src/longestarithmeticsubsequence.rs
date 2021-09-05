use std::process::exit;
use std::{cmp::max, collections::HashMap};

// https://leetcode.com/problems/longest-arithmetic-subsequence/

pub struct Solution;

type Index = usize;
type Difference = i32;
type Amount = i32;

const MIN_COUNT: i32 = 2;

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut result = 2;
        let mut dp: HashMap<Difference, HashMap<Index, Amount>> = HashMap::new();

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let d = i32::abs(nums[i] - nums[j]);
                let max_value = max(Self::get_from_dp(&dp, i, d) + 1, 2);
                Self::insert_or_update(&mut dp, j, d, max_value);
                result = max(result, max_value);
            }
        }

        result
    }

    fn insert_or_update(
        dp: &mut HashMap<Difference, HashMap<Index, Amount>>,
        i: Index,
        d: Difference,
        val: Amount,
    ) {
        if let Some(dif) = dp.get_mut(&d) {
            if let Some(am) = dif.get_mut(&i) {
                *am = val;
            } else {
                dif.insert(i, val);
            }
        } else {
            let mut tmp = HashMap::new();
            tmp.insert(i, val);
            dp.insert(d, tmp);
        }
    }

    fn get_from_dp(
        dp: &HashMap<Difference, HashMap<Index, Amount>>,
        i: Index,
        d: Difference,
    ) -> Amount {
        if let Some(f) = dp.get(&d) {
            if let Some(s) = f.get(&i) {
                return *s;
            }
        }

        0
    }
}

#[test]
fn test_one() {
    assert_eq!(4, Solution::longest_arith_seq_length(vec![3, 6, 9, 12]));
}

#[test]
fn test_two() {
    assert_eq!(3, Solution::longest_arith_seq_length(vec![9, 4, 7, 2, 10]));
}

#[test]
fn test_three() {
    assert_eq!(
        4,
        Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8])
    );
}

// https://leetcode.com/problems/permutations-ii/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();

        let mut map = HashMap::new();
        for i in &nums {
            *map.entry(*i).or_insert(0) += 1;
        }

        let keys = map.keys().map(|i| *i).collect::<Vec<i32>>();

        f(&mut map, Vec::new(), &mut res, nums.len(), &keys);
        res
    }
}

fn f(
    nums: &mut HashMap<i32, i32>,
    mut v: Vec<i32>,
    results: &mut Vec<Vec<i32>>,
    n: usize,
    keys: &Vec<i32>,
) {
    if v.len() == n {
        results.push(v.clone());
        return;
    }

    for i in keys {
        if *nums.get(i).unwrap() == 0 {
            continue;
        }

        *nums.get_mut(i).unwrap() -= 1;
        v.push(*i);
        f(nums, v.clone(), results, n, keys);
        v.pop();
        *nums.get_mut(i).unwrap() += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::permutations_ii::Solution;

    #[test]
    fn test_one() {
        let sol = Solution::permute_unique(vec![1, 1, 2]);
        assert_eq!(sol, vec![vec![2, 1, 1], vec![1, 2, 1], vec![1, 1, 2]]);
    }
}

use crate::orderedmap::OrderedMap;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut results: Vec<Vec<i32>> = Vec::new();
        let mut ignore = OrderedMap::new();

        Self::find_permutations(&mut ignore, &nums, &mut results);

        results
    }

    fn find_permutations(
        ignore: &mut OrderedMap,
        original: &Vec<i32>,
        results: &mut Vec<Vec<i32>>,
    ) {
        if ignore.len() == original.len() {
            let mut res: Vec<i32> = ignore.iter().map(|x| original[*x]).collect();
            results.push(res);
            return;
        }

        for i in 0..original.len() {
            if !ignore.contains(&i) {
                ignore.insert(i);
                Self::find_permutations(ignore, original, results);
                ignore.remove(&i);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permute_1() {
        let result = Solution::permute(vec![1, 2, 3]);
        assert_eq!(
            result,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}

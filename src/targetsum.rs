use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, s: i32) -> i32 {
        let mut map = HashMap::new();

        Solution::walk_through(&nums, s, 0, 0, &mut map)
    }

    fn walk_through(
        vec: &Vec<i32>,
        s: i32,
        index: usize,
        sum: i32,
        done: &mut HashMap<(usize, i32), i32>,
    ) -> i32 {
        if index == vec.len() {
            if sum == s {
                return 1;
            }
            return 0;
        }

        let key = (index, sum);

        if let Some(occurrence) = done.get(&key) {
            return *occurrence;
        }

        let new_sum = Self::walk_through(vec, s, index + 1, sum + vec[index], done);
        let substract = Self::walk_through(vec, s, index + 1, sum - vec[index], done);

        let result = new_sum + substract;

        done.insert(key, result);

        result
    }
}

#[test]
pub fn example1() {
    assert_eq!(5, Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3));
}

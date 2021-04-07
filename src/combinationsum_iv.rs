use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut res_map = HashMap::new();

        Self::combs(&mut res_map, &nums, target) 
    }

    pub fn combs(memo: &mut HashMap<i32, i32>, nums: &Vec<i32>, remaining: i32) -> i32 {
        if remaining == 0 {
            return 1;
        }

        if let Some(cn) = memo.get(&remaining) {
            return *cn;
        }

        let mut result = 0;

        for i in nums {
            if remaining - i >= 0 {
                result += Self::combs(memo, nums, remaining - i);
            }
        }

        memo.insert(remaining, result);

        result
    }

}

#[test]
fn test_one() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
}

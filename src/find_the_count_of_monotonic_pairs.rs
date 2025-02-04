use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        dp(0, 0, &nums, &mut map) as i32
    }
}

const MOD: i64 = 1_000_000_007;

fn dp(index: i64, previous: i64, nums: &Vec<i32>, memo: &mut HashMap<(i64, i64), i64>) -> i64 {
    if index as usize >= nums.len() {
        return 1;
    }

    if let Some(result) = memo.get(&(index, previous)) {
        return *result;
    }

    let mut c = 0;

    for val in previous..nums[index as usize] as i64 {
        let k = nums[index as usize] as i64 - val;

        if index == 0 || k <= nums[index as usize - 1] as i64 - previous {
            c = (c + dp(index + 1, val, nums, memo)) % MOD;
        }
    }

    memo.insert((index, previous), c).unwrap()
}

#[test]
fn test_one() {
    assert_eq!(4, Solution::count_of_pairs(vec![2, 3, 2]));
}

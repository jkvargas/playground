use std::{cmp::max, collections::HashMap};

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }

        let mut map = HashMap::new();

        Solution::try_rob(0, &mut map, &nums)
    }

    fn try_rob(index: usize, map: &mut HashMap<usize, i32>, nums: &Vec<i32>) -> i32 {
        if let Some(res) = map.get(&index) {
            return *res;
        }

        let next = index + 1;

        if next >= nums.len() {
            map.insert(index, nums[index]);
            return *map.get(&index).unwrap();
        }

        let val = Self::try_rob(next, map, nums);

        if index + 2 > nums.len() - 1 {
            map.insert(index, max(val, nums[index]));
            return *map.get(&index).unwrap();
        }

        let a = nums[index] + *map.get(&(index + 2)).unwrap();
        let b = *map.get(&(index + 1)).unwrap();

        map.insert(index, max(a, b));

        *map.get(&index).unwrap()
    }
}

#[test]
pub fn example1() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
}

#[test]
pub fn example2() {
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
}

#[test]
pub fn example3() {
    assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
}

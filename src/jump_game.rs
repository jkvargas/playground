use std::collections::HashSet;

// https://leetcode.com/problems/jump-game/description/
struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut map = HashSet::new();
        Self::perform_jump(&nums, 0, &mut map)
    }

    fn perform_jump(nums: &Vec<i32>, pos: usize, memo: &mut HashSet<usize>) -> bool {
        if memo.contains(&pos) {
            return false;
        }

        if pos == nums.len() - 1 {
            return true;
        }

        let possible_jump_size = nums[pos];

        for i in 1..possible_jump_size as usize + 1 {
            if Self::perform_jump(nums, pos + i, memo) {
                return true;
            }
        }

        memo.insert(pos);
        return false;
    }
}

// better solution
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut index_max = 0;
    let index_last = nums.len() - 1;
    for i in 0..nums.len() - 1 {
        index_max = index_max.max(nums[i] as usize + i as usize);
        if index_max >= index_last {
            break; //  for
        }
        if index_max == i {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::jump_game::Solution;

    #[test]
    fn test_one() {
        let nums = vec![2,3,1,1,4];
        assert!(Solution::can_jump(nums));
    }
}
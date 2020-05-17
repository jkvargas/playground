struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        Self::len_lis(&nums, i32::min_value(), 0)
    }

    fn len_lis(nums: &Vec<i32>, prev: i32, cur_pos: usize) -> i32 {
        if cur_pos == nums.len() {
            return 0;
        }

        let mut taken = 0;
        if nums[cur_pos] > prev {
            taken = 1 + Self::len_lis(nums, nums[cur_pos], cur_pos +1);
        }
        let mut not_taken = Self::len_lis(nums, prev, cur_pos + 1);
        std::cmp::max(taken, not_taken)
    }
}

// 10         101
//  \         / \
//   9   5   7  18
//    \ / \ /
//     2   3

// 3
// 1

// if e_res < 0 {
//     continue;
// }
// if old_res > 0 e res < 0 {
//     return max(old_res, old_res + e_res + go_through(i + 1))
// }

// 10 -> res = 0 global = 0
// 9  -> res = vec[i] - vec[i-1] = -1 global = 0
// 2 -> res = -5 global = 0
// 5 -> res = 3 global = 3
// 3 -> res = -2 global = 3 ->

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_stone_weight_1() {
        assert_eq!(Solution::length_of_lis(vec![10,9,2,5,3,7,101,18]), 4);
    }
}
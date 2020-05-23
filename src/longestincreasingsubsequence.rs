struct Solution;

impl Solution {
    /// nlogn
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let mut lis = Vec::new();
        lis.push(nums[0]);

        for n in nums {
            lis.binary_search(&n).map_err(|x| {
                if x >= lis.len() {
                    lis.push(n);
                } else {
                    lis[x] = n;
                }
            });
        }

        lis.len() as i32
    }

    pub fn lldynamic_programming(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 { return nums.len() as i32; }

        let mut dp = vec![1; nums.len()];
        let mut max_lis = 1;

        for i in 1..nums.len() {
            let mut tmp_max = 0;
            for j in 0..i {
                if nums[i] > nums[j] {
                    tmp_max = std::cmp::max(tmp_max, dp[j]);
                }
                dp[i] = tmp_max + 1;
            }
            max_lis = std::cmp::max(max_lis, dp[i]);
        }
        max_lis
    }

    pub fn length_of_lis_s(nums: Vec<i32>) -> i32 {
        Self::ls(&nums, i32::min_value(), 0)
    }

    pub fn ls(nums: &Vec<i32>, prev: i32, cur: usize) -> i32 {
        if cur == nums.len() {
            return 0;
        }

        let mut temp = 0;

        if nums[cur] > prev {
            temp = 1 + Self::ls(nums, nums[cur], cur + 1);
        }

        let not = Self::ls(nums, prev, cur + 1);

        return std::cmp::max(not, temp);
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

    #[test]
    fn last_stone_weight_2() {
        assert_eq!(Solution::length_of_lis(vec![2, 2]), 1);
    }

    #[test]
    fn last_stone_weight_3() {
        assert_eq!(Solution::length_of_lis(vec![1,3,6,7,9,4,10,5,6]), 6);
    }
}
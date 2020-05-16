struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {

    }

    fn go(vec: &Vec<i32>, position: usize) -> (i32, usize) {
        let mut old_res = None;
        for i position + 1..vec.len() {
            let res = vec[position] - vec[position - 1];

            if old_res > 0 {
                if res < 0 {
                    return std::cmp::max(old_res, old_res + res + Self::go(vec, i));
                }
            }

            old_res = res;
        }
    }
}

// 10         101
//  \         / \
//   9   5   7  18
//    \ / \ /
//     2   3

// 3
// 1

if e_res < 0 {
    continue;
}
if old_res > 0 e res < 0 {
    return max(old_res, old_res + e_res + go_through(i + 1))
}

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
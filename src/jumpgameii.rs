use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        let res = Self::try_jump(&nums, 0, &mut map).unwrap();

        dbg!(&map);

        res
    }

    fn try_jump(
        nums: &Vec<i32>,
        pos: usize,
        jump_count: &mut HashMap<usize, Option<i32>>,
    ) -> Option<i32> {
        if let Some(&res) = jump_count.get(&pos) {
            return res;
        }

        if pos > nums.len() - 1 {
            return None;
        }

        if pos == nums.len() - 1 {
            return Some(0);
        }

        let mut min_jumps = None;

        if nums[pos] > 0 {
            for jump_size in (1..=nums[pos]).rev() {
                match Self::try_jump(nums, pos + jump_size as usize, jump_count) {
                    None => {
                        continue;
                    }
                    Some(val) => match min_jumps {
                        None => min_jumps = Some(1 + val),
                        Some(min) => min_jumps = Some(std::cmp::min(min, 1 + val)),
                    },
                }
            }
        }

        jump_count.insert(pos, min_jumps);

        min_jumps
    }
}

// leetcode best answer

struct SolutionLeet;

impl SolutionLeet {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; nums.len()];
        let len = nums.len();
        (0..len - 1).rev().for_each(|idx| {
            dp[idx] = if idx + nums[idx] as usize >= len - 1 {
                1
            } else {
                dp[idx + 1..=idx + nums[idx] as usize]
                    .iter()
                    .filter(|x| **x != 0)
                    .min()
                    .map_or(0, |x| *x + 1)
            }
        });
        dp[0]
    }
}

#[test]
fn test_one() {
    assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
}

#[test]
fn test_two() {
    assert_eq!(2, Solution::jump(vec![2, 3, 0, 1, 4]));
}

#[test]
fn test_three() {
    assert_eq!(3, Solution::jump(vec![1, 2, 1, 1, 1]));
}

#[test]
fn test_four() {
    assert_eq!(
        3,
        Solution::jump(vec![
            9, 8, 2, 2, 0, 2, 2, 0, 4, 1, 5, 7, 9, 6, 6, 0, 6, 5, 0, 5
        ])
    );
}

#[test]
fn test_five() {
    assert_eq!(3, Solution::jump(vec![5, 9, 3, 2, 1, 0, 2, 3, 3, 1, 0, 0]));
}

struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut sum = Vec::new();

        Self::try_sum(&mut result, &mut sum, &candidates, target);

        result
    }

    fn try_sum(res: &mut Vec<Vec<i32>>, sum: &[i32], nums: &[i32], target: i32) {
        let total: i32 = sum.iter().sum();

        if total > target {
            return;
        }

        if total == target {
            res.push(sum.to_vec());
            return;
        }

        for (i, v) in nums.iter().enumerate() {
            let mut s = sum.to_vec();
            s.push(*v);
            Self::try_sum(res, &s, &nums[i..], target);
        }
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );
}

#[test]
fn test2() {
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 5], 8),
        vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
    );
}

#[test]
fn test3() {
    let res: Vec<Vec<i32>> = Vec::new();

    assert_eq!(Solution::combination_sum(vec![2], 1), res);
}

#[test]
fn test4() {
    assert_eq!(Solution::combination_sum(vec![1], 1), vec![vec![1]]);
}

#[test]
fn test5() {
    assert_eq!(Solution::combination_sum(vec![1], 2), vec![vec![1, 1]]);
}

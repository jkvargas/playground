// https://leetcode.com/discuss/interview-question/4978447/google-coding-interview-experience

use std::collections::HashMap;

struct Solution;

impl Solution {
    fn best_score_jumps(vec: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();

        dp(&mut memo, &vec, 0, 0)
    }
}

fn dp(
    memo: &mut HashMap<(usize, usize), i32>,
    vec: &Vec<i32>,
    last_index: usize,
    current_index: usize,
) -> i32 {
    if current_index >= vec.len() {
        return 0;
    }

    if memo.contains_key(&(current_index, last_index)) {
        return memo[&(current_index, last_index)];
    }

    let land = ((current_index - last_index) as i32 * vec[current_index])
        + dp(memo, vec, current_index, current_index + 1);
    let do_not_land = dp(memo, vec, last_index, current_index + 1);
    let result = land.max(do_not_land);

    memo.insert((current_index, last_index), result);

    result
}

#[cfg(test)]
mod tests {
    use crate::best_score_jumps::Solution;

    #[test]
    fn test_one() {
        assert_eq!(32, Solution::best_score_jumps(vec![3, 12, 9, 10]));
    }
}

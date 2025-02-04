// https://leetcode.com/problems/maximize-score-of-numbers-in-ranges/description/

struct Solution;

impl Solution {
    pub fn max_possible_score(mut start: Vec<i32>, d: i32) -> i32 {
        start.sort();
        let n = start.len();
        let mut low: i64 = 0;
        let mut hi: i64 = start[n - 1] as i64 - start[0] as i64 + d as i64;
        let mut ans: i64 = 0;

        while low <= hi {
            let mid = (low + hi) / 2;
            if check(&start, d, mid) {
                ans = mid;
                low = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        ans as i32
    }
}

pub fn check(start: &Vec<i32>, d: i32, mid: i64) -> bool {
    let mut prev: i64 = start[0] as i64;
    for i in 1..start.len() {
        let next: i64 = std::cmp::max(prev + mid, start[i] as i64);
        if next > start[i] as i64 + d as i64 {
            return false;
        }
        prev = next;
    }
    true
}

#[test]
fn test_one() {
    assert_eq!(Solution::max_possible_score(vec![6, 0, 3], 2), 4);
}

#[test]
fn test_two() {
    assert_eq!(Solution::max_possible_score(vec![6, 0, 3], 2), 4);
}

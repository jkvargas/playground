// https://leetcode.com/problems/minimum-number-of-seconds-to-make-mountain-height-zero/description/

struct Solution;

// Time limit exceeded
impl Solution {
    pub fn min_number_of_seconds_from_web(mut mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut s: i64 = 0;
        let mut e: i64 = 1_000_000_000_000_000_000; // 1e18
        let mut ans: i64 = 0;

        while s <= e {
            let mid = s + (e - s) / 2;
            if can_find(mid, mountain_height, &worker_times) {
                e = mid - 1;
                ans = mid;
            } else {
                s = mid + 1;
            }
        }
        ans
    }

    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let mut vec = vec![vec![-1i64; worker_times.len()]; mountain_height as usize];
        let mut ptrs: Vec<Option<(usize, usize)>> = vec![None; worker_times.len()];

        let mut climb = 0;
        let mut result = i64::MIN;

        while climb < mountain_height {
            for i in 0..worker_times.len() {
                if vec[climb as usize][i] == -1 {
                    vec[climb as usize][i] = cost(worker_times[i], climb + 1i32);
                }
            }

            let mut row_chosen = 0;
            let mut col_chosen = 0;
            let mut min = i64::MAX;
            let mut worker = 0;
            for i in 0..ptrs.len() {
                let (x, y) = if let Some(x) = ptrs[i] { x } else { (0, i) };
                let value_to_compare = vec[x][y];

                if value_to_compare < min {
                    min = value_to_compare;
                    row_chosen = x;
                    col_chosen = y;
                    worker = i;
                }
            }

            result = result.max(min);
            ptrs[worker] = Some((row_chosen + 1, col_chosen));
            climb += 1;
        }

        result
    }
}

fn can_find(t: i64, h: i32, wt: &Vec<i32>) -> bool {
    let mut total_h: i64 = 0;

    for &w in wt.iter() {
        let mut left: i64 = 0;
        let mut right: i64 = 1_000_000;
        while left <= right {
            let mid = left + (right - left) / 2;
            if (w as i64) * mid * (mid + 1) / 2 <= t {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        total_h += right;
        if total_h >= h as i64 {
            return true;
        }
    }
    total_h >= h as i64
}

fn cost(worker_time: i32, amount: i32) -> i64 {
    let mut sum = 0i64;
    for i in 1..=amount {
        sum += (worker_time * i) as i64;
    }
    sum
}

#[test]
fn test_one() {
    assert_eq!(
        3,
        Solution::min_number_of_seconds_from_web(4, vec![2, 1, 1])
    );
}

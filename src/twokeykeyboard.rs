use std::{cmp::min, collections::HashMap};

struct Solution;

impl Solution {
    pub fn min_steps(mut n: i32) -> i32 {
        let mut ans: i32 = 0;
        let mut d: i32 = 2;

        while n > 1 {
            while n % d == 0 {
                ans += d;
                n /= d;
            }
            d += 1;
        }

        ans
    }
}

#[test]
fn first_test() {
    assert_eq!(3, Solution::min_steps(3));
}

#[test]
fn second_test() {
    assert_eq!(0, Solution::min_steps(1));
}

#[test]
fn third_test() {
    assert_eq!(7, Solution::min_steps(7));
}

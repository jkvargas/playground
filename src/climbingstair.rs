struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut memo: Vec<i32> = vec![0; n as usize + 1];

        Self::check_stairs(0, n, &mut memo)
    }

    fn check_stairs(i: i32, n: i32, memo: &mut Vec<i32>) -> i32 {
        if i > n {
            return 0;
        }

        if i == n {
            return 1;
        }

        if memo[i as usize] > 0 {
            return memo[i as usize];
        }

        let a = Self::check_stairs(i + 1, n, memo);
        let b = Self::check_stairs(i + 2, n, memo);
        memo[i as usize] = a + b;

        dbg!(&memo);

        memo[i as usize]
    }
}

#[test]
pub fn checks() {
    assert_eq!(2, Solution::climb_stairs(2));
    assert_eq!(3, Solution::climb_stairs(3));
    assert_eq!(5, Solution::climb_stairs(4));
}

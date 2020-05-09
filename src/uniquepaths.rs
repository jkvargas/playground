struct Solution;
// https://leetcode.com/problems/unique-paths/
// (m+n)!/(m! * n!)

impl Solution {
    pub fn unique_paths_l(m: i32, n: i32) -> i32 {
        let mut routes = 0;

        Self::go_through(&mut routes, 1, 1, m, n);

        routes
    }

    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 {
            return 1;
        }

        Self::unique_paths(m - 1, n) + Self::unique_paths(m, n - 1)
    }

    fn go_through(routes: &mut i32, i: i32, j: i32, m: i32, n: i32) {
        if i == m && j == n {
            *routes += 1;
            return;
        }

        if i + 1 <= m {
            Self::go_through(routes, i + 1, j, m, n);
        }

        if j + 1 <= n {
            Self::go_through(routes, i, j + 1, m, n);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_paths_1() {
        assert_eq!(Solution::unique_paths(7, 3), 28);
    }
}
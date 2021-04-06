use std::cmp::min;

struct Solution;

impl Solution {
    pub fn minimum_total_jho(triangle: Vec<Vec<i32>>) -> i32 {
        Self::go(0, 0, 0, &triangle)
    }

    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        if triangle.len() == 0 {
            return 0;
        }

        if triangle.len() == 1 {
            return triangle[0][0];
        }

        let mut dp = vec![Vec::new(); triangle.len()];
        dp[0].push(triangle[0][0]);

        let mut width = 0;

        for i in 1..triangle.len() {
            width = i + 1;

            for j in 0..width {
                let sum = if j == 0 {
                    dp[i - 1][j] + triangle[i][j]
                } else if j == width - 1 {
                    dp[i - 1][j - 1] + triangle[i][j]
                } else {
                    min(dp[i - 1][j - 1], dp[i - 1][j]) + triangle[i][j]
                };

                dp[i].push(sum);
            }
        }

        let mut min = i32::MAX;

        for x in &dp[triangle.len() - 1] {
            if *x < min {
                min = *x;
            }
        }

        min
    }

    fn go(cur_ind: usize, mut sum: i32, tri_ind: usize, tri: &Vec<Vec<i32>>) -> i32 {
        sum += tri[tri_ind][cur_ind];

        if tri.len() - 1 == tri_ind {
            return sum;
        }

        std::cmp::min(
            Self::go(cur_ind, sum, tri_ind + 1, tri),
            Self::go(cur_ind + 1, sum, tri_ind + 1, tri),
        )
    }
}

#[test]
fn test_one() {
    assert_eq!(
        11,
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
    );
}

#[test]
fn test_sec() {
    assert_eq!(-10, Solution::minimum_total(vec![vec![-10]]));
}

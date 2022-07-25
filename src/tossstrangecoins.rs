struct Solution;

impl Solution {
    pub fn probability_of_heads(prob: Vec<f64>, target: i32) -> f64 {
        let mut dp = vec![vec![0.0; std::cmp::max(target as usize + 1, 2)]; prob.len()];

        dp[0][0] = 1.0 - prob[0];
        dp[0][1] = prob[0];

        for i in 1..prob.len() {
            for j in 0..=target as usize {
                dp[i][j] = dp[i - 1][j] * (1.0 - prob[i])
                    + if j == 0 {
                        0.0
                    } else {
                        dp[i - 1][j - 1] * prob[i]
                    };
            }
        }

        dp[prob.len() - 1][target as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::tossstrangecoins::Solution;

    #[test]
    fn test_one() {
        assert_eq!(0.4, Solution::probability_of_heads(vec![0.4], 1));
    }

    #[test]
    fn test_two() {
        assert_eq!(
            0.0,
            Solution::probability_of_heads(
                vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                9
            )
        );
    }
}

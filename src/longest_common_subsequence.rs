struct Solution;

// https://leetcode.com/problems/longest-common-subsequence/description/?envType=study-plan-v2&envId=amazon-spring-23-high-frequency

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let n = text1.len();
        let m = text2.len();

        let text1: Vec<char> = text1.chars().collect();
        let text2: Vec<char> = text2.chars().collect();

        let mut dp = vec![vec![0; m + 1]; n + 1];

        for i in 1..=n {
            for j in 1..=m {
                dp[i][j] = dp[i][j]
                    .max(dp[i - 1][j])
                    .max(dp[i][j - 1])
                    .max(dp[i - 1][j - 1] + (text1[i - 1] == text2[j - 1]) as i32);
            }
        }

        dp[n][m]
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_common_subsequence::Solution;

    #[test]
    fn test_two() {
        let string_one = "abcde".to_string();
        let string_two = "ace".to_string();

        assert_eq!(Solution::longest_common_subsequence(string_one, string_two), 3);
    }

    #[test]
    fn test_one() {
        let string_one = "actgattag".to_string();
        let string_two = "gtgtgatcg".to_string();

        assert_eq!(
            3,
            Solution::longest_common_subsequence(string_one, string_two)
        );
    }
}

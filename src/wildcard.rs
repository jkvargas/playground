struct Solution;

impl Solution {
    pub fn is_match(mut s: String, mut p: String) -> bool {
        let word : Vec<char> = s.chars().collect();
        let rule : Vec<char> = p.chars().collect();
        let word_len = s.len();
        let rule_len = p.len();
        let mut word_index : usize = 0;
        let mut rule_index : usize = 0;
        let mut i_idx : i32 = -1;
        let mut j_idx : i32 = -1;

        /// *ab  --- cdab
        while word_index < word_len {
            if rule_index < rule_len && (rule[rule_index] == '?' || rule[rule_index] == word[word_index]) {
                rule_index += 1;
                word_index += 1;
            } else if rule_index < rule_len && rule[rule_index] == '*' {
                i_idx = rule_index as i32;
                j_idx = word_index as i32;
                rule_index += 1;
            } else if i_idx == -1 {
                return false;
            } else {
                rule_index = (i_idx + 1) as usize;
                word_index = (j_idx + 1) as usize;
                j_idx = word_index as i32;
            }
        }

        for i in rule_index..rule_len {
            if rule[i] != '*' {
                return false;
            }
        }

        true
    }

    pub fn is_match_1<S: AsRef<str>>(s: S, p: S) -> bool {
        let (s, p) = (s.as_ref().as_bytes(), p.as_ref().as_bytes());
        if s.is_empty() { return p.is_empty() || p.iter().all(|x| *x == b'*'); }
        else if p.is_empty() { return false; }

        let mut dp = vec![false; p.len()+1];
        dp[0] = true;
        for j in 1..dp.len() {
            dp[j] = if p[j-1] == b'*' { dp[j-1] } else { break };
        }
        for i in 1..=s.len() {
            let mut dp_i_1_j_1 = dp[0];
            for j in 1..dp.len() {
                let saved = dp[j];
                dp[j] = if s[i-1] == p[j-1] || p[j-1] == b'?' { dp_i_1_j_1 }
                else if p[j-1] == b'*' { dp[j] || dp[j-1] }
                else { false };
                dp_i_1_j_1 = saved;
            }
            if i == 1 { dp[0] = false; }
        }
        *dp.last().unwrap()
    }

    pub fn is_match_2(s: String, p: String) -> bool {
        let m = s.len();
        let n = p.len();
        let mut dp = vec![vec![false; n + 1]; m + 1];
        dp[0][0] = true;
        //if p == 0 s != 0 false
        //if s==0 p!=0 all of p is *?
        for j in 1..n+1 {
            if &p[j - 1..j] == "*" {
                dp[0][j] = true
            } else {
                break;
            }
        }
        for i in 1..m + 1 {
            for j in 1..n + 1 {
                let current_pattern = (&p[j - 1..j]).to_owned();
                let current_str = (&s[i - 1..i]).to_owned();
                if current_pattern != "*".to_string() {
                    dp[i][j] = dp[i - 1][j - 1]
                        && (current_str == current_pattern || current_pattern == "?".to_string());
                } else {
                    dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
                }
            }
        }
        dp[m][n]
    }
}

// a

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_match_1() {
        assert_eq!(Solution::is_match("acdcb".to_string(), "a*c?b".to_string()), true);
    }

    // a - * - c - ? - b
    // * - a - ? - jhonny - *

    #[test]
    fn is_match_2() {
        assert_eq!(Solution::is_match("adceb".to_string(), "*a*b".to_string()), true);
    }
}
struct Solution;

impl Solution {
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        let (m, n) = (s.len(), t.len());

        if m > n {
            return Solution::is_one_edit_distance(t.into_iter().collect(), s.into_iter().collect());
        }

        if n - m > 1 {
            return false;
        }

        let mut i = 0;
        while i < m {
            if s[i] != t[i] {
                if m == n {
                    return s[i + 1..] == t[i + 1..];
                } else {
                    return s[i..] == t[i + 1..];
                }
            }
            i += 1;
        }

        n == m + 1
    }
}

#[cfg(test)]
mod tests {
    use crate::one_edit_distance::Solution;

    #[test]
    fn test_is_one_edit_distance() {
        assert!(Solution::is_one_edit_distance("ab".to_string(), "acb".to_string()));
    }
}
struct Solution;

impl Solution {
    pub fn is_match_jho(s: String, p: String) -> bool {
        let phrase: Vec<char> = s.chars().collect();
        let rules: Vec<char> = p.chars().collect();
        let mut stack: Vec<char> = Vec::new();
        let mut rule_pos = 0;
        let mut phrase_pos = 0;

        stack.push(rules[rule_pos]);
        rule_pos += 1;

        loop {
            if stack.is_empty() {
                if phrase_pos > phrase.len() - 1 {
                    return true;
                } else {
                    break;
                }
            } else {
                if rule_pos < rules.len() && rules[rule_pos] == '*' && !stack.is_empty() {
                    let popped = stack.pop().unwrap();

                    while phrase_pos < phrase.len() && (popped == '.' || phrase[phrase_pos] == popped) {
                        phrase_pos += 1;
                    }

                    rule_pos += 1;
                } else {
                    let letter = stack.pop().unwrap();

                    if phrase_pos > phrase.len() - 1 || (letter != '.' && letter != phrase[phrase_pos]) {
                        return false;
                    } else {
                        phrase_pos += 1;
                    }
                }

                if rule_pos < rules.len() {
                    stack.push(rules[rule_pos]);
                    rule_pos += 1;
                }
            }
        }

        false
    }

    pub fn is_match(s: String, p: String) -> bool {
        let mut result : Vec<Vec<Option<bool>>> = vec![vec![None; p.len() + 1]; s.len() + 1];
        let text : Vec<char> = s.chars().collect();
        let pattern : Vec<char> = p.chars().collect();

        Self::dp(0, 0, &text, &pattern, &mut result)
    }

    fn dp(i: usize, j: usize, text: &Vec<char>, pattern: &Vec<char>, result: &mut Vec<Vec<Option<bool>>>) -> bool {
        if let Some(res) = result[i][j] {
            return res;
        }

        let mut ans = false;

        if j == pattern.len() {
            ans = i == text.len();
        } else {
            let first_match = (i < text.len() &&
                (pattern[j] == text[i] || pattern[j] == '.'));

            if j + 1 < pattern.len() && pattern[j+1] == '*' {
                ans = Self::dp(i, j + 2, text, pattern, result) || first_match && Self::dp(i + 1, j, text, pattern, result);
            } else {
                ans = first_match && Self::dp(i + 1, j + 1, text, pattern, result);
            }
        }

        result[i][j] = Some(ans);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_match_1() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    }

    #[test]
    fn is_match_2() {
        assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    }

    #[test]
    fn is_match_3() {
        assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    }

    #[test]
    fn is_match_4() {
        assert_eq!(Solution::is_match("aab".to_string(), "c*a*b".to_string()), true);
    }

    #[test]
    fn is_match_5() {
        assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
    }

    #[test]
    fn is_match_6() {
        assert_eq!(Solution::is_match("vera aaaao de 2020".to_string(), "vera.a*o.de.2020".to_string()), true);
    }

    #[test]
    fn is_match_7() {
        assert_eq!(Solution::is_match("porco fumo".to_string(), ".*.*".to_string()), true);
    }

    #[test]
    fn is_match_8() {
        assert_eq!(Solution::is_match("ab".to_string(), ".*c".to_string()), false);
    }

    #[test]
    fn is_match_9() {
        assert_eq!(Solution::is_match("aaa".to_string(), "a*a".to_string()), true);
    }
}
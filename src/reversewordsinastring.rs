use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut result: Vec<String> = Vec::new();
        let letters = s.chars().collect();
        Self::get_words(&letters, 0, &mut result);

        result.join(" ")
    }

    fn get_words(s: &Vec<char>, pos: usize, result: &mut Vec<String>) {
        let mut start_word = false;
        let mut result_string = String::new();

        for i in pos..s.len() {
            if s[i] == ' ' {
                if start_word {
                    Self::get_words(s, i, result);
                    break;
                } else {
                    continue;
                }
            } else {
                start_word = true;
                result_string.push(s[i]);
            }
        }

        if start_word {
            result.push(result_string);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_words_1() {
        assert_eq!(
            Solution::reverse_words("the sky is blue".to_string()),
            "blue is sky the".to_string()
        );
    }

    #[test]
    fn reverse_words_2() {
        assert_eq!(
            Solution::reverse_words("  hello world!  ".to_string()),
            "world! hello".to_string()
        );
    }

    #[test]
    fn reverse_words_3() {
        assert_eq!(
            Solution::reverse_words("a good   example".to_string()),
            "example good a".to_string()
        );
    }
}

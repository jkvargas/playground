use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s_parse = Self::string_build(s);
        let t_parse = Self::string_build(t);

        s_parse.eq(&t_parse)
    }

    fn string_build(s: String) -> String {
        let mut vec = VecDeque::<char>::new();

        for i in s.chars() {
            if i == '#' {
                vec.pop_front();
            } else {
                vec.push_front(i)
            }
        }

        let result = vec.into_iter().collect::<String>();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middle_node_1() {
        let S = "ab#c".to_string();
        let T = "ad#c".to_string();

        assert_eq!(Solution::backspace_compare(S, T), true);
    }

    #[test]
    fn middle_node_2() {
        let S = "ab##".to_string();
        let T = "c#d#".to_string();

        assert_eq!(Solution::backspace_compare(S, T), true);
    }

    #[test]
    fn middle_node_3() {
        let S = "a##c".to_string();
        let T = "#a#c".to_string();

        assert_eq!(Solution::backspace_compare(S, T), true);
    }

    #[test]
    fn middle_node_4() {
        let S = "a#c".to_string();
        let T = "b".to_string();

        assert_eq!(Solution::backspace_compare(S, T), false);
    }
}
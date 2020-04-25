use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut opening = false;
        let mut close_letter = ')';

        let mut vec_deque = VecDeque::<char>::new();

        for i in s.chars() {
            match i {
                '(' => opening = true,
                '{' => opening = true,
                '[' => opening = true,
                ')' => { close_letter = '('; opening = false; },
                ']' => { close_letter = '['; opening = false; },
                '}' => { close_letter = '{'; opening = false; },
                _ => ()
            }

            if !Self::validate_letter(i, close_letter, opening, &mut vec_deque) {
                return false;
            }
        }

        vec_deque.is_empty()
    }

    fn validate_letter(letter: char, close_letter: char, opening: bool, vec: &mut VecDeque<char>) -> bool {
        if opening {
            vec.push_front(letter);
            return true;
        }

        if vec.is_empty() {
            return false;
        }

        if let Some(result) = vec.pop_front() {
            if result != close_letter {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }

    #[test]
    fn search_2() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn search_3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }

    #[test]
    fn search_4() {
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
    }

    #[test]
    fn search_5() {
        assert_eq!(Solution::is_valid("]".to_string()), false);
    }
}

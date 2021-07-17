use std::collections::VecDeque;

struct Solution;

const MAX_LETTERS: usize = 2;
// two pointers

impl Solution {
    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        let s_vec: Vec<char> = s.chars().collect();
        if s_vec.len() == 0 {
            return 0;
        }

        let mut pos_a = 0;
        let mut pos_b = 0;
        let mut letters = VecDeque::new();
        let mut max = 1;

        letters.push_front(s_vec[0]);

        while pos_b < s_vec.len() - 1 {
            pos_b += 1;
            if !letters.contains(&s_vec[pos_b]) {
                if letters.len() == MAX_LETTERS {
                    pos_a = pos_b - 1;
                    if letters[0] == s_vec[pos_a] {
                        letters.remove(1);
                    } else {
                        letters.remove(0);
                    }

                    while pos_a - 1 >= 0 && letters.contains(&s_vec[pos_a - 1]) {
                        pos_a -= 1;
                    }
                }

                letters.push_back(s_vec[pos_b]);
            }

            max = std::cmp::max(max, pos_b - pos_a + 1);
        }

        max as i32
    }
}

#[test]
fn test_one() {
    assert_eq!(
        3,
        Solution::length_of_longest_substring_two_distinct("eceba".to_string())
    );
}

#[test]
fn test_two() {
    assert_eq!(
        5,
        Solution::length_of_longest_substring_two_distinct("ccaabbb".to_string())
    );
}

#[test]
fn test_three() {
    assert_eq!(
        1,
        Solution::length_of_longest_substring_two_distinct("a".to_string())
    );
}

#[test]
fn test_four() {
    assert_eq!(
        0,
        Solution::length_of_longest_substring_two_distinct("".to_string())
    );
}

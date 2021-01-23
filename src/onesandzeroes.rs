use std::{cmp::max, collections::HashMap};

struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut done = HashMap::new();

        Self::go_through(&strs, 0, n, m, &mut done)
    }

    fn convert_word(word: &String) -> (i32, i32) {
        let mut zeros = 0;
        let mut ones = 0;

        for i in word.chars() {
            if i == '0' {
                zeros += 1;
                continue;
            }

            if i == '1' {
                ones += 1;
            }
        }

        (zeros, ones)
    }

    fn go_through(
        strs: &Vec<String>,
        index: usize,
        max_one: i32,
        max_zero: i32,
        done: &mut HashMap<(usize, i32, i32), i32>,
    ) -> i32 {
        if index == strs.len() {
            return 0;
        }

        let key = (index, max_one, max_zero);
        if let Some(contained_key) = done.get(&key) {
            return *contained_key;
        }

        let (zeros, ones) = Self::convert_word(&strs[index]);

        let taken: i32 = if max_zero - zeros >= 0 && max_one - ones >= 0 {
            1 + Self::go_through(strs, index + 1, max_one - ones, max_zero - zeros, done)
        } else {
            -1
        };

        let not_taken = Self::go_through(strs, index + 1, max_one, max_zero, done);

        let max = max(taken, not_taken);

        done.insert(key, max);

        max
    }
}

#[test]
fn example1() {
    assert_eq!(
        4,
        Solution::find_max_form(
            vec![
                "10".to_string(),
                "0001".to_string(),
                "111001".to_string(),
                "1".to_string(),
                "0".to_string(),
            ],
            5,
            3,
        )
    );
}

#[test]
fn example2() {
    assert_eq!(
        2,
        Solution::find_max_form(
            vec!["10".to_string(), "0".to_string(), "1".to_string()],
            1,
            1,
        )
    );
}

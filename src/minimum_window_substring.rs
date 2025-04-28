// https://leetcode.com/explore/interview/card/facebook/5/array-and-strings/285/

use std::cmp::min;
use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut a = 0;
        let mut b = 0;
        let mut cur_min = i32::max_value();
        let mut overall_min = i32::max_value();
        let mut start = 0;
        let mut end = 0;

        let letters = s.chars().collect::<Vec<_>>();
        let mut map = t.chars().map(|c| (c, 0)).collect::<HashMap<char, i32>>();
        let mut shorten = false;
        while b < s.len() && !shorten {
            if shorten {
                map.entry(letters[a]).and_modify(|x| *x -= 1);
                a += 1;
                if !is_valid(&map) {
                    shorten = false;
                } else {
                    cur_min = b as i32 - a as i32;
                    if cur_min < overall_min {
                        overall_min = cur_min;
                        start = a;
                        end = b;
                    }
                }
            }
            else {
                map.entry(letters[b]).and_modify(|v| *v += 1).or_insert(1);
                b += 1;
                if is_valid(&map) {
                    shorten = true;
                    cur_min = b as i32 - a as i32;

                    if cur_min < overall_min {
                        overall_min = cur_min;
                        start = a;
                        end = b;
                    }
                }
            }
        }

        s[start..end].to_string()
    }
}

fn is_valid(map: &HashMap<char, i32>) -> bool {
    map.iter().all(|(_, &c)| c > 0)
}

#[cfg(test)]
mod tests {
    use crate::minimum_window_substring::Solution;

    #[test]
    fn example_1() {
        assert_eq!("BANC".to_string(), Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned()))
    }
}
// https://leetcode.com/problems/dota2-senate/description/?envType=study-plan-v2&envId=leetcode-75

use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut radiant = VecDeque::new();
        let mut dire = VecDeque::new();

        senate.chars().enumerate().for_each(|(i, c)| {
            match c {
                'R' => radiant.push_front(i),
                _ => { dire.push_front(i)},
            };
        });

        let mut rad_victory = false;
        let mut senate = senate.len();

        loop {
            if radiant.is_empty() ^ dire.is_empty() {
                rad_victory = !radiant.is_empty();
                break;
            }

            if radiant.back().unwrap() < dire.back().unwrap() {
                radiant.push_front(senate);
            } else {
                dire.push_front(senate);
            }
            senate += 1;

            radiant.pop_back();
            dire.pop_back();
        }

        if rad_victory {
            "Radiant".to_string()
        } else {
            "Dire".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dota_senate::Solution;

    #[test]
    fn test_one() {
        assert_eq!("Radiant", Solution::predict_party_victory("RD".to_string()));
    }

    #[test]
    fn test_two() {
        assert_eq!("Dire", Solution::predict_party_victory("RDD".to_string()));
    }

    #[test]
    fn test_three() {
        assert_eq!("Dire", Solution::predict_party_victory("DDRRR".to_string()));
    }
}
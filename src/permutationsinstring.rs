use std::collections::HashMap;

struct Solution;

const SIZE: usize = 26; // Only a-z ASCII characters.

impl Solution {
    pub fn check_inclusion_jho(s1: String, s2: String) -> bool {
        let mut letter_check = LetterCheck::new(s1.chars().collect());

        for (pos, i) in s2.chars().enumerate() {
            if letter_check.try_consume_letter(i, pos) {
                if letter_check.is_complete() {
                    return true;
                }
            }
        }

        false
    }

    // time complexity = O(s1len + (s2len - s1len))
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let idx = |c: u8| (c - 'a' as u8) as usize;

        let mut d1 = [0u8; SIZE];
        let mut d2 = [0u8; SIZE];

        for c in s1.chars() {
            d1[idx(c as u8)] += 1;
        }

        let s: &[u8] = s2.as_bytes();

        for (i, c) in s.iter().enumerate() {
            d2[idx(*c)] += 1;
            if i >= s1.len() {
                d2[idx(s[i - s1.len()])] -= 1;
            }
            if d1 == d2 {
                return true;
            }
        }

        false
    }
}

struct LetterCheck {
    needs: HashMap<char, u32>,
    has: HashMap<char, u32>,
    letters_completed: usize,
    start_pos: i32,
}

impl LetterCheck {
    pub fn new(data: Vec<char>) -> Self {
        let mut needs: HashMap<char, u32> = HashMap::new();
        let has: HashMap<char, u32> = HashMap::new();

        for i in data {
            if needs.contains_key(&i) {
                *needs.get_mut(&i).unwrap() += 1;
            } else {
                needs.insert(i, 1);
            }
        }

        Self {
            needs,
            has,
            letters_completed: 0,
            start_pos: -1,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.needs.len() == self.letters_completed
    }

    pub fn try_consume_letter(&mut self, letter: char, index: usize) -> bool {
        if self.needs.contains_key(&letter) {
            if self.has.contains_key(&letter) {
                *self.has.get_mut(&letter).unwrap() = std::cmp::min(
                    *self.has.get(&letter).unwrap() + 1,
                    *self.needs.get(&letter).unwrap(),
                );
            } else {
                self.has.insert(letter, 1);
            }

            if self.has.get(&letter).unwrap() == self.needs.get(&letter).unwrap() {
                self.letters_completed += 1;
            }
        } else {
            self.start_pos = -1;
            self.reset();
            return false;
        }

        true
    }

    fn reset(&mut self) {
        self.has.clear();
        self.letters_completed = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_inclusion_1() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
    }

    #[test]
    fn check_inclusion_2() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );
    }

    #[test]
    fn check_inclusion_3() {
        assert_eq!(
            Solution::check_inclusion("adc".to_string(), "dcda".to_string()),
            true
        );
    }

    #[test]
    fn check_inclusion_4() {
        assert_eq!(
            Solution::check_inclusion("hello".to_string(), "ooolleoooleh".to_string()),
            false
        );
    }
}

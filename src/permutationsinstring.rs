use std::collections::HashMap;

struct Solution;

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

    pub fn check_inclusion_user(s1: String, s2: String) -> bool {
        // Closure to (hopefully, elegantly) convert char into its counter position:
        let idx = |c: u8| { (c - 'a' as u8) as usize };

        // Create s1's counter:
        let mut c1 = [0u8; SIZE];
        for c in s1.chars() {
            c1[idx(c as u8)] += 1;
        }

        // Create the counter for the sliding window on s2:
        let mut c2 = [0u8; SIZE];
        // Convert to byte array for direct access and no more Option<char>:
        let s2: &[u8] = s2.as_bytes();

        // Maintain a sliding window of size len(s1) over s2.
        // If at any point, the counter c2 for the sliding window is equal to c1,
        // the counter for s1, then we have found a valid permutation.
        for i in 0..s2.len() {
            c2[idx(s2[i])] += 1;
            if i >= s1.len() - 1 {
                if c1 == c2 {
                    return true;
                }
                c2[idx(s2[i - s1.len() + 1])] -= 1;
            }
        }
        false
    }

    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1_array : Vec<char> = s1.chars().collect();
        let s2_array : Vec<char> = s2.chars().collect();

        if s1_array.len() > s2_array.len() {
            return false;
        }

        let mut s1map : Vec<usize> = vec![0; 26];
        let mut s2map : Vec<usize> = vec![0; 26];

        for i in 0..s1.len() {
            s1map[s1_array[i] as usize - 'a' as usize] += 1;
            s2map[s2_array[i] as usize - 'a' as usize] += 1;
        }

        let mut count = 0;

        for i in 0..26 {
            if s1map[i] == s2map[i] {
                count += 1;
            }
        }

        for i in 0..(s2map.len() - s1map.len()) {
            let r = s2map[i + s1map.len()] - 'a' as usize;
            let l = s2map[i] - 'a' as usize;

            if count == 26 {
                continue;
            }

            s2map[r] += 1;

            if s2map[r] == s1map[r] {
                count += 1;
            } else if s2map[r] == s1map[r] + 1 {
                count -= 1;
            }
            s2map[l] -= 1;
            if s2map[l] == s1map[l] {
                count += 1;
            } else if s2map[l] == s1map[l] - 1 {
                count -= 1;
            }
        }

        count == 26
    }
}

struct LetterCheck {
    needs: HashMap<char, u32>,
    has: HashMap<char, u32>,
    letters_completed: usize,
    start_pos: i32
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
            start_pos: -1
        }
    }

    pub fn is_complete(&self) -> bool {
        self.needs.len() == self.letters_completed
    }

    pub fn try_consume_letter(&mut self, letter: char, index: usize) -> bool {
        if self.needs.contains_key(&letter) {
            if self.has.contains_key(&letter) {
                *self.has.get_mut(&letter).unwrap() = std::cmp::min(*self.has.get(&letter).unwrap() + 1, *self.needs.get(&letter).unwrap());
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
        assert_eq!(Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()), true);
    }

    #[test]
    fn check_inclusion_2() {
        assert_eq!(Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()), false);
    }

    #[test]
    fn check_inclusion_3() {
        assert_eq!(Solution::check_inclusion("adc".to_string(), "dcda".to_string()), true);
    }

    #[test]
    fn check_inclusion_4() {
        assert_eq!(Solution::check_inclusion("hello".to_string(), "ooolleoooleh".to_string()), false);
    }
}
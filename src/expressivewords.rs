struct Solution;

impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        let (letter_s, ocurrences_s) = Self::map_word_to_ocurrences(s);
        let map_words : Vec<(Vec<char>, Vec<u32>)> = words.into_iter().map(|x| Self::map_word_to_ocurrences(x)).collect();

        let mut count = 0;
        for (letters, ocurrences) in map_words {
            if Self::compare_maps(&letter_s, &ocurrences_s, &letters, &ocurrences) {
                count += 1;
            }
        }

        count
    }

    fn compare_maps(letters_s: &Vec<char>, ocurrences_s: &Vec<u32>, letters: &Vec<char>, ocurrences: &Vec<u32>) -> bool {
        if letters.len() != letters_s.len() {
            return false;
        }

        for i in 0..letters_s.len() {
            if letters_s[i] != letters[i] {
                return false;
            }

            if ocurrences_s[i] < 3 && ocurrences[i] < ocurrences_s[i] {
                return false;
            }

            if ocurrences[i] > ocurrences_s[i] {
                return false;
            }
        }

        true
    }

    pub fn map_word_to_ocurrences(s: String) -> (Vec<char>, Vec<u32>) {
        let mut letters = Vec::new();
        let mut ocurrences = Vec::new();
        let mut last_pos = -1;

        for i in s.chars() {
            if letters.is_empty() || letters[last_pos as usize] != i {
                letters.push(i);
                ocurrences.push(1);
                last_pos += 1;
            } else {
                ocurrences[last_pos as usize] += 1;
            }
        }

        (letters, ocurrences)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn expressive_words_1() {
        assert_eq!(Solution::expressive_words("heeellooo".to_string(), vec!["hello".to_string(), "hi".to_string(), "helo".to_string()]), 1);
    }

    #[test]
    pub fn expressive_words_2() {
        assert_eq!(Solution::expressive_words("aaa".to_string(), vec!["aaaa".to_string()]), 0);
    }
}
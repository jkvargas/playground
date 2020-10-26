use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

struct Solution;

struct WordDictionary {
    by_word: HashMap<String, u32>,
    banned: HashSet<String>,
    max: u32,
    word: String,
    word_to_add: Vec<char>,
}

impl WordDictionary {
    pub fn new(banned: HashSet<String>) -> Self {
        Self {
            by_word: HashMap::new(),
            max: 0,
            banned,
            word: "".to_string(),
            word_to_add: Vec::new(),
        }
    }

    fn insert(&mut self, mut word: String) {
        word = word.to_lowercase();
        if !self.banned.contains(&word) {
            let number = self.by_word.entry(word.clone()).or_insert(0);
            *number += 1;

            if *number > self.max {
                self.word = word;
                self.max = *number;
            }
        }
    }

    pub fn parse_words_from_paragraph(&mut self, paragraph: String) {
        self.word_to_add.clear();

        for letter in paragraph.chars() {
            if letter == ' ' {
                self.add_word_if_not_empty();
            } else {
                if letter.is_alphabetic() {
                    self.word_to_add.push(letter);
                } else {
                    self.add_word_if_not_empty();
                }
            }
        }

        self.add_word_if_not_empty();
    }

    fn add_word_if_not_empty(&mut self) {
        if !self.word_to_add.is_empty() {
            self.insert(self.word_to_add.iter().collect());
            self.word_to_add.clear();
        }
    }

    pub fn get_most_word_with_most_occurrences(&self) -> String {
        self.word.clone()
    }
}

impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
        let banned_set: HashSet<String> = banned.iter().map(|x| x.clone()).collect();
        let mut dictionary = WordDictionary::new(banned_set);

        dictionary.parse_words_from_paragraph(paragraph);

        dictionary.get_most_word_with_most_occurrences()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_common_word_1() {
        assert_eq!(
            Solution::most_common_word(
                "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
                vec!["hit".to_string()]
            ),
            "ball".to_string()
        );
    }

    #[test]
    fn most_common_word_2() {
        assert_eq!(
            Solution::most_common_word("".to_string(), vec!["hit".to_string()]),
            "".to_string()
        );
    }

    #[test]
    fn most_common_word_3() {
        assert_eq!(
            Solution::most_common_word("a.".to_string(), vec![]),
            "a".to_string()
        );
    }

    #[test]
    fn most_common_word_4() {
        assert_eq!(
            Solution::most_common_word("a, a, a, a, b,b,b,c, c".to_string(), vec!["a".to_string()]),
            "b".to_string()
        );
    }
}

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, Debug)]
struct TrieNode {
    letter: char,
    is_sentence: bool,
    how_many_times_searched: i32,
    neighbors: HashSet<TrieNode>,
}

impl TrieNode {
    fn new(letter: char) -> Self {
        Self {
            letter,
            is_sentence: false,
            how_many_times_searched: 0,
            neighbors: HashSet::new(),
        }
    }
}

impl Hash for TrieNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.letter.hash(state)
    }
}

impl From<char> for TrieNode {
    fn from(letter: char) -> Self {
        TrieNode::new(letter)
    }
}

struct AutocompleteSystem {
    trie: HashSet<TrieNode>,
}

/**
 * `&self` means the method takes an immutable reference
 * If you need a mutable reference, change it to `&mut self` instead
 */

impl AutocompleteSystem {
    fn insert_word(&mut self, word: String, times: i32) {
        let mut trie_ref = &mut self.trie;

        for i in word.chars().map(|x| TrieNode::from(x)) {
            if let Some(node) = trie_ref.get(&i) {
                trie_ref = node.neighbors;
            } else {
                trie_ref.insert(i);
            }
        }
    }

    fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
        Self {
            trie: HashSet::new(),
        }
    }

    fn input(&self, c: char) -> Vec<String> {
        vec![]
    }
}

/**
 * Your AutocompleteSystem object will be instantiated and called as such:
 * let obj = AutocompleteSystem::new(sentences, times);
 * let ret_1: Vec<String> = obj.input(c);
 */
#[cfg(test)]
mod test {
    use crate::nodes::designsearchautocompletesystem::AutocompleteSystem;

    #[test]
    fn test_one() {
        let ac = AutocompleteSystem::new(
            vec![
                "i love you.".to_string(),
                "island".to_string(),
                "iroman".to_string(),
                "i love leetcode".to_string(),
            ],
            vec![5, 3, 2, 2],
        );

        assert_eq!(
            vec![
                "i love you".to_string(),
                "island".to_string(),
                "i love leetcode".to_string(),
            ],
            ac.input('i')
        );

        assert_eq!(
            vec![
                "i love you".to_string(),
                "i i love you".to_string(),
                "i love leetcode".to_string(),
            ],
            ac.input(' ')
        );

        let empty_vec: Vec<String> = vec![];

        assert_eq!(empty_vec, ac.input('a'));
        assert_eq!(empty_vec, ac.input('#'));
    }
}

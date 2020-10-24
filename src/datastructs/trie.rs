use crate::datastructs::trienode::{NodeType, TrieNode};
use crate::datastructs::{TrieError, TrieResult, TrieT};
use std::collections::HashMap;

#[derive(Default)]
struct Trie {
    children: HashMap<char, NodeType>,
}

impl TrieT for Trie {
    fn insert(&mut self, key: String) -> TrieResult<&mut NodeType> {
        if key.is_empty() {
            return TrieResult::Err(TrieError::EmptyString);
        }

        let letters: Vec<char> = key.chars().collect();

        Ok(self
            .children
            .entry(letters[0])
            .or_insert(TrieNode::from(&letters)?))
    }

    fn contains(&self, key: String) -> bool {
        if key.is_empty() {
            return false;
        }

        let letters: Vec<char> = key.chars().collect();

        if let Some(root_node) = self.children.get(&letters[0]) {
            return root_node.contains(&letters);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_works() {
        let mut trie: Trie = Default::default();

        trie.insert("jhonny".to_string());

        assert_eq!(trie.contains("jhonny".to_string()), true);
    }

    #[test]
    fn does_not_contain() {
        let mut trie: Trie = Default::default();

        trie.insert("jhonny".to_string());

        assert_eq!(trie.contains("jhonn".to_string()), false);
    }
}

use crate::datastructs::{TrieError, TrieResult, TrieT};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

pub type NodeType = Box<TrieNode>;

pub struct TrieNode {
    is_leaf: bool,
    letter: char,
    children: HashMap<char, NodeType>,
}

impl Hash for TrieNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.letter.hash(state);
    }
}

impl TrieNode {
    #[inline]
    pub fn new(letter: &char) -> Self {
        Self {
            is_leaf: false,
            children: HashMap::new(),
            letter: letter.clone(),
        }
    }

    #[inline]
    pub fn set_as_leaf(&mut self) {
        self.is_leaf = true;
    }

    pub fn from(letters: &[char]) -> TrieResult<NodeType> {
        if letters.is_empty() {
            return TrieResult::Err(TrieError::EmptyString);
        }

        let mut root: NodeType = Box::new(TrieNode::new(&letters[0]));

        let mut node_ref = &mut root;
        for i in 1..letters.len() {
            node_ref = node_ref.get_or_create_child(&letters[i]);
        }

        node_ref.set_as_leaf();

        Ok(root)
    }

    pub fn insert(&mut self, letters: &[char]) -> TrieResult<()> {
        if letters.is_empty() {
            return TrieResult::Err(TrieError::EmptyString);
        }

        let mut node_ref = self;
        for i in 1..letters.len() {
            node_ref = node_ref.get_or_create_child(&letters[i]);
        }

        node_ref.set_as_leaf();

        Ok(())
    }

    pub fn contains(&self, letters: &[char]) -> bool {
        let mut node_ref = self;

        if self.letter != letters[0] {
            return false;
        }

        for i in 1..letters.len() {
            if !node_ref.children.contains_key(&letters[i]) {
                return false;
            }

            node_ref = node_ref.children.get(&letters[i]).unwrap();
        }

        node_ref.is_leaf
    }

    pub fn get_or_create_child(&mut self, key: &char) -> &mut NodeType {
        let node = Box::new(TrieNode::new(key));
        self.children.entry(*key).or_insert(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_works() {
        let some_strings = vec![
            vec!['j', 'h', 'o', 'n', 'n', 'y'],
            vec!['j', 'h', 'o', 'n'],
            vec!['j', 'o', 'a', 'o'],
        ];

        let mut node = TrieNode::from(&some_strings[0]).unwrap();

        node.insert(&some_strings[1]);
        node.insert(&some_strings[2]);

        for i in 0..some_strings.len() {
            assert_eq!(node.contains(&some_strings[i]), true);
        }
    }

    #[test]
    fn not_contains_works() {
        let does_not_contain = vec!['p', 'e', 'd', 'r', 'o'];

        let contain = vec!['j', 'h', 'o', 'n', 'n', 'y'];

        let node = TrieNode::from(&contain).unwrap();

        assert_eq!(node.contains(&does_not_contain), false);
    }
}

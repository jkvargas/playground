use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

pub type NodeType<TValue> = Rc<RefCell<Box<TrieNode<TValue>>>>;

pub trait Trie<TValue> {
    fn insert(&mut self, value: Vec<TValue>);

    fn contains(&self, value: &Vec<TValue>) -> bool;
}

pub struct TrieNode<TValue>
where
    TValue: Hash,
    TValue: Eq,
    TValue: Copy,
{
    value: TValue,
    is_leaf: bool,
    children: HashMap<TValue, NodeType<TValue>>,
}

impl<TValue> Hash for TrieNode<TValue>
where
    TValue: Hash,
    TValue: Eq,
    TValue: Copy,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<TValue> TrieNode<TValue>
where
    TValue: Hash,
    TValue: Eq,
    TValue: Copy,
{
    fn new(value: TValue, is_leaf: bool) -> Self {
        Self {
            value,
            is_leaf,
            children: HashMap::new(),
        }
    }

    pub fn set_as_leaf(&mut self) {
        self.is_leaf = true;
    }

    pub fn create_node(value: TValue, is_leaf: bool) -> NodeType<TValue> {
        Rc::new(RefCell::new((Box::new(TrieNode::new(value, is_leaf)))))
    }
}

pub struct DefaultTrie<TValue>
where
    TValue: Hash,
    TValue: Eq,
    TValue: Copy,
{
    children: HashMap<TValue, NodeType<TValue>>,
}

impl<TValue> DefaultTrie<TValue>
where
    TValue: Hash,
    TValue: Eq,
    TValue: Copy,
{
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }
}

impl<TValue> Trie<TValue> for DefaultTrie<TValue>
where
    TValue: Hash,
    TValue: Eq,
    TValue: Copy,
{
    fn insert(&mut self, value: Vec<TValue>) {
        let mut reference = &mut self.children;

        let last = value.last().unwrap();

        for item in &value {
            let eq_last = item.eq(last);

            if reference.contains_key(item) {
                let mut borrow = reference.get_mut(item).unwrap().borrow_mut();

                if eq_last {
                    borrow.set_as_leaf();
                }

                reference = &mut borrow.children;
            } else {
                let node = TrieNode::create_node(item.clone(), eq_last);
                reference.insert(*item, node);
            }
        }
    }

    fn contains(&self, value: &Vec<TValue>) -> bool {
        let mut reference = &self.children;

        for item in value {
            if reference.contains_key(&item) {
                let borrow = reference.get(&item).unwrap().borrow();
                reference = &mut borrow.children;
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::exit;

    #[test]
    pub fn trie_1() {
        let temp = String::from("jhonny");

        let mut trie = DefaultTrie::new();

        trie.insert(temp.chars().collect());

        let expect = String::from("jhonny").chars().collect();

        assert_eq!(true, trie.contains(expect));
    }
}

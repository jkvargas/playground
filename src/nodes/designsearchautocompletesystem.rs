// use std::{cell::RefCell, rc::Rc};
//
// /// https://leetcode.com/problems/design-search-autocomplete-system/
// use std::{
//     cmp::Ordering,
//     collections::{BinaryHeap, HashMap, HashSet},
//     hash::{Hash, Hasher},
// };
//
// type Trie = HashMap<char, TrieNode>;
//
// #[derive(Eq, PartialEq, Debug)]
// struct TrieNode {
//     number_of_occurrences: i32,
//     is_sentence: bool,
//     neighbors: Rc<RefCell<Trie>>,
// }
//
// impl TrieNode {
//     fn new(number_of_occurrences: i32, is_sentence: bool) -> TrieNode {
//         Self {
//             neighbors: Rc::new(RefCell::new(HashMap::new())),
//             number_of_occurrences,
//             is_sentence,
//         }
//     }
// }
//
// struct Statement {
//     occurrence: String,
//     times: i32,
// }
//
// impl Statement {
//     fn new(occurrence: String, times: i32) -> Self {
//         Self { occurrence, times }
//     }
// }
//
// impl Eq for Statement {}
//
// impl PartialEq<Self> for Statement {
//     fn eq(&self, other: &Self) -> bool {
//         self.occurrence.eq(&other.occurrence)
//     }
// }
//
// impl PartialOrd<Self> for Statement {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }
//
// impl Ord for Statement {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.times.cmp(&other.times)
//     }
// }
//
// struct AutocompleteSystem {
//     trie: Rc<RefCell<Trie>>,
//     sentence: Vec<char>,
//     last_trie: Option<Rc<RefCell<Trie>>>,
//     sentences_so_far: BinaryHeap<Statement>,
// }
//
// /**
//  * `&self` means the method takes an immutable reference
//  * If you need a mutable reference, change it to `&mut self` instead
//  */
//
// impl AutocompleteSystem {
//     fn insert_word(&mut self, word: &String, times: i32) {
//         let mut trie_ref = self.trie.clone();
//         let letters: Vec<char> = word.chars().collect();
//
//         for (ind, i) in letters.iter().enumerate() {
//             let mut borrow_mut = self.trie.borrow_mut();
//             if let Some(node) = borrow_mut.get_mut(&i) {
//                 if letters.len() - 1 == ind {
//                     node.is_sentence = true;
//                     node.number_of_occurrences = times;
//                 }
//
//                 trie_ref = node.neighbors.clone();
//             } else {
//                 if letters.len() - 1 == ind {
//                     trie_ref.borrow_mut().insert(*i, TrieNode::new(times, true));
//                 } else {
//                     trie_ref.borrow_mut().insert(*i, TrieNode::new(0, false));
//                 }
//             }
//         }
//     }
//
//     fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
//         let mut result = Self {
//             trie: Rc::new(RefCell::new(HashMap::new())),
//             sentence: Vec::new(),
//             last_trie: None,
//             sentences_so_far: BinaryHeap::new(),
//         };
//
//         for (ind, word) in sentences.iter().enumerate() {
//             result.insert_word(word, times[ind]);
//         }
//
//         result
//     }
//
//     fn input(&mut self, c: char) -> Vec<String> {
//         self.add_to_sentence(c);
//
//         if self.sentence.is_empty() {
//             vec![]
//         } else {
//             let mut result = Vec::new();
//             self.set_last_node(c);
//             if let Some(to_borrow) = self.last_trie.as_ref() {
//                 let lt = to_borrow.borrow();
//                 Self::get_words(&lt, self.sentence.clone(), &mut result);
//
//                 result
//             } else {
//                 vec![]
//             }
//         }
//     }
//
//     fn get_words(trie: &Trie, letters: Vec<char>, result: &mut Vec<String>) {
//         if trie.is_empty() {
//             result.push(letters.iter().collect::<String>());
//             return;
//         }
//
//         for (letter, node) in trie.iter() {
//             let mut with_new_letter = letters.clone();
//             with_new_letter.push(*letter);
//
//             let bor = node.neighbors.borrow();
//
//             Self::get_words(&bor, with_new_letter, result);
//         }
//     }
//
//     fn add_to_sentence(&mut self, c: char) {
//         if c == '#' {
//             let word = self.sentence.iter().cloned().collect::<String>();
//             self.insert_word(&word, 1);
//             self.sentence.clear();
//             return;
//         }
//
//         self.sentence.push(c);
//     }
//
//     fn set_last_node(&mut self, c: char) {
//         let result = self.last_trie.as_ref().map_or_else(
//             || {
//                 self.trie
//                     .borrow()
//                     .get(&c)
//                     .map_or_else(|| None, |y| Some(y.neighbors.clone()))
//             },
//             |x| {
//                 x.borrow()
//                     .get(&c)
//                     .map_or_else(|| None, |y| Some(y.neighbors.clone()))
//             },
//         );
//
//         self.last_trie = result;
//     }
// }
//
// /**
//  * Your AutocompleteSystem object will be instantiated and called as such:
//  * let obj = AutocompleteSystem::new(sentences, times);
//  * let ret_1: Vec<String> = obj.input(c);
//  */
// #[cfg(test)]
// mod test {
//     use crate::nodes::designsearchautocompletesystem::AutocompleteSystem;
//
//     #[test]
//     fn test_one() {
//         let mut ac = AutocompleteSystem::new(
//             vec![
//                 "i love you.".to_string(),
//                 "island".to_string(),
//                 "iroman".to_string(),
//                 "i love leetcode".to_string(),
//             ],
//             vec![5, 3, 2, 2],
//         );
//
//         assert_eq!(
//             vec![
//                 "i love you".to_string(),
//                 "island".to_string(),
//                 "i love leetcode".to_string(),
//             ],
//             ac.input('i')
//         );
//
//         assert_eq!(
//             vec![
//                 "i love you".to_string(),
//                 "i i love you".to_string(),
//                 "i love leetcode".to_string(),
//             ],
//             ac.input(' ')
//         );
//
//         let empty_vec: Vec<String> = vec![];
//
//         assert_eq!(empty_vec, ac.input('a'));
//         assert_eq!(empty_vec, ac.input('#'));
//     }
// }

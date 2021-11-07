// /// https://leetcode.com/problems/design-search-autocomplete-system/
// use std::{
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
//     neighbors: Trie,
// }
//
// impl TrieNode {
//     fn new(number_of_occurrences: i32, is_sentence: bool) -> TrieNode {
//         Self {
//             neighbors: HashMap::new(),
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
// struct AutocompleteSystem<'a> {
//     trie: Trie,
//     sentence: Vec<char>,
//     last_trie: Option<&'a Trie>,
//     sentences_so_far: BinaryHeap<Statement>,
// }
//
// /**
//  * `&self` means the method takes an immutable reference
//  * If you need a mutable reference, change it to `&mut self` instead
//  */
//
// impl<'a> AutocompleteSystem {
//     fn insert_word(&mut self, word: &String, times: i32) {
//         let mut trie_ref = &mut self.trie;
//         let letters: Vec<char> = word.chars().collect();
//
//         for (ind, i) in letters.iter().enumerate() {
//             if let Some(node) = trie_ref.get_mut(&i) {
//                 if letters.len() - 1 == ind {
//                     node.is_sentence = true;
//                     node.number_of_occurrences = times;
//                 }
//
//                 trie_ref = &mut node.neighbors;
//             } else {
//                 if letters.len() - 1 == ind {
//                     trie_ref.insert(*i, TrieNode::new(times, true));
//                 } else {
//                     trie_ref.insert(*i, TrieNode::new(0, false));
//                 }
//             }
//         }
//     }
//
//     fn new(sentences: Vec<String>, times: Vec<i32>) -> Self {
//         let mut result = Self {
//             trie: HashMap::new(),
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
//             if let Some(lt) = self.last_trie {
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
//             result.push(letters.into());
//             return;
//         }
//
//         for (letter, node) in trie.iter() {
//             let mut with_new_letter = letters.clone();
//             with_new_letter.push(*letter);
//
//             Self::get_words(&node.neighbors, with_new_letter, result);
//         }
//     }
//
//     fn add_to_sentence(&mut self, c: char) {
//         if c == '#' {
//             self.insert_word(self.sentence.into(), 1);
//             self.sentence.clear();
//             return;
//         }
//
//         self.sentence.push(c);
//     }
//
//     fn set_last_node(&mut self, c: char) {
//         if let Some(lt) = self.last_trie {
//             if let Some(node) = lt.get(&c) {
//                 self.last_trie = Some(&node.neighbors);
//             } else {
//                 self.last_trie = None;
//             }
//         } else {
//             if let Some(node) = self.trie.get(&c) {
//                 self.last_trie = Some(&node.neighbors);
//             }
//         }
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

use std::collections::{BinaryHeap, HashMap};

struct Solution;

// https://leetcode.com/problems/reorganize-string/

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    count: i32,
    letter: char,
}

impl Item {
    fn new(count: i32, letter: char) -> Self {
        Self { count, letter }
    }

    fn append_letter(&mut self, result: &mut Vec<char>) -> bool {
        self.count -= 1;
        if let Some(&l) = result.last() {
            if l == self.letter {
                return false;
            }
        }
        result.push(self.letter);
        true
    }
}

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut char_map: HashMap<char, i32> = HashMap::new();
        let mut queue = BinaryHeap::new();
        let char_array: Vec<char> = s.chars().collect();
        let mut result = Vec::new();

        char_array
            .iter()
            .for_each(|&x| *char_map.entry(x).or_insert(0) += 1);
        char_map
            .into_iter()
            .for_each(|(letter, count)| queue.push(Item::new(count, letter)));

        while let Some(mut item) = queue.pop() {
            if !item.append_letter(&mut result) {
                return "".to_string();
            }
            if let Some(mut sec) = queue.pop() {
                if !sec.append_letter(&mut result) {
                    return "".to_string();
                }
                if sec.count > 0 {
                    queue.push(sec);
                }
            }
            if item.count > 0 {
                queue.push(item);
            }
        }

        result.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use crate::reorganizestrings::Solution;

    #[test]
    fn test_one() {
        assert_eq!(
            "aba".to_string(),
            Solution::reorganize_string("aab".to_string())
        );
    }

    #[test]
    fn test_two() {
        assert_eq!(
            String::new(),
            Solution::reorganize_string("aaab".to_string())
        );
    }
}

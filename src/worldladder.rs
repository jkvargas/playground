use std::cmp::min;
use std::collections::{HashMap, HashSet, VecDeque};

struct Solution;

type WordLetterList = Vec<char>;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut hm: HashMap<String, Vec<String>> = HashMap::new();
        for word in word_list.iter() {
            for i in 0..word.len() {
                let s = word.as_str();
                let key = (&s[0..i]).to_string() + "*" + &s[i + 1..];
                if let Some(v) = hm.get_mut(&key) {
                    v.push(s.to_string());
                } else {
                    hm.insert(key, vec![s.to_string()]);
                }
            }
        }

        dbg!(&hm);

        let mut hs: HashSet<String> = HashSet::new();
        let mut vq: VecDeque<(String, i32)> = VecDeque::new();
        hs.insert(begin_word.to_string());
        vq.push_back((begin_word, 1));
        while let Some(front) = vq.pop_front() {
            let s = front.0.as_str();
            for i in 0..s.len() {
                let key = (&s[0..i]).to_string() + "*" + &s[i + 1..];
                if let Some(v) = hm.get(&key) {
                    for next in v.iter() {
                        if *next == end_word {
                            return front.1 + 1;
                        }
                        if !hs.contains(next) {
                            hs.insert(next.to_string());
                            vq.push_back((next.to_string(), front.1 + 1));
                        }
                    }
                }
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exist_1() {
        assert_eq!(
            Solution::ladder_length(
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string(),
                ],
            ),
            5
        );
    }
}

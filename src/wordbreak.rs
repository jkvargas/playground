use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let word: Vec<char> = s.chars().collect();
        let mut memo: Vec<Option<bool>> = vec![None; word.len()];

        let mut map: HashMap<char, Vec<Vec<char>>> = HashMap::new();

        for wdic in word_dict {
            let i: Vec<char> = wdic.chars().collect();

            if !map.contains_key(&i[0]) {
                map.insert(i[0], Vec::new());
            }

            map.get_mut(&i[0]).unwrap().push(i);
        }

        Self::wb(word.len(), 0, &word, &map, &mut memo)
    }

    pub fn work_break_bfs(s: String, word_dict: Vec<String>) -> bool {
        let hs: HashSet<String> = HashSet::from_iter(word_dict);
        let mut queue: VecDeque<i32> = VecDeque::new();
        let mut visited: Vec<bool> = vec![false; s.len()];
        queue.push_back(0);

        while !queue.is_empty() {
            let start = queue.pop_front().unwrap();

            if !visited[start as usize] {
                let mut end = start + 1;

                while end < s.len() as i32 {
                    if hs.contains(&s[start as usize..end as usize]) {
                        queue.push_back(end);
                        if end == s.len() as i32 {
                            return true;
                        }
                    }

                    end += 1;
                }
            }

            visited[start as usize] = true;
        }

        false
    }

    fn contained(mut pos: usize, word: &Vec<char>, vec: &Vec<char>) -> bool {
        if word.len() - pos < vec.len() {
            return false;
        }

        for letter in vec {
            if word[pos] == *letter {
                pos += 1;
            } else {
                return false;
            }
        }

        true
    }

    fn wb(
        wlen: usize,
        pos: usize,
        word: &Vec<char>,
        map: &HashMap<char, Vec<Vec<char>>>,
        memo: &mut Vec<Option<bool>>,
    ) -> bool {
        if pos == wlen {
            return true;
        }

        if memo[pos].is_some() {
            return memo[pos].unwrap();
        }

        if !map.contains_key(&word[pos]) {
            return false;
        }

        for test in map.get(&word[pos]).unwrap() {
            if Self::contained(pos, word, test) {
                let ret = Self::wb(wlen, pos + test.len(), word, map, memo);
                memo[pos] = Some(true);

                if ret {
                    return true;
                }
            }
        }

        memo[pos] = Some(false);
        false
    }
}

// memoization = holds results

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_break_1() {
        assert!(Solution::word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ));
    }

    #[test]
    fn word_break_2() {
        assert!(Solution::word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()]
        ));
    }

    #[test]
    fn word_break_3() {
        assert_eq!(
            Solution::word_break(
                "catsandog".to_string(),
                vec![
                    "cats".to_string(),
                    "dog".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "cat".to_string()
                ]
            ),
            false
        );
    }

    #[test]
    fn word_break_4() {
        assert_eq!(
            Solution::word_break(
                "aaaaaaa".to_string(),
                vec!["aaaa".to_string(), "aa".to_string()]
            ),
            false
        );
    }

    #[test]
    fn word_break_6() {
        assert_eq!(
            Solution::work_break_bfs(
                "catsanddog".to_string(),
                vec![
                    "cat".to_string(),
                    "cats".to_string(),
                    "sand".to_string(),
                    "and".to_string(),
                    "dog".to_string()
                ]
            ),
            false
        );
    }

    #[test]
    fn word_break_5() {
        assert_eq!(Solution::word_break("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(),
     vec!["a".to_string(),
                    "aa".to_string(),
                    "aaa".to_string(),
                    "aaaa".to_string(),
                    "aaaaa".to_string(),
                    "aaaaaa".to_string(),
                    "aaaaaaa".to_string(),
                    "aaaaaaaa".to_string(),
                    "aaaaaaaaa".to_string(),
                    "aaaaaaaaaa".to_string()]), false);
    }
}

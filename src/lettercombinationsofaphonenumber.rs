use std::collections::HashMap;
use std::ops::Add;

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut map = HashMap::new();
        map.insert('0', Vec::new());
        map.insert('1', Vec::new());
        map.insert('2', vec!['a', 'b', 'c']);
        map.insert('3', vec!['d', 'e', 'f']);
        map.insert('4', vec!['g', 'h', 'i']);
        map.insert('5', vec!['j', 'k', 'l']);
        map.insert('6', vec!['m', 'n', 'o']);
        map.insert('7', vec!['p', 'q', 'r', 's']);
        map.insert('8', vec!['t', 'u', 'v']);
        map.insert('9', vec!['w', 'x', 'y', 'z']);

        let mut numbers: Vec<char> = digits.chars().collect();

        let mut first = true;

        let mut ans = Vec::new();

        while !numbers.is_empty() {
            let n = numbers.pop().unwrap();

            if first {
                first = false;
                ans = map[&n].iter().map(|x| x.to_string()).collect()
            } else {
                let mut v: Vec<String> = Vec::new();
                for i in &map[&n] {
                    for j in &ans {
                        v.push(i.to_string().add(&j));
                    }
                }
                ans = v;
            }
        }

        ans
    }
}

#[test]
fn test1() {
    assert_eq!(
        Solution::letter_combinations("8".to_string()),
        vec!["t".to_string(), "u".to_string(), "v".to_string()]
    );
}

#[test]
fn test2() {
    assert_eq!(
        Solution::letter_combinations("23".to_string()),
        vec![
            "ad".to_string(),
            "ae".to_string(),
            "af".to_string(),
            "bd".to_string(),
            "be".to_string(),
            "bf".to_string(),
            "cd".to_string(),
            "ce".to_string(),
            "cf".to_string()
        ]
    );
}

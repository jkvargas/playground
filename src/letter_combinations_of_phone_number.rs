use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![]
        }

        let numbers = digits.chars().collect::<Vec<char>>();

        let map = HashMap::from([
            ('2', vec!['a', 'b', 'c']),
            ('3', vec!['d', 'e', 'f']),
            ('4', vec!['g', 'h', 'i']),
            ('5', vec!['j', 'k', 'l']),
            ('6', vec!['m', 'n', 'o']),
            ('7', vec!['p', 'q', 'r', 's']),
            ('8', vec!['t', 'u', 'v']),
            ('9', vec!['w', 'x', 'y', 'z']),
        ]);

        let mut result = Vec::new();

        bt(0, &numbers, &map, &mut Vec::new(), &mut result);

        result
    }
}

fn bt(
    pos: usize,
    numbers: &Vec<char>,
    map: &HashMap<char, Vec<char>>,
    combination: &mut Vec<char>,
    results: &mut Vec<String>,
) {
    if pos == numbers.len() {
        results.push(combination.iter().collect());
        return;
    }

    let letters = &map[&numbers[pos]];
    for letter in letters {
        combination.push(letter.clone());
        bt(pos + 1, numbers, map, combination, results);
        combination.pop();
    }
}

#[test]
fn test_one() {
    assert_eq!(
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
        ],
        Solution::letter_combinations("23".to_string())
    );
}

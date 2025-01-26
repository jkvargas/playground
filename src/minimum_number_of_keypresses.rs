use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn minimum_keypresses(s: String) -> i32 {
        let word = s.chars().collect::<Vec<char>>();
        let mut occurrence = HashMap::new();

        word.iter().for_each(|x| *occurrence.entry(x).or_insert(0) += 1);
        let mut letters = occurrence.keys().map(|&&x| x).collect::<Vec<char>>();
        letters.sort_by(|a, b| occurrence[b].cmp(&occurrence[a]));
        let mut position_map = HashMap::new();
        for (ind, &character) in letters.iter().enumerate() {
            position_map.insert(character, ind as i32 + 1);
        }

        let mut number_hits = 0;
        for l in &word {
            number_hits += position_map[l] / 9 + if position_map[l] % 9 > 0 { 1 } else { 0 };
        }

        number_hits
    }
}

// best

// impl Solution {
//     pub fn minimum_keypresses(s: String) -> i32 {
//         let s = s.chars();
//         let mut count = vec![0; 26];
//         for x in s {
//             count[x as usize - 'a' as usize] += 1;
//         }
//         count.sort();
//         count.reverse();
//
//         let mut cost = 1;
//         let mut result = 0;
//         let mut cnt = 0;
//         for x in count {
//             result += x * cost;
//             cnt += 1;
//             if cnt >= 9 {
//                 cnt = 0;
//                 cost += 1;
//             }
//         }
//         result
//     }
// }

#[cfg(test)]
mod tests {
    use crate::minimum_number_of_keypresses::Solution;

    #[test]
    fn test() {
        assert_eq!(15, Solution::minimum_keypresses("abcdefghijkl".to_string()));
    }
}
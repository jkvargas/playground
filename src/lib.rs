use std::collections::HashMap;

mod backspacecompare;
mod minstack;
mod nodes;
mod laststone;
mod contiguousarray;
mod productarray;
mod kthlargest;
mod battleships;
mod minimumpathsum;
mod threesum;
mod rotatedsortedarray;
mod validparenthesis;
mod besttimetobuyandsellstock;
mod rottingoranges;
mod longestpalyndromic;
mod countcompletetreenodes;
mod kclosestpoint;
mod lrucache;
mod longestsubstringwithoutrepat;
mod gameoflife;
mod countprimes;
mod coinchange;
mod reversedlinkedlist;
mod reverseint;
mod trackpath;
mod threesumclosest;
mod nextpermutation;
mod insertdeleterandom;
mod uniquepaths;
mod courseschedule;
mod coursescheduleii;
mod wordsearch;
mod worldladder;
mod cracking;
mod decodestring;

pub struct Leetcode;

impl Leetcode {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut letters_map = HashMap::<char, i32>::new();
        letters_map.insert('a', 2);
        letters_map.insert('b', 3);
        letters_map.insert('c', 5);
        letters_map.insert('d', 7);
        letters_map.insert('e', 11);
        letters_map.insert('f', 13);
        letters_map.insert('g', 17);
        letters_map.insert('h', 19);
        letters_map.insert('i', 23);
        letters_map.insert('j', 29);
        letters_map.insert('k', 31);
        letters_map.insert('l', 37);
        letters_map.insert('m', 41);
        letters_map.insert('n', 43);
        letters_map.insert('o', 47);
        letters_map.insert('p', 53);
        letters_map.insert('q', 59);
        letters_map.insert('r', 61);
        letters_map.insert('s', 67);
        letters_map.insert('t', 71);
        letters_map.insert('u', 73);
        letters_map.insert('v', 79);
        letters_map.insert('w', 83);
        letters_map.insert('x', 89);
        letters_map.insert('y', 97);
        letters_map.insert('z', 101);

        let mut hash = HashMap::<i32, Vec<String>>::new();

        for i in strs {
            let hashed = i.chars().fold(1, |acc, x| acc * letters_map[&x]);

            if hash.contains_key(&hashed) {
                hash.get_mut(&hashed).unwrap().push(i);
            } else {
                let mut new_vec = Vec::<String>::new();
                new_vec.push(i);
                hash.insert(hashed, new_vec);
            }
        }

        // let mut result: Vec<Vec<String>> = hash.into_iter().map(|(_key, mut val)|
        //     {
        //         val.sort();
        //         val
        //     }).collect();
        //
        // result.sort_by(|a, b| a.len().cmp(&b.len()).reverse());

        hash.into_iter().map(|(_key, val)| val).collect()
    }

    pub fn count_elements(mut arr: Vec<i32>) -> i32 {
        let mut hash = HashMap::<i32, i32>::new();

        arr.sort();
        hash.insert(arr[0], 1);

        let mut count = 0;
        for i in 1..arr.len() {
            if hash.contains_key(&(arr[i] - 1)) {
                count += hash.get(&(arr[i] - 1)).unwrap();
                *hash.get_mut(&(arr[i] - 1)).unwrap() = 0;
            }

            if !hash.contains_key(&arr[i]) {
                hash.insert(arr[i], 1);
            }
            else {
                *hash.get_mut(&arr[i]).unwrap() += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::Leetcode;

    fn get_string_param(list: Vec<&str>) -> Vec<String> {
        list.into_iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn count_elements_1() {
        let result = Leetcode::count_elements(vec![1, 2, 3]);

        assert_eq!(result, 2);
    }

    #[test]
    fn count_elements_6() {
        let result = Leetcode::count_elements(vec![1,1,2]);

        assert_eq!(result, 2);
    }

    #[test]
    fn count_elements_2() {
        let result = Leetcode::count_elements(vec![1, 1, 3, 3, 5, 5, 7, 7]);

        assert_eq!(result, 0);
    }

    #[test]
    fn count_elements_3() {
        let result = Leetcode::count_elements(vec![1, 3, 2, 3, 5, 0]);

        assert_eq!(result, 3);
    }

    #[test]
    fn count_elements_4() {
        let result = Leetcode::count_elements(vec![1, 1, 2, 2]);

        assert_eq!(result, 2);
    }

    #[test]
    fn count_elements_5() {
        let result = Leetcode::count_elements(vec![1,3,2,3,5,0]);

        assert_eq!(result, 3);
    }

    #[test]
    fn group_anagram_1() {
        let output = vec![
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["bat".to_string()],
        ];

        assert_eq!(
            Leetcode::group_anagrams(vec![
                "eat".to_string(),
                "tea".to_string(),
                "tan".to_string(),
                "ate".to_string(),
                "nat".to_string(),
                "bat".to_string()
            ]),
            output
        );
    }

    #[test]
    fn group_anagram_2() {
        let intro = get_string_param(vec![
            "cab", "tin", "pew", "duh", "may", "ill", "buy", "bar", "max", "doc",
        ]);
        assert_eq!(
            Leetcode::group_anagrams(intro),
            vec![
                vec!["doc".to_string()],
                vec!["bar".to_string()],
                vec!["buy".to_string()],
                vec!["ill".to_string()],
                vec!["may".to_string()],
                vec!["tin".to_string()],
                vec!["cab".to_string()],
                vec!["pew".to_string()],
                vec!["max".to_string()],
                vec!["duh".to_string()]
            ]
        );
    }
}

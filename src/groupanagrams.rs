use std::collections::HashMap;

static LETTERS: &[(char, i32)] = &[
    ('a', 2),
    ('b', 3),
    ('c', 5),
    ('d', 7),
    ('e', 11),
    ('f', 13),
    ('g', 17),
    ('h', 19),
    ('i', 23),
    ('j', 29),
    ('k', 31),
    ('l', 37),
    ('m', 41),
    ('n', 43),
    ('o', 47),
    ('p', 53),
    ('q', 59),
    ('r', 61),
    ('s', 67),
    ('t', 71),
    ('u', 73),
    ('v', 79),
    ('w', 83),
    ('x', 89),
    ('y', 97),
    ('z', 101),
];

struct Solution;

impl Solution {
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

        hash.into_iter().map(|(_key, mut val)| val).collect()
    }
}

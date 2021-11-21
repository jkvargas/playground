use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let s_map = Self::to_map(s);
        let t_map = Self::to_map(t);

        s_map.eq(&t_map)
    }

    fn to_map(s: String) -> HashMap<char, u32> {
        let mut set = HashMap::new();

        for x in s.chars() {
            if set.contains_key(&x) {
                *set.get_mut(&x).unwrap() += 1;
            } else {
                set.insert(x, 1);
            }
        }

        set
    }
}

#[test]
fn test_one() {
    assert!(Solution::is_anagram(
        "anagram".to_string(),
        "nagaram".to_string()
    ));
}

#[test]
fn test_two() {
    assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
}

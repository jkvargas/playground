struct Solution;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let mut chars_type: Vec<char> = s.chars().collect();
        chars_type.sort();

        let mut split = String::new();
        let mut words = Vec::<String>::new();
        let mut new_word = true;

        for (ind, val) in chars_type.iter().enumerate() {
            if new_word {
                split.push(*val);
                new_word = false;
            } else {
                if chars_type[ind - 1] == *val {
                    split.push(*val);
                } else {
                    words.push(split);
                    split = String::new();
                    split.push(*val);
                }
            }
        }

        words.push(split);

        words.sort_by(|a, b| a.len().cmp(&b.len()).reverse());
        words.join("")
    }
}

#[cfg(test)]
mod test {
    use crate::sortbyfrequency::Solution;

    #[test]
    pub fn test_one() {
        assert_eq!(
            "eert".to_string(),
            Solution::frequency_sort("tree".to_string())
        );
    }
}

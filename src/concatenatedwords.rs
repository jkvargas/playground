struct Solution;

impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        vec!["oi".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_all_concatenated_words_in_a_dict() {
        let input = vec![
            "cat".to_string(),
            "cats".to_string(),
            "catsdogcats".to_string(),
            "dog".to_string(),
            "dogcatsdog".to_string(),
            "hippopotamuses".to_string(),
            "rat".to_string(),
            "ratcatdogcat".to_string(),
        ];

        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(input),
            vec![
                "catsdogcats".to_string(),
                "dogcatsdog".to_string(),
                "ratcatdogcat".to_string()
            ]
        );
    }
}

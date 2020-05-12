struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exist_1() {
        assert_eq!(Solution::ladder_length("hit".to_string(),
                                           "cog".to_string(),
                                           vec!["hot".to_string(),
                                                "dot".to_string(),
                                                "dog".to_string(),
                                                "lot".to_string(),
                                                "log".to_string(),
                                                "cog".to_string()]), 5);
    }
}
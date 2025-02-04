struct Solution;

impl Solution {
    pub fn words_typing(sentence: Vec<String>, rows: i32, cols: i32) -> i32 {
        let s = (sentence.join(" ") + " ").chars().collect::<Vec<char>>();
        let l = s.len();
        let mut start = 0;

        for _ in 0..rows {
            start += cols as usize;

            while s[start % l] != ' ' {
                if start == 0 {
                    break;
                }
                start -= 1;
            }

            start += 1;
        }

        (start / l) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::sentence_screen_fitting::Solution;

    #[test]
    fn it_works() {
        assert_eq!(1, Solution::words_typing(vec!["hello".to_string(),"world".to_string()], 2, 8));
    }

    #[test]
    fn it_works2() {
        assert_eq!(0, Solution::words_typing(vec!["hello".to_string()], 1000, 1));
    }
}

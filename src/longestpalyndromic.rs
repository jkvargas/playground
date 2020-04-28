pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }

        let letters = s.chars().collect::<Vec<char>>();
        let vec_size = letters.len();
        let mut result : String = "".to_string();
        let size_end = 0;

        for i in 0..vec_size {
            let slice = Self::search_on_string(&letters, vec_size, i);

            let temp = if slice.0 == slice.1 {
                letters[slice.0].to_string()
            } else {
                (&s[slice.0..slice.1 + 1]).to_string()
            };

            if temp.len() > result.len() {
                result = temp;
            }
        }

        result
    }

    fn search_on_string(letters: &[char], vec_size: usize, mut begin: usize) -> (usize, usize) {
        let mut equal_letters = true;
        let mut end = begin;

        'main_loop: loop {
            if begin == 0 && end == vec_size - 1 {
                return (begin, end);
            }

            let original_end = end;
            let original_begin = begin;

            if end < vec_size - 1 && equal_letters {
                while equal_letters {
                    end += 1;
                    if letters[begin] != letters[end] {
                        end -= 1;
                        equal_letters = false;
                        continue 'main_loop;
                    }

                    if end == vec_size - 1 {
                        return (begin, end);
                    }
                }
            } else if end < vec_size - 1 && begin > 0 {
                begin -= 1;
                end += 1;
            } else {
                break;
            }

            if letters[begin] != letters[end] {
                begin = original_begin;
                end = original_end;

                break;
            }
        }

        (begin, end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_palindrome_1() {
        let result = Solution::longest_palindrome("babad".to_string());

        assert_eq!("bab".to_string(), result);
    }

    #[test]
    fn longest_palindrome_2() {
        let result = Solution::longest_palindrome("cbbd".to_string());

        assert_eq!("bb".to_string(), result);
    }

    #[test]
    fn longest_palindrome_3() {
        let result = Solution::longest_palindrome("ac".to_string());

        assert_eq!("a".to_string(), result);
    }

    #[test]
    fn longest_palindrome_4() {
        let result = Solution::longest_palindrome("bb".to_string());

        assert_eq!("bb".to_string(), result);
    }

    #[test]
    fn longest_palindrome_5() {
        let result = Solution::longest_palindrome("aaabaaaa".to_string());

        assert_eq!("aaabaaa".to_string(), result);
    }
}
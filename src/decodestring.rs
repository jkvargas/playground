use std::collections::VecDeque;

struct Solution;

impl Solution {
    // S <- multiplicador
    // P <- tamannho maior de string
    // O(S + P)
    // 4[3[3[4[5[6[7[3[AAAAAAAAAAAAAAAAAAA]]]]]]] -> ...7[AAA AAA AAA] ->  S+N*(S*P)
    // N -> numero de multiplicador
    // 3[AAAABBBBBAAAAAAA]2[AAAAAAAAAAAAAA] => O(1)
    // 3*2^3
    // O( N ) + (S*P)^T <- Numero de multiplicadores
    // 3[3[3[AA]]]
    // O(N)
    // O(N) + S * O(N)
    // S[N] ONDE N = S[Y]
    pub fn decode_string(s: String) -> String {
        let str_len = s.len();
        let str_chars: Vec<char> = s.chars().collect();
        let mut result = String::new();
        let mut pos = 0;

        loop {
            if pos >= str_len {
                break;
            }

            if str_chars[pos].is_alphabetic() {
                result.push(str_chars[pos]);
                pos += 1;
                continue;
            }

            if str_chars[pos].is_digit(10) {
                let (word, npos) = Self::decode_expr(&str_chars, pos);
                pos = npos;
                result.push_str(&word);
            }
        }

        result
    }

    fn decode_expr(vec: &Vec<char>, mut pos: usize) -> (String, usize) {
        let mut is_open = false;
        let mut base_stack: VecDeque<u32> = VecDeque::new();
        let mut base_word = String::new();

        loop {
            if vec[pos].is_digit(10) {
                if is_open {
                    let (word, npos) = Self::decode_expr(vec, pos);
                    base_word.push_str(&word);
                    pos = npos;
                    if pos >= vec.len() {
                        break;
                    }
                    continue;
                } else {
                    base_stack.push_back(vec[pos].to_digit(10).unwrap());
                    pos += 1;
                    continue;
                }
            }
            if vec[pos].is_alphabetic() {
                base_word.push(vec[pos]);
                pos += 1;
                continue;
            }
            if vec[pos] == '[' {
                is_open = true;
                pos += 1;
                continue;
            }
            if vec[pos] == ']' {
                pos += 1;
                break;
            }
        }

        let mut iter_count = 0;
        let mut j = 1;
        while let Some(res) = base_stack.pop_back() {
            iter_count += j * res;
            j *= 10;
        }

        let mut result = String::new();
        for i in 0..iter_count {
            result.push_str(&base_word);
        }

        (result, pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // a3[a]2[bc]
    // string result <- a

    // a

    // decode expression
    // stack <- 3  1
    // open_expr = true
    // internal_string <- a
    // 3 [ a 2 ] -> decode_expression
    // for i in 0..3 { result += internal_string; }
    // return internal_string

    // 3[a2[c]]
    // 3[acc]
    // accaccacc

    #[test]
    pub fn decode_string_1() {
        assert_eq!(
            Solution::decode_string("3[a]2[bc]".to_string()),
            "aaabcbc".to_string()
        );
    }

    #[test]
    pub fn decode_string_2() {
        assert_eq!(
            Solution::decode_string("3[a2[c]]".to_string()),
            "accaccacc".to_string()
        );
    }

    #[test]
    pub fn decode_string_3() {
        assert_eq!(
            Solution::decode_string("2[abc]3[cd]ef".to_string()),
            "abcabccdcdcdef".to_string()
        );
    }

    #[test]
    pub fn decode_string_4() {
        assert_eq!(
            Solution::decode_string("at2[abc]3[cd]ef".to_string()),
            "atabcabccdcdcdef".to_string()
        );
    }
}

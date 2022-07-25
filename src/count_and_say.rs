use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        fn f(n: i32) -> Vec<char> {
            if n == 1 {
                return vec!['1'];
            }

            let content = f(n - 1);

            let mut result = Vec::new();
            let mut ocurrences = 0;
            let mut previous_letter = None;
            for i in 0..content.len() {
                let letter = content[i];

                if let Some(prev) = previous_letter {
                    if prev == letter {
                        ocurrences += 1;
                    } else {
                        result.push(char::from_digit(ocurrences as u32, 10).unwrap());
                        result.push(prev);

                        ocurrences = 1;
                        previous_letter = Some(letter);
                    }
                } else {
                    previous_letter = Some(letter);
                    ocurrences = 1;
                }
            }

            result.push(char::from_digit(ocurrences as u32, 10).unwrap());
            result.push(previous_letter.unwrap());

            result
        }

        f(n).iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::count_and_say::Solution;

    #[test]
    fn test_one() {
        assert_eq!("1211".to_string(), Solution::count_and_say(4));
    }
}

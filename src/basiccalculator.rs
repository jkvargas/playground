use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn calculate_ans(s: String) -> i32 {
        let mut num = 0;
        let mut stack = vec![];
        let mut last_operator = '+';
        for c in (s + "+").chars() {
            match c {
                ' ' => continue,
                ('0'..='9') => num = num * 10 + (c as i32 - '0' as i32),
                _ => {
                    match last_operator {
                        '+' => stack.push(num),
                        '-' => stack.push(-num),
                        '*' => *stack.last_mut().unwrap() *= num,
                        '/' => *stack.last_mut().unwrap() /= num,
                        _ => (),
                    };
                    last_operator = c;
                    num = 0;
                }
            };
        }
        stack.iter().sum()
    }

    pub fn calculate_second(s: String) -> i32 {
        let mut new_s: Vec<char> = format!("+{}", s).chars().collect();
        let mut ans = 0;
        let mut last = 0;
        let mut n: i32 = 0;

        while !new_s.is_empty() {
            let op = new_s.pop().unwrap();

            n = if op.is_digit(10) {
                op.to_digit(10).unwrap() as i32
            } else {
                0
            };

            if op == '+' || op == '-' {
                n = if op == '+' { n } else { -n };

                ans += n;
            } else {
                n = if op == '*' { last * n } else { last / n };

                ans = ans - last + n;
            }

            last = n;
        }

        ans
    }

    pub fn calculate(mut s: String) -> i32 {
        let mut letters: VecDeque<char> = s.chars().collect();
        let mut sums: Vec<i32> = Vec::new();
        sums.push(Self::number(&mut letters).unwrap() as i32);

        while !letters.is_empty() {
            if let Some((op, number)) = Self::operand_and_number(&mut letters) {
                if op == '*' {
                    let prev = sums.pop().unwrap();
                    sums.push((number * (prev as u32)) as i32);
                    continue;
                }

                if op == '/' {
                    let prev = sums.pop().unwrap();
                    sums.push((prev / number as i32));
                    continue;
                }

                if op == '-' {
                    sums.push(-(number as i32));
                }

                if op == '+' {
                    sums.push(number as i32);
                }
            }
        }

        sums.into_iter().sum()
    }

    fn number(letters: &mut VecDeque<char>) -> Option<u32> {
        let mut number_started = false;
        let mut number = 0;
        let mut multiplier = 1;

        loop {
            if letters.is_empty() {
                if number_started {
                    return Some(number);
                }

                return None;
            }

            let letter = letters.pop_front().unwrap();

            if letter == ' ' {
                continue;
            }

            if letter.is_digit(10) {
                number_started = true;
                number = number * multiplier;

                let n = letter.to_digit(10).unwrap();
                number += n;

                multiplier = 10;
            } else {
                if number_started {
                    letters.push_back(letter);
                    return Some(number);
                }
            }
        }
    }

    fn operand_and_number(letters: &mut VecDeque<char>) -> Option<(char, u32)> {
        let mut operand = '+';

        loop {
            if letters.is_empty() {
                return None;
            }

            let letter = letters.pop_back().unwrap();

            if letter == ' ' {
                continue;
            }

            if letter == '*' || letter == '/' || letter == '+' || letter == '-' {
                operand = letter;
                break;
            }
        }

        Some((operand, Self::number(letters).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn calculate_1() {
        assert_eq!(Solution::calculate_second("3+2*2".to_string()), 7);
    }

    #[test]
    pub fn calculate_2() {
        assert_eq!(1, Solution::calculate_second(" 3/2 ".to_string()));
    }

    #[test]
    pub fn calculate_3() {
        assert_eq!(5, Solution::calculate_second(" 3+5 / 2 ".to_string()));
    }

    #[test]
    pub fn calculate_4() {
        assert_eq!(
            18,
            Solution::calculate_second(" 3   +   5*4/2/2     +     10 ".to_string())
        );
    }

    #[test]
    pub fn calculate_5() {
        assert_eq!(3, Solution::calculate_second(" 3   ".to_string()));
    }

    #[test]
    pub fn calculate_6() {
        assert_eq!(69, Solution::calculate_second("9 * 8 - 3   ".to_string()));
    }

    #[test]
    pub fn calculate_7() {
        assert_eq!(
            26,
            Solution::calculate_second("30 - 12 / 2 + 20 / 10   ".to_string())
        );
    }

    #[test]
    pub fn calculate_8() {
        assert_eq!(42, Solution::calculate_second("42".to_string()));
    }

    #[test]
    pub fn calculate_9() {
        assert_eq!(1337, Solution::calculate_second("1337".to_string()));
    }
}

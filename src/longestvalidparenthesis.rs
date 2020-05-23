struct Solution;

impl Solution {
    pub fn longest_valid_parentheses_sec(s: String) -> i32 {
        let mut vec: Vec<bool> = vec![false; s.len()];
        let mut stack : Vec<usize> = Vec::new(); // stack

        if s.len() < 2 {
            return 0;
        }

        for (pos, i) in s.chars().enumerate() {
            if i == '(' {
                stack.push(pos);
                continue;
            }

            if i == ')' && !stack.is_empty() {
                let res = stack.pop().unwrap();

                vec[pos] = true;
                vec[res] = true;
                continue;
            }
        }

        let mut count = if vec[0] {
            1
        } else {
            0
        };

        let mut max = count;

        for i in 1..vec.len() {
            if vec[i] {
                count += 1;
            } else {
                max = std::cmp::max(count, max);
                count = 0;
            }
        }

        std::cmp::max(count, max)
    }

    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut arr: Vec<i32> = vec![0; s.len()];
        let vec: Vec<char> = s.chars().collect();
        let mut maxans = 0;

        // [ 0, 2, 0, 0, 0, 0, 0 ]
        // ( )
        // | |
        // ( ) ( ) ) ) ( )
        //   2   4 0 0 0 2
        // ( ( ) )

        for i in 1..vec.len() {
            if vec[i] == ')' {
                if vec[i - 1] == '(' {
                    arr[i] = if i >= 2 {
                        arr[i - 2]
                    } else {
                        0
                    } + 2;
                } else if i as i32 - arr[i - 1] > 0 && vec[i - arr[i - 1] as usize - 1] == '(' {
                    arr[i] = if arr[i - 1] + (i as i32 - arr[i - 1]) >= 2 {
                        arr[i - arr[i - 1] as usize - 2]
                    } else {
                        0
                    } + 2;
                }
                maxans = std::cmp::max(maxans, arr[i]);
            }
        }

        maxans
    }
}

// (          ...
//  \       /
//   (    )
//    \  /
//     )

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_valid_parentheses_1() {
        assert_eq!(Solution::longest_valid_parentheses_sec("(()".to_string()), 2);
    }

    #[test]
    fn longest_valid_parentheses_2() {
        assert_eq!(Solution::longest_valid_parentheses_sec(")()())".to_string()), 4);
    }

    #[test]
    fn longest_valid_parentheses_3() {
        assert_eq!(Solution::longest_valid_parentheses_sec("))()()))(()()())".to_string()), 8);
    }

    #[test]
    fn longest_valid_parentheses_4() {
        assert_eq!(Solution::longest_valid_parentheses_sec("()(()".to_string()), 2);
    }

    #[test]
    fn longest_valid_parentheses_5() {
        assert_eq!(Solution::longest_valid_parentheses_sec("()(())".to_string()), 6);
    }

    #[test]
    fn longest_valid_parentheses_6() {
        assert_eq!(Solution::longest_valid_parentheses_sec("".to_string()), 0);
    }
}
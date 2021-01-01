use std::collections::HashMap;

struct Solution;

struct LeetCode {
    memo: HashMap<usize, i32>,
}

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut leet = LeetCode::new();

        leet.num_decodings(s)
    }
}

impl LeetCode {
    pub fn new() -> Self {
        LeetCode {
            memo: HashMap::new(),
        }
    }

    pub fn num_decodings(&mut self, s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let str = s.chars().collect();

        self.recusive_with_memo(0, &str)
    }

    // fn somemethod(s: String) -> i32 {
    //     s.chars()
    //         .rev()
    //         .fold((' ', 0, 1), |(p, np, n), c| {
    //             (
    //                 c,
    //                 n,
    //                 (if ('1'..='9').contains(&c) {
    //                     n + format!("{}{}", c, p)
    //                         .parse::<i32>()
    //                         .ok()
    //                         .filter(|&i| i <= 26)
    //                         .map(|_| np)
    //                         .unwrap_or(0)
    //                 } else {
    //                     0
    //                 }),
    //             )
    //         })
    //         .2
    // }

    fn recusive_with_memo(&mut self, index: usize, str: &Vec<char>) -> i32 {
        if index == str.len() {
            return 1;
        }

        if str[index] == '0' {
            return 0;
        }

        if let Some(res) = self.memo.get(&index) {
            return *res;
        }

        let mut ans = self.recusive_with_memo(index + 1, str);

        if index + 2 <= str.len() {
            let sec_index = index + 2;
            let temp = str[index..sec_index]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            if temp <= 26 {
                ans += self.recusive_with_memo(index + 2, str);
            }
        }

        self.memo.insert(index, ans);

        ans
    }
}

#[test]
pub fn first() {
    let decodings = Solution::num_decodings("2326".to_string());

    assert_eq!(4, decodings)
}

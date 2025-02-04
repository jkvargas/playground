// https://leetcode.com/problems/gray-code/description/

struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut result = Vec::new();
        Helper::default().helper(&mut result, n as u32);
        result.iter().map(|&number| number as i32).collect()
    }
}

#[derive(Default)]
struct Helper {
    next_num: u32,
}

impl Helper {
    fn helper(&mut self, result: &mut Vec<u32>, n: u32) {
        if n == 0 {
            result.push(self.next_num);
            return;
        }

        self.helper(result, n - 1);
        self.next_num ^= 1 << (n - 1);
        self.helper(result, n - 1);
    }
}

// best solution
fn gray_code(n: i32) -> Vec<i32> {
    (0..2usize.pow(n as u32))
        .into_iter()
        .map(|idx| (idx ^ (idx >> 1)) as i32)
        .collect()
}

#[test]
fn test_one() {
    assert_eq!(vec![0, 1, 3, 2], Solution::gray_code(2));
}

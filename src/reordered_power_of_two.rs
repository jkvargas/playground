// https://leetcode.com/problems/reordered-power-of-2/description/

struct Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let bits = get_bits(n as u32);
        bits.iter().filter(|&&x| x == 1).count() == 1
    }
}

fn get_bits(n: u32) -> Vec<u8> {
    (0..32).rev().map(|i| ((n >> i) & 1) as u8).collect()
}

#[cfg(test)]
mod tests {
    use crate::reordered_power_of_two::Solution;

    #[test]
    fn test() {
        assert!(Solution::reordered_power_of2(46));
    }
}
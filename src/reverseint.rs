struct Solution;

/// made my own solution and then I got this from the internet because it was so ninja =)
impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        const LUT: [i32; 19] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let neg = x < 0;
        let mut y: i64 = 0;
        while x != 0 {
            y = y * 10 + i64::from(LUT[(x % 10 + 9) as usize]);
            if y > i64::from(i32::max_value()) {
                return 0;
            }
            x /= 10;
        }
        if neg {
            -y as i32
        } else {
            y as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_1() {
        assert_eq!(Solution::reverse(123), 321);
    }

    #[test]
    fn reverse_2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn reverse_3() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn reverse_4() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }

    #[test]
    fn reverse_5() {
        assert_eq!(Solution::reverse(-2147483648), 0);
    }
}

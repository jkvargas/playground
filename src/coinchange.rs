pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut amountvec: Vec<i32> = vec![amount + 1; (amount + 1) as usize];
        amountvec[0] = 0;

        for i in 0..amount + 1 {
            for j in &coins {
                if *j <= i {
                    amountvec[i as usize] =
                        std::cmp::min(amountvec[i as usize], 1 + amountvec[(i - *j) as usize])
                }
            }
        }

        if amountvec[amount as usize] > amount {
            -1
        } else {
            amountvec[amount as usize]
        }
    }

    fn coin_change_two(coins: Vec<i32>, amount: i32) -> i32 {
        if amount < 1 {
            return 0;
        }

        let mut count = vec![0; amount as usize];
        Self::cc(&coins, amount, &mut count)
    }

    fn cc(coins: &Vec<i32>, rem: i32, count: &mut Vec<i32>) -> i32 {
        if rem < 0 { return -1; }
        if rem == 0 { return 0; }
        if count[rem as usize - 1] != 0 { return count[rem as usize - 1]; }
        let mut min = i32::MAX;
        for &coin in coins {
            let res = Self::cc(coins, rem - coin, count);
            if res >= 0 && res < min {
                min = 1 + res;
            }
        }

        count[rem as usize - 1] = if min == i32::MIN { -1 } else { min };
        return count[rem as usize - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn coin_change_1() {
        assert_eq!(Solution::coin_change_two(vec![1, 2, 5], 11), 3);
    }

    #[test]
    pub fn coin_change_2() {
        assert_eq!(Solution::coin_change_two(vec![2], 3), -1);
    }

    #[test]
    pub fn coin_change_3() {
        assert_eq!(Solution::coin_change_two(vec![186, 419, 83, 408], 6249), 20);
    }
}

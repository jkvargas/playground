pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = i32::max_value();
        let mut max_profit = 0;
        for i in 0..prices.len() {
            if prices[i] < min_price {
                min_price = prices[i];
            } else {
                max_profit = std::cmp::max(max_profit, prices[i] - min_price);
            }
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_profit_1() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn max_profit_2() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn max_profit_3() {
        assert_eq!(Solution::max_profit(vec![1, 2]), 1);
    }
}
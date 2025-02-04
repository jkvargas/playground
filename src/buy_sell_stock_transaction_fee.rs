use std::cmp::max;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut memo = HashMap::new();
        dp(&prices, 0, fee, false, &mut memo)
    }
}

fn dp(
    prices: &Vec<i32>,
    index: usize,
    fee: i32,
    has_share: bool,
    memo: &mut HashMap<(usize, bool), i32>,
) -> i32 {
    if index == prices.len() {
        return 0;
    }

    if let Some(&res) = memo.get(&(index, has_share)) {
        return res;
    }

    let skip_stock = dp(prices, index + 1, fee, has_share, memo);
    let result = if has_share {
        max(
            dp(prices, index + 1, fee, false, memo) - fee + prices[index],
            skip_stock,
        )
    } else {
        max(
            dp(prices, index + 1, fee, true, memo) - prices[index],
            skip_stock,
        )
    };

    memo.insert((index, has_share), result);
    result
}

#[cfg(test)]
mod tests {
    use crate::buy_sell_stock_transaction_fee::Solution;

    #[test]
    fn test() {
        assert_eq!(8, Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2));
    }
}

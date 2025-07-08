struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        run_pow(x, n as f64)
    }
}

fn run_pow(mut x: f64, mut n: f64) -> f64 {
    if n == 0.0 { return 1.0; }
    if n == 1.0 { return x; }
    
    if n < 0.0 {
        n *= -1.0;
        x = 1.0 / x;
    }
    
    if n % 2.0 == 1.0 {
        return x * run_pow(x, n - 1.0);
    }
    
    let half = run_pow(x, n / 2.0);
    half * half
}

#[cfg(test)]
mod tests {
    use crate::pow::Solution;

    #[test]
    fn test_one() {
        let result = Solution::my_pow(2.0, 10);
        assert_eq!(1024.0, result);
    }

    #[test]
    fn test_two() {
        let result = Solution::my_pow(2.0, -2);
        assert_eq!(0.25000, result);
    }
}

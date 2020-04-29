use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut map : HashMap<i32, bool> = HashMap::new();
        let mut count_prime = 0;

        'next: for i in 2..n-1 {
            for j in 2..n-1 {
                let temp = i * j;

                if temp > n {
                    continue 'next;
                }

                map.insert(i * j, true);
            }
        }

        for i in 2..n {
            if !map.contains_key(&i) {
                count_prime += 1;
            }
        }

        count_prime
    }

    pub fn sieveoferatosthenes(n: i32) -> i32{
        let mut not_prime: Vec<bool> = vec![false; n as usize];
        let mut count = 0;
        for i in 2..n as usize {
            if !not_prime[i] { count += 1; }
            let mut j = 2;
            while j*i < n as usize {
                not_prime[j*i] = true;
                j += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_primes_1() {
        assert_eq!(Solution::sieveoferatosthenes(10), 4);
    }

    #[test]
    fn count_primes_2() {
        assert_eq!(Solution::sieveoferatosthenes(10), 4);
    }
}
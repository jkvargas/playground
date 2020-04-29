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
        let mut result : Vec<bool> = vec![true; n as usize];
        let mut p = 2;

        loop {
            if p * p > n {
                break;
            }

            if result[(p - 1)as usize] {
                let mut i = p * p;

                loop {
                    if i > n {
                        break;
                    }
                    result[(i - 1) as usize] = false;

                    i += p;
                }
            }

            p += 1;
        }

        let mut count = 0;

        for (i, val) in result.iter().enumerate() {
            if i > 0 && *val {
                count += 1;
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

    #[test]
    fn count_primes_3() {
        assert_eq!(Solution::sieveoferatosthenes(2), 1);
    }
}
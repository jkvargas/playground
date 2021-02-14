use std::cmp::min;
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn shopping_offers(price: Vec<i32>, mut special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        Self::shopping(&price, &special, needs, &mut map)
    }

    fn shopping(
        price: &Vec<i32>,
        special: &Vec<Vec<i32>>,
        needs: Vec<i32>,
        map: &mut HashMap<Vec<i32>, i32>,
    ) -> i32 {
        if map.contains_key(&needs) {
            return *map.get(&needs).unwrap();
        }

        let mut res = Self::dot(&needs, &price);
        let mut j = 0;

        for s in special {
            let mut s_clone = s.clone();

            for j in 0..needs.len() {
                let diff = s_clone[j] - s[j];
                if diff < 0 {
                    break;
                }

                s_clone[j] = diff;
            }

            if j == needs.len() {
                res = min(res, s[j] + Self::shopping(price, special, s_clone, map));
            }
        }

        map.insert(needs, res);

        return res;

        10
    }

    fn dot(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
        let mut sum = 0;

        for i in 0..a.len() {
            sum += a[i] * b[i];
        }

        sum
    }
}

#[test]
pub fn example1() {
    assert_eq!(
        14,
        Solution::shopping_offers(vec![2, 5], vec![vec![3, 0, 5], vec![1, 2, 10]], vec![3, 2])
    );
}

#[test]
pub fn example2() {
    assert_eq!(
        11,
        Solution::shopping_offers(
            vec![2, 3, 4],
            vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]],
            vec![1, 2, 1]
        )
    );
}

use std::collections::HashMap;

struct Solution;

// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {
        let mut map = HashMap::new();

        map.insert(1, 'I');
        map.insert(5, 'V');
        map.insert(10, 'X');
        map.insert(50, 'L');
        map.insert(100, 'C');
        map.insert(500, 'D');
        map.insert(1000, 'M');

        let priority = vec![1, 5, 10, 50, 100, 500, 1000];
        let mut result = String::new();

        for pos in (0..7).rev() {
            let div = num / priority[pos];
            let remain = num % priority[pos];

            if pos > 0 && remain / priority[pos - 1] == 4 {
                continue;
            }

            if div == 4 {
                result.push(*map.get(&priority[pos]).unwrap());
                let front_letter = *map.get(&priority[pos + 1]).unwrap();
                result.push(front_letter);
            } else {
                if div == 9 {
                    result.push(*map.get(&priority[pos]).unwrap());
                    let front_letter = *map.get(&(priority[pos] * 10)).unwrap();
                    result.push(front_letter);
                } else {
                    for _ in 0..div {
                        result.push(*map.get(&priority[pos]).unwrap());
                    }
                }
            }

            num = remain;
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::inttoroman::Solution;

    #[test]
    fn test_one() {
        assert_eq!("IV".to_string(), Solution::int_to_roman(4));
    }

    #[test]
    fn test_two() {
        assert_eq!("III".to_string(), Solution::int_to_roman(3));
    }

    #[test]
    fn test_three() {
        assert_eq!("XIV".to_string(), Solution::int_to_roman(14));
    }

    #[test]
    fn test_four() {
        assert_eq!("MCMXCIV".to_string(), Solution::int_to_roman(1994));
    }
}
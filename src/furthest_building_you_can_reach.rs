use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, mut ladders: i32) -> i32 {
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        for i in 0..(heights.len() - 1) {
            let need = heights[i + 1] - heights[i];
            if need <= 0 {
                continue;
            }
            heap.push(need);
            bricks -= need;
            if bricks < 0 && ladders > 0 {
                bricks += heap.pop().unwrap();
                ladders -= 1;
            }

            if bricks < 0 {
                return i as i32;
            }
        }

        (heights.len() - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::furthest_building_you_can_reach::Solution;

    #[test]
    fn test_one() {
        assert_eq!(
            4,
            Solution::furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1)
        );
    }

    #[test]
    fn test_two() {
        assert_eq!(
            7,
            Solution::furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2)
        );
    }

    #[test]
    fn test_three() {
        assert_eq!(3, Solution::furthest_building(vec![14, 3, 19, 3], 17, 0));
    }

    #[test]
    fn test_four() {
        assert_eq!(1, Solution::furthest_building(vec![3, 19], 87, 1));
    }
}

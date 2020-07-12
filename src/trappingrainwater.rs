use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let height = height.iter().map(|x| *x as usize).collect::<Vec<usize>>();
        if height.len() == 0 { return 0 as i32; }

        let mut left = 0;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut right = height.len() - 1;
        let mut water = 0;

        while left != right {
            left_max = std::cmp::max(height[left], left_max);
            right_max = std::cmp::max(height[right], right_max);

            if left_max < right_max {
                water += left_max - height[left];
                left += 1;
            } else {
                water += right_max - height[right];
                right -= 1;
            }
        }
        water as i32
    }

    // O(1)
    // O(N)
    // [0,1,0,2,1,0,1,3,2,1,2,1]
    // [3,2,1,0,1]

    pub fn trap_jho(height: Vec<i32>) -> i32 {
        let mut cur = 0;
        let mut ammo_lvl: HashMap<i32, i32> = HashMap::new();

        while cur < height.len() {
            match Self::get_trapped(cur, &height, &mut ammo_lvl) {
                Ok(pos) => {
                    if !ammo_lvl.contains_key(&height[cur]) {
                        ammo_lvl.insert(height[cur], 0);
                    }
                    *ammo_lvl.get_mut(&height[cur]).unwrap() += (pos - cur - 1) as i32;
                    cur += pos;
                },
                Err(err) => {
                    cur += err;
                },
            }
        }

        ammo_lvl.values().sum()
    }

    fn get_trapped(start: usize, height: &Vec<i32>, ammo_level: &mut HashMap<i32, i32>) -> Result<usize, usize> {
        let mut cur = start + 1;

        loop {
            if cur >= height.len() {
                return Err(cur);
            }

            if height[cur] < height[cur - 1] {
                if !ammo_level.contains_key(&height[start]) {
                    ammo_level.insert(height[start], 0);
                }

                match Self::get_trapped(cur, height, ammo_level) {
                    Ok(last) => {
                        *ammo_level.get_mut(&height[start]).unwrap() += (last - cur - 1) as i32;
                        cur = last;
                    },
                    Err(last) => {
                        cur = last;
                    },
                }
            } else if height[cur] > height[start] {
                return Ok(start);
            }

            cur += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trap_1() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }
}
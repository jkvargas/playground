struct Solution;

impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let directions = vec![vec![0, 1], vec![1, 0], vec![0, -1], vec![-1, 0]];
        let mut x = 0;
        let mut y = 0;
        let mut idx: usize = 0;

        for i in instructions.chars() {
            match i {
                'L' => {
                    idx = (idx + 3) % 4;
                }
                'R' => idx = (idx + 1) % 4,
                _ => {
                    x += directions[idx][0];
                    y += directions[idx][1];
                }
            }
        }

        (x == 0 && y == 0) || idx != 0
    }
}

#[test]
fn test_one() {
    assert!(Solution::is_robot_bounded("GGLLGG".to_string()));
}

#[test]
fn test_sec() {
    assert_eq!(false, Solution::is_robot_bounded("GG".to_string()));
}

#[test]
fn test_third() {
    assert_eq!(true, Solution::is_robot_bounded("GL".to_string()));
}

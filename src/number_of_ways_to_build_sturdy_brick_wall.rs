use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};

const MOD: i64 = 1_000_000_007;

// My solution
// Too slow...
struct Solution;

impl Solution {
    // pub fn build_wall(height: i32, width: i32, bricks: Vec<i32>) -> i32 {
    //
    // }

    pub fn build_wall_jhonny(height: i32, width: i32, bricks: Vec<i32>) -> i32 {
        let mut placed = vec![HashSet::new(); height as usize];
        dp(0, height, width, 0, &bricks, &mut placed) as i32
    }
}

fn generate_all_possible_for_row(
    pos: i32,
    width: i32,
    bricks: &Vec<i32>,
    placed: &mut HashSet<i32>,
    result: &mut Vec<HashSet<i32>>,
) {
    if pos == width {
        result.push(placed.clone());
        return;
    }

    if pos > width {
        return;
    }

    for current_brick in bricks {
        let new_pos = pos + current_brick;
        if new_pos != width {
            placed.insert(new_pos);
        }

        generate_all_possible_for_row(new_pos, width, bricks, placed, result);

        if new_pos != width {
            placed.remove(&new_pos);
        }
    }
}

fn dp(
    mut line: i32,
    height: i32,
    width: i32,
    mut pos: i32,
    bricks: &Vec<i32>,
    placed: &mut Vec<HashSet<i32>>,
) -> i64 {
    if pos > width {
        return 0;
    }

    if line == height {
        return 1;
    }

    let mut result = 0;

    for &chosen_brick in bricks {
        let new_pos = pos + chosen_brick;

        if new_pos != width && line > 0 && placed[line as usize - 1].contains(&new_pos) {
            continue;
        }

        placed[line as usize].insert(new_pos);

        result += dp(
            if new_pos == width { line + 1 } else { line },
            height,
            width,
            if new_pos == width { 0 } else { new_pos },
            bricks,
            placed,
        ) % MOD;

        placed[line as usize].remove(&new_pos);
    }

    result
}

#[test]
fn test_one() {
    assert_eq!(2, Solution::build_wall_jhonny(2, 3, vec![1, 2]));
}

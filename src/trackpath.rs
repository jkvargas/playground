use std::collections::{HashMap, VecDeque};

const WALL: i32 = 1;
const END: i32 = 3;

pub struct Maze {
    visited: HashMap<(usize, usize), bool>,
    nodes_left: u32,
    nodes_next: u32,
    queue: VecDeque<(usize, usize, Vec<Vec<i32>>)>,
    size_i: usize,
    size_j: usize,
    maze: Vec<Vec<i32>>,
    end_found: bool,
}

impl Maze {
    pub fn find_best_path(&mut self) -> Vec<Vec<i32>> {
        self.queue.push_back((0, 0, vec![vec![0, 0]]));
        self.visited.insert((0, 0), true);
        self.nodes_left = 1;

        while !self.queue.is_empty() {
            let node = self.queue.pop_front().unwrap();

            if self.maze[node.0][node.1] == END {
                self.end_found = true;
                return node.2;
            }

            self.visit_nodes(node);
            self.nodes_left -= 1;

            if self.nodes_left == 0 {
                self.nodes_left = self.nodes_next;
                self.nodes_next = 0;
            }
        }

        Vec::<Vec<i32>>::new()
    }

    pub fn new(vec: Vec<Vec<i32>>) -> Self {
        Maze {
            visited: HashMap::new(),
            nodes_left: 0,
            nodes_next: 0,
            queue: VecDeque::new(),
            size_i: vec.len(),
            size_j: vec[0].len(),
            maze: vec,
            end_found: false,
        }
    }

    fn visit_nodes(&mut self, pos: (usize, usize, Vec<Vec<i32>>)) {
        let mut positions = Vec::<(usize, usize)>::new();

        if pos.0 > 0 {
            positions.push((pos.0 - 1, pos.1));
        }

        if pos.1 > 0 {
            positions.push((pos.0, pos.1 - 1));
        }

        if pos.0 + 1 < self.size_i {
            positions.push((pos.0 + 1, pos.1));
        }

        if pos.1 + 1 < self.size_j {
            positions.push((pos.0, pos.1 + 1));
        }

        for new_pos in positions {
            if self.visited.contains_key(&new_pos) {
                continue;
            }

            if self.maze[new_pos.0][new_pos.1] == WALL {
                continue;
            }

            self.visited.insert(new_pos, true);
            self.nodes_next += 1;
            let mut newvec = pos.2.clone();
            let mut temp: Vec<i32> = Vec::new();
            temp.push(new_pos.0 as i32);
            temp.push(new_pos.1 as i32);
            newvec.push(temp);

            self.queue.push_back((new_pos.0, new_pos.1, newvec));
        }
    }
}

/// BSF
/// 0 -> 1 -> 2
/// 1 -> 2
/// 2
/// EVERY TIME IT ENCOUNTERS SOMETHING THATS NOT THE END OR A WALL IT FEEDS THE QUEUE

pub struct Solution;

impl Solution {
    pub fn find_path(vec: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut maze = Maze::new(vec);

        maze.find_best_path()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_path_2() {
        let mut matrix_origin = vec![
            vec![2, 1, 0, 1, 0, 1, 0, 0, 0, 1],
            vec![0, 1, 0, 0, 0, 1, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 0, 0, 1, 0],
            vec![0, 1, 0, 1, 1, 1, 1, 0, 1, 0],
            vec![0, 1, 0, 0, 0, 0, 0, 0, 1, 3],
        ];

        let result = Solution::find_path(matrix_origin.clone());

        for i in result {
            matrix_origin[i[0] as usize][i[1] as usize] = 5;
        }

        for i in 0..matrix_origin.len() {
            for j in 0..matrix_origin[0].len() {
                print!("{} ", matrix_origin[i][j]);
            }
            println!();
        }
    }

    #[test]
    fn find_path_1() {
        let mut matrix_origin = vec![
            vec![2, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 3],
        ];

        let result = Solution::find_path(matrix_origin.clone());

        for i in result {
            matrix_origin[i[0] as usize][i[1] as usize] = 5;
        }

        for i in 0..matrix_origin.len() {
            for j in 0..matrix_origin[0].len() {
                print!("{} ", matrix_origin[i][j]);
            }
            println!();
        }
    }
}

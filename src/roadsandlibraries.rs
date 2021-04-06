use std::collections::HashMap;

type RoadMap = HashMap<usize, Vec<usize>>;

struct Roads {
    visited: Vec<bool>,
    map: RoadMap,
    n: usize,
}

impl Roads {
    pub fn new(n: usize, cities: Vec<Vec<usize>>) -> Self {
        let mut map = HashMap::new();

        for i in 0..n {
            map.insert(i, vec![]);
        }

        for city in cities {
            (*map.get_mut(&(city[0] - 1)).unwrap()).push(city[1] - 1);
            (*map.get_mut(&(city[1] - 1)).unwrap()).push(city[0] - 1);
        }

        Self {
            visited: vec![false; n],
            map,
            n,
        }
    }

    pub fn roads_and_libraries(&mut self, c_lib: i32, c_road: i32) -> usize {
        if c_lib < c_road {
            return self.n * c_lib as usize;
        }

        let mut total = 0;

        for i in 0..self.n {
            if !self.visited[i] {
                let count = self.dfs(i, 0);

                total += (c_lib + (count * c_road)) as usize;
            }
        }

        total
    }

    fn dfs(&mut self, position: usize, mut count: i32) -> i32 {
        self.visited[position] = true;

        for neighbor in self.map.get(&position).unwrap().clone() {
            if !self.visited[neighbor] {
                count = 1 + self.dfs(neighbor, count);
            }
        }

        count
    }
}

struct Solution;

impl Solution {
    pub fn roads_and_libraries(
        n: usize,
        c_lib: i32,
        c_road: i32,
        cities: Vec<Vec<usize>>,
    ) -> usize {
        let mut roads = Roads::new(n, cities);

        roads.roads_and_libraries(c_lib, c_road)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn roads_and_libraries_1() {
        assert_eq!(
            Solution::roads_and_libraries(3, 2, 1, vec![vec![1, 2], vec![3, 1], vec![2, 3]]),
            4
        );
    }
}

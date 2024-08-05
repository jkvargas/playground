use std::collections::HashMap;

struct neighborSum {
    grid: Vec<Vec<i32>>,
    positions: HashMap<i32, (usize, usize)>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl neighborSum {

    fn new(grid: Vec<Vec<i32>>) -> Self {
        let mut positions = HashMap::new();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                positions.insert(grid[i][j], (i, j));
            }
        }


        Self {
            grid,
            positions
        }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        let &(x, y) = self.positions.get(&value).unwrap();

        (if x > 0 { self.grid[x - 1][y] } else { 0 }) +
            (if x < self.grid.len() - 1 { self.grid[x +1 ][y] } else { 0 }) +
            (if y > 0 { self.grid[x][y - 1] } else { 0 }) +
            (if y < self.grid.len() - 1 { self.grid[x][y + 1] } else { 0 })
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        let &(x, y) = self.positions.get(&value).unwrap();

        (if x > 0 && y > 0 { self.grid[x - 1][y - 1] } else { 0 }) +
            (if x > 0 && y < self.grid.len() - 1 { self.grid[x - 1][y + 1] } else { 0 }) +
            (if x < self.grid.len() - 1 && y > 0 { self.grid[x + 1][y - 1] } else { 0 }) +
            (if x < self.grid.len() - 1 && y < self.grid.len() - 1 { self.grid[x + 1][y + 1] } else { 0 })
    }
}

#[cfg(test)]
mod test {
    use crate::neighborsum::neighborSum;

    #[test]
    fn test_one() {
        let n = neighborSum::new(vec![vec![0,1,2],vec![3,4,5],vec![6,7,8]]);


        n.adjacent_sum(1);
        println!("{}", n.adjacent_sum(4));
        println!("{}", n.diagonal_sum(4));
        n.diagonal_sum(8);
    }
}
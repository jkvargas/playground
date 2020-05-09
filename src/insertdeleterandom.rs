use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::prelude::*;

struct RandomizedSet {
    map: HashMap<i32, usize>,
    vec: Vec<i32>
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            vec: Vec::new()
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            self.vec.push(val);
            self.map.insert(val, self.vec.len() - 1);

            return true;
        }

        false
    }

    // map(5) -> 2
    // map(3) -> 1

    // 1 3 5 -> 1 5 3 pop
    // 0 1 2 -> 0 1 2
    //   | |

    // last_pos = 2
    // last_key = 5


    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            let last_pos = self.vec.len() - 1;
            let last_key = self.vec[last_pos];
            let pos_rem = *self.map.get(&val).unwrap();
            Self::swap(&mut self.vec, pos_rem, last_pos);
            *self.map.get_mut(&last_key).unwrap() = pos_rem;

            self.map.remove(&val);
            self.vec.pop();

            return true;
        }

        false
    }

    fn swap(vec: &mut Vec<i32>, i: usize, j: usize) {
        let temp = vec[i];
        vec[i] = vec[j];
        vec[j] = temp;
    }

    /** Get a random element from the set. */
    fn get_random(&self) -> i32 {
        ///
        /// if no elements is going to panic!!!
        ///
        let mut rng = rand::thread_rng();
        let index : usize = rng.gen::<usize>() % self.vec.len() - 1;

        self.vec[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn randomized_set_2() {
        let mut randomSet = RandomizedSet::new();

        randomSet.insert(2);
        randomSet.insert(3);
        randomSet.insert(5);

        randomSet.remove(3);

        assert_eq!(0, *randomSet.map.get(&2).unwrap());
        assert_eq!(1, *randomSet.map.get(&5).unwrap());
        assert_eq!(2, randomSet.map.len());
        assert_eq!(2, randomSet.vec[0]);
        assert_eq!(5, randomSet.vec[1]);
        assert_eq!(2, randomSet.vec.len());
    }

    #[test]
    fn randomized_set_1() {
        let mut randomSet = RandomizedSet::new();

        assert_eq!(true, randomSet.insert(1));

        // Returns false as 2 does not exist in the set.
        assert_eq!(false, randomSet.remove(2));

        // Inserts 2 to the set, returns true. Set now contains [1,2].
        assert_eq!(true, randomSet.insert(2));

        // getRandom should return either 1 or 2 randomly.
        let range = std::ops::Range {
            start: 1,
            end: 2
        };

        assert!(range.contains(&randomSet.get_random()));

        // Removes 1 from the set, returns true. Set now contains [2].
        assert_eq!(true, randomSet.remove(1));

        // 2 was already in the set, so return false.
        assert_eq!(false, randomSet.insert(2));

        // Since 2 is the only number in the set, getRandom always return 2.
       assert_eq!(2, randomSet.get_random());
    }
}
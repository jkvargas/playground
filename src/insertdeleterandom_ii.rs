use std::collections::HashMap;

extern "C" {
    fn rand() -> u32;
}

struct RandomizedSet {
    set: HashMap<i32, usize>,
    vec: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            set: HashMap::new(),
            vec: Vec::new(),
        }
    }

    /** Inserts a value to the set. Returns true if the set did not already contain the specified element. */
    fn insert(&mut self, val: i32) -> bool {
        if self.set.contains_key(&val) {
            return false;
        }

        self.vec.push(val);
        self.set.insert(val, self.vec.len() - 1);

        true
    }

    /** Removes a value from the set. Returns true if the set contained the specified element. */
    fn remove(&mut self, val: i32) -> bool {
        if let Some(&v) = self.set.get(&val) {
            if v != self.vec.len() - 1 {
                self.swap_with_last(v);

                if let Some(s) = self.set.get_mut(&self.vec[v]) {
                    *s = v;
                }
            }

            self.vec.pop();
            self.set.remove(&val);

            return true;
        }

        false
    }

    #[inline]
    fn swap_with_last(&mut self, pos: usize) {
        let size = self.vec.len() - 1;
        let temp = self.vec[pos];
        self.vec[pos] = self.vec[self.vec.len() - 1];
        self.vec[size] = temp;
    }

    /** Get a random element from the set. */
    fn get_random(&self) -> i32 {
        self.vec[unsafe { rand() } as usize % self.vec.len()]
    }
}

#[test]
fn test_one() {
    let mut test = RandomizedSet::new();

    test.insert(1);
    test.remove(2);
    test.insert(2);
    let temp = test.get_random();

    test.remove(1);
    test.insert(2);
    let temp_sec = test.get_random();
}

#[test]
fn test_sec() {
    let mut test = RandomizedSet::new();

    test.remove(0);
    test.remove(0);
    test.insert(0);
    let temp = test.get_random();

    test.remove(0);
    test.insert(0);
}

#[test]
fn test_three() {
    let mut test = RandomizedSet::new();

    test.insert(1);
    test.insert(10);
    test.insert(20);
    test.insert(30);
}

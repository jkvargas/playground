use std::collections::HashMap;

pub struct OrderedMap {
    map: HashMap<usize, usize>,
    vec: Vec<usize>,
}

impl OrderedMap {
    pub fn new() -> OrderedMap {
        Self {
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }

    pub fn insert(&mut self, val: usize) {
        self.vec.push(val);
        self.map.insert(val, self.vec.len() - 1);
    }

    pub fn contains(&self, val: &usize) -> bool {
        self.map.contains_key(val)
    }

    pub fn remove(&mut self, val: &usize) {
        let last = self.vec.pop().unwrap();
        self.map.remove(&last);
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &usize> {
        self.vec.iter()
    }
}

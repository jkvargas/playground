use std::collections::HashMap;

struct FreqStack {
    map: HashMap<i32, i32>,
    out: HashMap<i32, Vec<i32>>,
    max_freq: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FreqStack {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            out: HashMap::new(),
            max_freq: 0,
        }
    }

    fn push(&mut self, x: i32) {
        if self.map.contains_key(&x) {
            *self.map.get_mut(&x).unwrap() += 1;
        } else {
            self.map.insert(x, 1);
        }

        let f = *self.map.get(&x).unwrap();

        if self.max_freq < f {
            self.max_freq = f;
        }

        if self.out.contains_key(&f) {
            self.out.get_mut(&f).unwrap().push(x);
        } else {
            let mut vec = Vec::new();
            vec.push(x);
            self.out.insert(f, vec);
        }
    }

    fn pop(&mut self) -> i32 {
        let x = self.out.get_mut(&self.max_freq).unwrap().pop().unwrap();
        *self.map.get_mut(&x).unwrap() -= 1;

        if self.out.get(&self.max_freq).unwrap().len() == 0 {
            self.max_freq -= 1;
        }

        x
    }
}

/**
 * Your FreqStack object will be instantiated and called as such:
 * let obj = FreqStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn freqstack_1() {
        let mut freq = FreqStack::new();

        freq.push(5);
        freq.push(7);
        freq.push(5);
        freq.push(7);
        freq.push(4);
        freq.push(5);

        assert_eq!(freq.pop(), 5);
        assert_eq!(freq.pop(), 7);
        assert_eq!(freq.pop(), 5);
        assert_eq!(freq.pop(), 4);
    }
}

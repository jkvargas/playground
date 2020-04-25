use std::collections::VecDeque;

struct MinStack {
    stack: VecDeque<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        Self {
            stack: VecDeque::<(i32, i32)>::new(),
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.is_empty() {
            self.stack.push_front((x, x));
        } else {
            let found_value = self.stack.get(0).unwrap().1;

            if found_value < x {
                self.stack.push_front((x, found_value))
            } else {
                self.stack.push_front((x, x));
            }
        }
    }

    fn pop(&mut self) {
        self.stack.pop_front();
    }

    fn top(&self) -> i32 {
        if let Some(found) = self.stack.get(0) {
            return found.0;
        }

        0
    }

    fn get_min(&self) -> i32 {
        if let Some(found) = self.stack.get(0) {
            return found.1;
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middle_node_1() {
        let mut temp = MinStack::new();
        temp.push(-2);
        temp.push(0);
        temp.push(-3);
        assert_eq!(-3, temp.get_min());
        temp.pop();
        assert_eq!(0, temp.top());
        assert_eq!(-2, temp.get_min());
    }
}

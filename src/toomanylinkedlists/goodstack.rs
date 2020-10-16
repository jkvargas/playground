use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Box<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    pub fn append(&self, element: T) -> List<T> {
        List {
            head: Some(Rc::new(Box::new(Node {
                elem: element,
                next: self.head.clone(),
            }))),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    use std::borrow::{Borrow, BorrowMut};

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.head(), None);

        let acquired = list.borrow_mut();

        let list = acquired.append(1).append(2).append(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }
}

use std::fmt::Debug;

// O(vec.len()*log(vec.len()))
pub fn heap_sort<T: Ord>(vec: &mut [T]) {
    if vec.len() <= 1 {
        return;
    }

    heapify(vec);

    for i in (1..vec.len()).rev() {
        vec.swap(0, i);
        move_down(&mut vec[..i], 0);
    }
}

/// convert vec into a max heap
fn heapify<T: Ord>(vec: &mut [T]) {
    let last_parent = (vec.len() - 2) / 2;
    for i in (0..=last_parent).rev() {
        move_down(vec, i);
    }
}

/// move the element at 'root' down until 'arr' is a max heap again
fn move_down<T: Ord>(vec: &mut [T], mut root: usize) {
    loop {
        let left = 2 * root + 1;
        if left >= vec.len() {
            break;
        }
        let right = left + 1;
        let max = if right < vec.len() && vec[right] > vec[left] {
            right
        } else {
            left
        };

        if vec[max] > vec[root] {
            vec.swap(root, max);
        }
        root = max;
    }
}

#[cfg(test)]
mod test {
    use crate::sortingalgos::heapsort::heap_sort;

    #[test]
    fn test_one() {
        let mut vec = vec![1, 12, 9, 5, 6, 10];
        heap_sort(&mut vec);
        assert_eq!(vec![1, 5, 6, 9, 10, 12], vec);
    }
}
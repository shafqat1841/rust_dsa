use std::fmt::Debug;

fn swap<T: Debug + Ord>(arr: &mut Vec<T>, left: usize, right: usize) {
    arr.swap(left, right);
}

fn get_left_child(index: usize) -> usize {
    2 * index + 1
}

fn get_right_child(index: usize) -> usize {
    2 * index + 2
}

fn get_parent(index: usize) -> usize {
    if index == 0 {
        return 0;
    }
    (index - 1) / 2
}

fn get_heigh_of_tree(length: usize) -> u32 {
    // log n
    length.ilog2()
}

pub fn sift_down<T: Debug + Ord>(arr: &mut Vec<T>, mut index: usize, len: usize) {
    // println!("arr: {:?}", arr);

    loop {
        let left = 2 * index + 1;
        let right = 2 * index + 2;
        let mut largest = index;

        if left < len && arr[left] > arr[largest] {
            largest = left;
        }

        if right < len && arr[right] > arr[largest] {
            largest = right;
        }

        if largest != index {
            swap(arr, index, largest);
            index = largest;
        } else {
            break;
        }
    }
}

pub fn build_max_heap<T: Debug + Ord>(arr: &mut Vec<T>) {
    let len = arr.len();

    if len <= 1 {
        return;
    }

    let last_non_leaf = (arr.len() - 2) / 2;
    let mut i = last_non_leaf + 1;

    while i > 0 {
        i -= 1;
        sift_down(arr, i, len);
    }
}

struct solution;

impl solution {
    fn create<T: Debug + Ord>(arr: &mut Vec<T>) {
        build_max_heap(arr);
    }

    fn insert<T: Debug + Ord>(arr: &mut Vec<T>, number: T) {
        arr.push(number);

        let mut last_ele_i = arr.len() - 1;

        loop {
            let parent_i = get_parent(last_ele_i);
            if arr[last_ele_i] > arr[parent_i] {
                swap(arr, parent_i, last_ele_i);
                last_ele_i = parent_i
            } else {
                break;
            }
        }
    }

    fn pop<T: Debug + Ord>(arr: &mut Vec<T>) -> Option<T> {
        let len = arr.len();

        if len == 0 {
            return None;
        }
        
        let last_index = arr.len() - 1;
        swap(arr, 0, last_index);
        
        // Pop the target element off first or scope the sift-down
        let extracted = arr.pop(); // Removes the old root safely
        
        // If elements remain, sift down the new root within the remaining slice
        if !arr.is_empty() {
            sift_down(arr, 0, arr.len()); // sift_down adapted to a slice
        }
        
        extracted

    }
}

#[cfg(test)]
mod max_heap_test {
    use super::*;

    #[test]
    fn create_data_structure() {
        let mut array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result_array = [10, 9, 7, 8, 5, 6, 3, 1, 4, 2];
        solution::create(&mut array);

        assert_eq!(array, result_array);
    }

    #[test]
    fn insert_largest_ele() {
        let mut array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result_array = [20, 10, 7, 8, 9, 6, 3, 1, 4, 2, 5];
        solution::create(&mut array);
        solution::insert(&mut array, 20);

        // println!("array: {:?}", array);
        assert_eq!(array, result_array);
    }

    #[test]
    fn insert_smallest_ele() {
        let mut array = vec![2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result_array = [10, 9, 8, 5, 6, 7, 4, 3, 2, 1];
        solution::create(&mut array);
        solution::insert(&mut array, 1);

        // println!("array: {:?}", array);
        assert_eq!(array, result_array);
    }

    #[test]
    fn insert_mid_ele() {
        let mut array = vec![1, 2, 3, 4, 6, 7, 8, 9, 10];
        let result_array = [10, 9, 8, 4, 6, 7, 3, 2, 1, 5];
        solution::create(&mut array);
        solution::insert(&mut array, 5);

        // println!("array: {:?}", array);
        assert_eq!(array, result_array);
    }
    #[test]
    fn pop_root() {
        let mut array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result_array = [9, 8, 7, 4, 5, 6, 3, 1, 2];
        solution::create(&mut array);
        solution::pop(&mut array);
        // println!("array: {:?}", array);
        assert_eq!(array, result_array);
    }
}

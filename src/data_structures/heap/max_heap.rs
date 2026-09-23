fn swap(arr: &mut Vec<i32>, left: usize, right: usize) {
    let temp = arr[left];
    arr[left] = arr[right];
    arr[right] = temp;
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

fn sift_down(arr: &mut Vec<i32>, mut index: usize) {
    println!("arr: {:?}", arr);
    let len = arr.len();

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
            arr.swap(index, largest);
            index = largest;
        } else {
            break;
        }
    }
}

pub fn build_max_heap(arr: &mut Vec<i32>) {
    if arr.len() <= 1 {
        return;
    }

    let last_non_leaf = (arr.len() - 2) / 2;
    let mut i = last_non_leaf + 1;

    while i > 0 {
        i -= 1;
        sift_down(arr, i);
    }
}

struct solution;

impl solution {
    fn create(arr: &mut Vec<i32>) {
        build_max_heap(arr);
    }

    fn insert(arr: &mut Vec<i32>, number: i32) {
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

    fn pop(arr: &mut Vec<i32>) {
        let last_index = arr.len() - 1;
        let mut index = 0;
        swap(arr, index, last_index);
        let len = arr.len();

        loop {
            let left = get_left_child(index);
            let right = get_right_child(index);

            let mut great = index;

            if left < len && arr[left] > arr[great] {
                great = left;
            }

            if right < len && arr[right] > arr[great] {
                great = right;
            }

            if great != index {
                swap(arr, index, great);
                index = great;
            } else {
                break;
            }
        }

        arr.pop();
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

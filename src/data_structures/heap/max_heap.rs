fn swap(arr: &mut [i32], left: usize, right: usize) {
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
// Sifts an element down the heap to maintain the max-heap property
fn sift_down(arr: &mut [i32], mut index: usize) {
    println!("arr: {:?}", arr);
    let len = arr.len();

    loop {
        let left = 2 * index + 1;
        let right = 2 * index + 2;
        let mut largest = index;

        // Check if left child exists and is greater than current largest
        if left < len && arr[left] > arr[largest] {
            largest = left;
        }

        // Check if right child exists and is greater than current largest
        if right < len && arr[right] > arr[largest] {
            largest = right;
        }

        // If the largest is not the current index, swap and continue sinking down
        if largest != index {
            arr.swap(index, largest);
            index = largest;
        } else {
            break;
        }
    }
}

// Builds a max-heap in place in O(N) time
pub fn build_max_heap(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    // Find the last non-leaf node
    // Formula: (arr.len() - 2) / 2
    let last_non_leaf = (arr.len() - 2) / 2;
    let mut i = last_non_leaf + 1;

    // Iterate backwards from the last non-leaf node to the root (index 0)
    while i > 0 {
        i -= 1;
        sift_down(arr, i);
    }
}

struct solution;

impl solution {
    fn create(arr: &mut [i32]) {
        build_max_heap(arr);
    }
}

#[cfg(test)]
mod max_heap_test {
    use super::*;

    #[test]
    fn create_data_structure() {
        let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        solution::create(&mut array);

        println!("array: {:?}", array);

        // array: [10, 9, 7, 8, 5, 6, 3, 1, 4, 2]
        //               10
        //       9       ,       7
        //   8   ,   5   ,   6   ,   3
        // 1 , 4 , 2 , - , - , - , - , -

        let height = get_heigh_of_tree(array.len());

        //            1               = 0
        //     2      ,     3         = 1
        //    4/5     ,    6/7        = 2
        // 8/9 , 10/- , -/- , -/-     = 3

        println!("height: {}", height)
    }
}

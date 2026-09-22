struct solution;

impl solution {
    fn insert_sort(arr: &mut [i32], index:usize, child_node: usize) {
        
        
        
        solution::swap(arr, index, child_node);


    }
    fn create(arr: &mut [i32],current_index: usize) {
        // let current_index = 0;

        let left_child_node = solution::get_left_child(current_index);
        let right_child_node = solution::get_right_child(current_index);

        if arr[current_index] < arr[left_child_node] {
            solution::insert_sort(arr, current_index, left_child_node);
        }

        if arr[current_index] < arr[right_child_node] {
            solution::insert_sort(arr, current_index, right_child_node);
        }

        solution::create(arr, left_child_node);

        solution::create(arr, right_child_node);




    }

    fn swap(arr: &mut [i32], left: usize, right: usize){
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
            return 0
        }
        (index - 1 ) / 2
    }

    fn get_heigh_of_tree(length: usize) -> u32 {
        // log n
        length.ilog2()
    }
}

#[cfg(test)]
mod max_heap_test {
    use super::*;

    #[test]
    fn create_data_structure() {
        let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        solution::create(&mut array, 0);
        let height = solution::get_heigh_of_tree(array.len());

        //            1               = 0
        //     2      ,     3         = 1
        //    4/5     ,    6/7        = 2
        // 8/9 , 10/- , -/- , -/-     = 3

        println!("height: {}", height)
    }
}

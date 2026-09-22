
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

fn sort_right(arr: &mut [i32], index: usize){
    if index >= arr.len(){
        return;
    }

    // sort_right(arr, index);
}

fn sort_arr(arr: &mut [i32],index: usize) -> usize {
    let right_node =  get_right_child(index);
    // let left_node = get_left_child(index);

    if right_node >= arr.len() {
        return index;
    }

    let child = sort_arr(arr,right_node);

    child


}

struct solution;

impl solution {
    fn create(arr: &mut [i32]) {

        sort_arr(arr, 0);

    }
}

#[cfg(test)]
mod max_heap_test {
    use super::*;

    #[test]
    fn create_data_structure() {
        let mut array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        solution::create(&mut array);
        let height = get_heigh_of_tree(array.len());

        //            1               = 0
        //     2      ,     3         = 1
        //    4/5     ,    6/7        = 2
        // 8/9 , 10/- , -/- , -/-     = 3

        println!("height: {}", height)
    }
}

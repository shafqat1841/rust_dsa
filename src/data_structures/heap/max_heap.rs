struct solution;

impl solution {
    fn create(arr: [i32; 10]) {}

    fn get_left_child(index: usize) -> usize {
        index * 2
    }

    fn get_right_child(index: usize) -> usize {
        index * 2 + 1
    }

    fn get_parent(index: usize) -> usize {
        index / 2
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
        let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        solution::create(array);
        let height = solution::get_heigh_of_tree(array.len());
        
        //            1               = 0
        //     2      ,     3         = 1
        //    4/5     ,    6/7        = 2
        // 8/9 , 10/- , -/- , -/-     = 3

        println!("height: {}",height)
    }
}

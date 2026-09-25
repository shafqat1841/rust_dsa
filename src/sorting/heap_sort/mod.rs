use std::fmt::Debug;

use crate::data_structures::heap::max_heap::{build_max_heap,sift_down};

pub fn heap_sort<T: Debug + Ord>(arr: &mut Vec<T>) {
    build_max_heap(arr);
    let len = arr.len();
    if  len <= 1 {
        return;
    }

    

    let mut end = len;
    while end > 1 {
        end -= 1;
        arr.swap(0, end);

        sift_down(arr, 0, end);
    }
}

struct Solution;

impl Solution {
    fn sort<T: Debug + Ord>(arr: &mut Vec<T>) {
        heap_sort(arr);
    }
}

#[cfg(test)]
mod heap_sort_tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut unsorted_arr = vec![30, 27, 25, 20, 18, 17, 10, 5];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }


    #[test]
    fn test_2() {
        let mut unsorted_arr = vec![30, 27, 25, 20, 18, 17, 10, 5];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_3() {
        let mut unsorted_arr = vec![30, 27, 18, 17, 10, 5, 25, 20];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_4() {
        let mut unsorted_arr = vec![5, 10, 17, 18, 30, 27, 25, 20];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_5() {
        let mut unsorted_arr = vec![30, 27, 25, 17, 10, 5, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_6() {
        let mut unsorted_arr = vec![17, 10, 5, 30, 27, 25, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_7() {
        let mut unsorted_arr = vec![5, 10, 17, 30, 27, 25, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_8() {
        let mut unsorted_arr = vec![20, 10, 17, 30, 27, 25, 5, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_9() {
        let mut unsorted_arr: Vec<i32> = vec![];
        let sorted_arr_res: [i32; 0] = [];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_10() {
        let mut unsorted_arr = vec![20];
        let sorted_arr_res = [20];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }
}

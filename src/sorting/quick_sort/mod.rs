mod optimize_arr;
mod quick_sort_1;
mod quick_sort_2;

use optimize_arr::optimize;
use quick_sort_1::quick_sort as quick_sort_1;
use quick_sort_2::quick_sort as quick_sort_2;
struct Solution;

impl Solution {
    fn sort(arr: &mut [i32]) {
        if arr.len() == 0 {
            return;
        }

        optimize(arr);

        quick_sort_2(arr);
    }
    fn sort2(arr: &mut [i32]) {
        if arr.len() == 0 {
            return;
        }

        optimize(arr);

        quick_sort_1(arr);
    }
}

#[cfg(test)]
mod quick_sort_test {
    use super::*;

    #[test]
    fn test_1() {
        let mut unsorted_arr = [30, 27, 25, 20, 18, 17, 10, 5];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    // #[test]
    // fn test_2() {
    //     let mut unsorted_arr = [30, 27, 25, 20, 18, 17, 10, 5];
    //     let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
    //     Solution::sort(&mut unsorted_arr);
    //     assert_eq!(unsorted_arr, sorted_arr_res);
    // }

    // #[test]
    // fn test_3() {
    //     let mut unsorted_arr = [30, 27, 18, 17, 10, 5, 25, 20];
    //     let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
    //     Solution::sort(&mut unsorted_arr);
    //     assert_eq!(unsorted_arr, sorted_arr_res);
    // }

    // #[test]
    // fn test_4() {
    //     let mut unsorted_arr = [5, 10, 17, 18, 30, 27, 25, 20];
    //     let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
    //     Solution::sort(&mut unsorted_arr);
    //     assert_eq!(unsorted_arr, sorted_arr_res);
    // }

    // #[test]
    // fn test_5() {
    //     let mut unsorted_arr = [30, 27, 25, 17, 10, 5, 20, 18];
    //     let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
    //     Solution::sort(&mut unsorted_arr);
    //     assert_eq!(unsorted_arr, sorted_arr_res);
    // }

    // #[test]
    // fn test_6() {
    //     let mut unsorted_arr = [17, 10, 5, 30, 27, 25, 20, 18];
    //     let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
    //     Solution::sort(&mut unsorted_arr);
    //     assert_eq!(unsorted_arr, sorted_arr_res);
    // }

    // #[test]
    // fn test_7() {
    //     let mut unsorted_arr = [5, 10, 17, 30, 27, 25, 20, 18];
    //     let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
    //     Solution::sort(&mut unsorted_arr);
    //     assert_eq!(unsorted_arr, sorted_arr_res);
    // }

    // #[test]
    // fn test_8() {
    //     let mut unsorted_arr = [20, 10, 17, 30, 27, 25, 5, 18];
    //     let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
    //     Solution::sort(&mut unsorted_arr);
    //     assert_eq!(unsorted_arr, sorted_arr_res);
    // }

    // #[test]
    // fn test_9() {
    //     let mut unsorted_arr = [];
    //     let sorted_arr_res = [];
    //     Solution::sort(&mut unsorted_arr);
    //     assert_eq!(unsorted_arr, sorted_arr_res);
    // }

    // #[test]
    // fn test_10() {
    //     let mut unsorted_arr = [20];
    //     let sorted_arr_res = [20];
    //     Solution::sort(&mut unsorted_arr);
    //     assert_eq!(unsorted_arr, sorted_arr_res);
    // }
}

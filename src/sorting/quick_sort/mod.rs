struct Solution;

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    // println!("arr 1: {:?}", arr);
    let pivot_value = arr[low];
    let mut left_low = low + 1;

    for i in low + 1..high {
        // println!("arr loop: {:?}", arr);
        if arr[i] < pivot_value {
            let temp = arr[i];
            arr[i] = arr[left_low];
            arr[left_low] = temp;

            left_low += 1;
        }
    }

    // println!("arr 1.5: {:?}", arr);
    let temp_left_low = arr[left_low - 1];
    arr[left_low - 1] = pivot_value;
    arr[0] = temp_left_low;

    // println!("arr 2: {:?}", arr);

    left_low - 1
}

fn quick_sort(arr: &mut [i32]) {
    let low: usize = 0;
    let high: usize = arr.len();

    if low < high {
        let pivot = partition(arr, low, high);
        // println!("pivot: {:?}", pivot);
        quick_sort(&mut arr[low..pivot]);
        quick_sort(&mut arr[pivot + 1..high]);
    }
}

impl Solution {
    fn sort(arr: &mut [i32]) {
        let low: usize = 0;
        let high: usize = arr.len();

        let mid = (high - 1) / 2;

        if arr[low] > arr[mid] {
            let temp = arr[low];
            arr[low] = arr[mid];
            arr[mid] = temp;
        }

        if arr[low] > arr[high - 1] {
            let temp = arr[low];
            arr[low] = arr[high - 1];
            arr[high - 1] = temp;
        }

        if arr[mid] > arr[high - 1] {
            let temp = arr[mid];
            arr[mid] = arr[high - 1];
            arr[high - 1] = temp;
        }

        let temp = arr[low];
        arr[low] = arr[mid];
        arr[mid] = temp;

        quick_sort(arr);
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

    #[test]
    fn test_2() {
        let mut unsorted_arr = [30, 27, 25, 20, 18, 17, 10, 5];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_3() {
        let mut unsorted_arr = [30, 27, 18, 17, 10, 5, 25, 20];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_4() {
        let mut unsorted_arr = [5, 10, 17, 18, 30, 27, 25, 20];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_5() {
        let mut unsorted_arr = [30, 27, 25, 17, 10, 5, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_6() {
        let mut unsorted_arr = [17, 10, 5, 30, 27, 25, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_7() {
        let mut unsorted_arr = [5, 10, 17, 30, 27, 25, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn test_8() {
        let mut unsorted_arr = [20, 10, 17, 30, 27, 25, 5, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }
}

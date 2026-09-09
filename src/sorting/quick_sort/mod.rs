struct Solution;

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    // println!("arr 1: {:?}", arr);
    let pivot_value = arr[low];
    let mut left_low = low;

    for i in low + 1..high {
        if arr[i] < pivot_value {
            let temp = arr[i];
            arr[i] = arr[left_low];
            arr[left_low] = temp;

            left_low += 1;
        }
    }

    arr[left_low] = pivot_value;
    // println!("arr 2: {:?}", arr);

    // println!("left_low: {:?}", left_low);
    left_low
}

fn quick_sort(arr: &mut [i32]){
    let low: usize = 0;
    let high: usize = arr.len();

    if low < high {
        let pivot = partition(arr, low, high);
        quick_sort(&mut arr[low..pivot]);
        quick_sort(&mut arr[pivot + 1..high]);
        println!("arr 4: {:?}", arr);
    }

}

impl Solution {
    fn sort(arr: &mut [i32]) {
        quick_sort(arr);
    }
}

#[cfg(test)]
mod quick_sort_test {
    use super::*;

    #[test]
    fn check_solution() {
        let mut unsorted_arr = [30, 27, 25, 20, 18, 17, 10, 5];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_2() {
        let mut unsorted_arr = [27, 25, 20, 18, 17, 10, 5];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_3() {
        let mut unsorted_arr = [30, 27, 18, 17, 10, 5, 25, 20];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_4() {
        let mut unsorted_arr = [5, 10, 17, 18, 30, 27, 25, 20];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_5() {
        let mut unsorted_arr = [27, 25, 17, 10, 5, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_6() {
        let mut unsorted_arr = [17, 10, 5, 27, 25, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_7() {
        let mut unsorted_arr = [5, 10, 17, 27, 25, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27];
        Solution::sort(&mut unsorted_arr);
        assert_eq!(unsorted_arr, sorted_arr_res);
    }
}

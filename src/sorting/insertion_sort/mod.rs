struct Solution;

impl Solution {
    fn sort(arr: &mut [i32]) -> &[i32] {
        for i in 1..arr.len() {
            let ele_i = arr[i];
            let mut j = i;

            while j > 0 && arr[j - 1] > ele_i {
                arr[j] = arr[j - 1];
                j -= 1;
            }

            arr[j] = ele_i;
        }

        arr
    }
    fn sort_code_two(arr: &mut [i32]) -> &[i32] {
        let mut i = 1;
        while i < arr.len() as isize {
            let mut j = i - 1;
            let ele_i = arr[i as usize];

            while j > -1 && arr[j as usize] > ele_i {
                let index = j + 1;
                arr[index as usize] = arr[j as usize];

                j -= 1;
            }

            let index = j + 1;
            arr[index as usize] = ele_i;

            i += 1;
        }

        arr
    }
    fn sort_code_one(arr: &mut [i32]) -> &[i32] {
        let mut i = 1;

        while i < arr.len() {
            let mut j = i;

            while j > 0 && arr[j - 1] > arr[j] {
                let elej = arr[j];
                arr[j] = arr[j - 1];
                arr[j - 1] = elej;
                j -= 1;
            }

            i += 1;
        }

        arr
    }
}

#[cfg(test)]
mod insertion_sort_test {
    use super::*;

    #[test]
    fn check_solution() {
        let mut unsorted_arr = [30, 27, 25, 20, 18, 17, 10, 5];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        let sorted_arr = Solution::sort(&mut unsorted_arr);
        assert_eq!(sorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_2() {
        let mut unsorted_arr = [27, 25, 20, 18, 17, 10, 5];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27];
        let sorted_arr =Solution::sort(&mut unsorted_arr);
        assert_eq!(sorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_3() {
        let mut unsorted_arr = [30, 27, 18, 17, 10, 5, 25, 20];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        let sorted_arr =Solution::sort(&mut unsorted_arr);
        assert_eq!(sorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_4() {
        let mut unsorted_arr = [5, 10, 17, 18, 30, 27, 25, 20];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        let sorted_arr =Solution::sort(&mut unsorted_arr);
        assert_eq!(sorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_5() {
        let mut unsorted_arr = [27, 25, 17, 10, 5, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27];
        let sorted_arr =Solution::sort(&mut unsorted_arr);
        assert_eq!(sorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_6() {
        let mut unsorted_arr = [17, 10, 5, 27, 25, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27];
        let sorted_arr =Solution::sort(&mut unsorted_arr);
        assert_eq!(sorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_7() {
        let mut unsorted_arr = [5, 10, 17, 27, 25, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27];
        let sorted_arr =Solution::sort(&mut unsorted_arr);
        assert_eq!(sorted_arr, sorted_arr_res);
    }
}

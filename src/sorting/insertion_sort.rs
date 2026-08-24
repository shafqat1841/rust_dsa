struct solution;

impl solution {
    fn sort(arr: &mut [i32]) -> &[i32] {
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
        let mut unsorted_arr = [20, 18, 17, 10, 5];
        let sorted_arr_res = [5, 10, 17, 18, 20];
        let sorted_arr = solution::sort(&mut unsorted_arr);
        assert_eq!(sorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_2() {
        let mut unsorted_arr = [17, 10, 5, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20];
        let sorted_arr = solution::sort(&mut unsorted_arr);
        assert_eq!(sorted_arr, sorted_arr_res);
    }
}

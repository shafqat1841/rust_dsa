struct solution;

fn merge_arr(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut sorted_arr = Vec::<i32>::new();

    let mut i = 0;
    let mut j = 0;

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            sorted_arr.push(arr1[i]);
            i += 1;
        } else {
            sorted_arr.push(arr2[j]);
            j += 1;
        }
    }

    if i < arr1.len() {
        while i < arr1.len() {
            sorted_arr.push(arr1[i]);
            i += 1;
        }
    }

    if j < arr2.len() {
        while j < arr2.len() {
            sorted_arr.push(arr1[j]);
            j += 1;
        }
    }

    sorted_arr
}

fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() == 1 {
        return arr;
    }

    let mid = arr.len() / 2;

    let left_arr = arr[..mid].to_vec();
    let sorted_left_arr = merge_sort(left_arr);

    let right_arr = arr[mid..].to_vec();
    let sorted_right_arr = merge_sort(right_arr);

    let sorted_arr = merge_arr(sorted_left_arr, sorted_right_arr);

    sorted_arr
}

impl solution {
    fn sort(arr: Vec<i32>) -> Vec<i32> {
        merge_sort(arr)
    }
}

#[cfg(test)]
mod merge_sort_test {
    use super::*;

    #[test]
    fn check_solution() {
        let unsorted_arr = [30, 27, 25, 20, 18, 17, 10, 5];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        let sorted_arr = solution::sort(unsorted_arr.to_vec());
        assert_eq!(sorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_2() {
        let unsorted_arr = [27, 25, 20, 18, 17, 10, 5];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27];
        let sorted_arr = solution::sort(unsorted_arr.to_vec());
        assert_eq!(sorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_3() {
        let unsorted_arr = [30, 27, 18, 17, 10, 5, 25, 20];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        let sorted_arr = solution::sort(unsorted_arr.to_vec());
        assert_eq!(sorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_4() {
        let unsorted_arr = [5, 10, 17, 18, 30, 27, 25, 20];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27, 30];
        let sorted_arr = solution::sort(unsorted_arr.to_vec());
        assert_eq!(sorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_5() {
        let unsorted_arr = [27, 25, 17, 10, 5, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27];
        let sorted_arr = solution::sort(unsorted_arr.to_vec());
        assert_eq!(sorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_6() {
        let unsorted_arr = [17, 10, 5, 27, 25, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27];
        let sorted_arr = solution::sort(unsorted_arr.to_vec());
        assert_eq!(sorted_arr, sorted_arr_res);
    }

    #[test]
    fn check_solution_7() {
        let unsorted_arr = [5, 10, 17, 27, 25, 20, 18];
        let sorted_arr_res = [5, 10, 17, 18, 20, 25, 27];
        let sorted_arr = solution::sort(unsorted_arr.to_vec());
        assert_eq!(sorted_arr, sorted_arr_res);
    }
}

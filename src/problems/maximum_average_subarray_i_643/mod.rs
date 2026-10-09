// max_number_of_k_sum_pairs_1679

#![allow(dead_code)]
#![allow(unused)]

fn sol1(nums: Vec<i32>, k: i32) -> f64 {
    0.0
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        sol1(nums, k)   
    }
}

#[cfg(test)]
mod maximum_average_subarray_i_643_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let nums = [].to_vec();
        let k = 0;
        let res = 0;
        let output = Solution::find_max_average(nums, k);
        assert_eq!(output,res)
    }

    // #[test]
    // fn check_no_1() {
    //     let mut nums = [1, 2, 3, 4].to_vec();
    //     let k = 5;
    //     let correct_output = 2;
    //     let result = Solution::max_operations(nums, k);
    //     assert_eq!(result, correct_output);
    // }
}

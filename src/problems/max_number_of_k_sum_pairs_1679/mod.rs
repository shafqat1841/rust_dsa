// max_number_of_k_sum_pairs_1679

#![allow(dead_code)]
#![allow(unused)]

use std::cmp;

// failed
fn sol1(nums: Vec<i32>, k: i32) -> i32 {
    0
}
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        sol1(nums, k)
    }
}

#[cfg(test)]
mod max_number_of_k_sum_pairs_1679_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let mut nums = [1, 2, 3, 4].to_vec();
        let k = 5;
        let correct_output = 2;
        let result = Solution::max_operations(nums, k);
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_2() {
        let mut nums = [3, 1, 3, 4, 3].to_vec();
        let k = 6;
        let correct_output = 1;
        let result = Solution::max_operations(nums, k);
        assert_eq!(result, correct_output);
    }
}

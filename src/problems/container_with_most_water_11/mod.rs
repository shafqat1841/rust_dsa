// container_with_most_water_11

#![allow(dead_code)]
#![allow(unused)]

use core::prelude::v1;
use std::collections::HashMap;

fn sol1(height: Vec<i32>) -> i32 {
    0
}
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        sol1(height)
    }
}

#[cfg(test)]
mod container_with_most_water_11_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let mut  height = [1,8,6,2,5,4,8,3,7].to_vec();
        let correct_output = 49;
        let result = Solution::max_area(height);
        assert_eq!(result, correct_output);
    }

        #[test]
    fn check_no_2() {
        let mut  height = [1,1].to_vec();
        let correct_output = 1;
        let result = Solution::max_area(height);
        assert_eq!(result, correct_output);
    }
}

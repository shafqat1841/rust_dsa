#![allow(dead_code)]

fn sol1(nums: Vec<i32>) -> Vec<i32> {
    Vec::<i32>::new()
}
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        sol1(nums)
    }
}

#[cfg(test)]
mod product_of_array_except_self_238_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let input = [1, 2, 3, 4];
        let correct_output = [24, 12, 8, 6];
        let result = Solution::product_except_self(input.to_vec());
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_2() {
        let input = [-1,1,0,-3,3];
        let correct_output = [0,0,9,0,0];
        let result = Solution::product_except_self(input.to_vec());
        assert_eq!(result, correct_output);
    }
}

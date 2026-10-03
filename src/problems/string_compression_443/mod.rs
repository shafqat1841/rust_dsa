#![allow(dead_code)]
#![allow(unused)]

fn sol1(chars: &mut Vec<char>) -> i32 {
    0
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        // sol1(nums)
        sol1(chars)
    }
}

#[cfg(test)]
mod string_compression_443_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let mut input = ['a','a','b','b','c','c','c'];
        let correct_output = 6;
        let result = Solution::compress(&mut input.to_vec());
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_2() {
        let mut input = ['a'];
        let correct_output = 1;
        let result = Solution::compress(&mut input.to_vec());
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_3() {
        let mut input = ['a','b','b','b','b','b','b','b','b','b','b','b','b'];
        let correct_output = 4;
        let result = Solution::compress(&mut input.to_vec());
        assert_eq!(result, correct_output);
    }
}

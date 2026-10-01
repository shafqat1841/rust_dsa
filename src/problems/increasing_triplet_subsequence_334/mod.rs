#![allow(dead_code)]


fn sol1(nums: Vec<i32>) -> bool {
    false
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
   pub fn increasing_triplet(nums: Vec<i32>) -> bool {
       sol1(nums)    
    }
}

#[cfg(test)]
mod increasing_triplet_subsequence_334 {
    use super::*;

    #[test]
    fn check_no_1() {
        let input = [1,2,3,4,5];
        let correct_output = true;
        let result = Solution::increasing_triplet(input.to_vec());
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_2() {
        let input = [5,4,3,2,1];
        let correct_output = false;
        let result = Solution::increasing_triplet(input.to_vec());
        assert_eq!(result, correct_output);
    }

      #[test]
    fn check_no_3() {
        let input = [2,1,5,0,4,6];
        let correct_output = true;
        let result = Solution::increasing_triplet(input.to_vec());
        assert_eq!(result, correct_output);
    }
}

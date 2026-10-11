// max_consecutive_ones_iii_1004

#![allow(dead_code)]
#![allow(unused)]

// total time = 
// total space = 
fn sol2(s: String, k: i32) -> i32 {
    0
}


struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        sol1(nums,k)
    }
}

#[cfg(test)]
mod max_consecutive_ones_iii_1004_tests {
    use super::*;

    #[test]
    fn main() {
        let input = [0,0,1,0,1,0,0,1].to_string();
        let k = 1;
        let res = 3;
        let output = Solution::longest_ones(input, k);
        assert_eq!(output,res)
    }
}

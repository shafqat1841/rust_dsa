// find_pivot_index_724

#![allow(dead_code)]
#![allow(unused)]

// [-1,-1,-1,-1,-1,0] = res = 2
// [1,1,,1,3,1,2] , res = -1
// [1,2,3,1,2] , res = 2
// [7, 5, -5] , res = 0
// [1,2,3] , res = -1

// accepted
fn sol1(nums: Vec<i32>) -> i32 {
   let total_sum: i32 = nums.iter().sum(); // time = n , space = 1

   let mut left_sum = 0;

   for (i, &val) in nums.iter().enumerate() {
        let right_sum = total_sum - left_sum - val;

        if right_sum == left_sum {
            return i as i32;
        }

        left_sum += val;
   } // time = n , space = 1
    -1
}
// total time = 2n
// total space = 1

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        sol1(nums)
    }
}

#[cfg(test)]
mod find_pivot_index_724_tests {
    use super::*;

    #[test]
    fn main() {
        let input =  [1,2,3,1,2].to_vec();
        let res = 2;
        let output = Solution::pivot_index(input);
        assert_eq!(output,res)
    }

}

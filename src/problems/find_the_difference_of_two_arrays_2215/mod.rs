// find_the_difference_of_two_arrays_2215

#![allow(dead_code)]
#![allow(unused)]

// accepted
fn sol1(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<Vec<i32>> = Vec::new();
    vec2.push(vec1);
    vec2
}
// total time = n
// total space = 1

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        sol1(nums1,nums2)
    }
}

#[cfg(test)]
mod find_the_difference_of_two_arrays_2215_tests {
    use super::*;

    #[test]
    fn main() {
        let input =  [1,2,3].to_vec();
        let input2 =  [2,4,6].to_vec();
        let res = [[1,3],[4,6]].to_vec();
        let output = Solution::find_difference(input,input2);
        assert_eq!(output,res)
    }

}

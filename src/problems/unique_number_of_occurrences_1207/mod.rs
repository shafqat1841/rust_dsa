// unique_number_of_occurrences_1207

#![allow(dead_code)]
#![allow(unused)]

// [1,2,2,3,3,3] = true
// [1,2,3] = false
// 
fn sol1(arr: Vec<i32>) -> bool {
   false
}
// total time = 2n + 2m
// total space = 5n + 5m

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        sol1(arr)
    }
}

#[cfg(test)]
mod unique_number_of_occurrences_1207_tests {
    use super::*;

    #[test]
    fn main() {
        let input =  [1,2,2,3,3,3].to_vec();
        let res = true;
        let output = Solution::unique_occurrences(input);
        assert_eq!(output,res)
    }

}

// maximum_number_of_vowels_in_a_substring_of_given_length_1456

#![allow(dead_code)]
#![allow(unused)]

fn sol1(s: String, k: i32) -> i32 {
    0
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_vowels(s: String, k: i32) -> i32 {
        sol1(s,k)   
    }
}

#[cfg(test)]
mod maximum_number_of_vowels_in_a_substring_of_given_length_1456_tests {
    use super::*;

    #[test]
    fn main() {
        let s = "abciiidef".to_string();
        let k = 3;
        let res = 3;
        let output = Solution::max_vowels(s, k);
        assert_eq!(output,res)
    }
}

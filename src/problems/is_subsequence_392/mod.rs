// is_subsequence_392

#![allow(dead_code)]
#![allow(unused)]

fn sol1(s: String, t: String) -> bool {
    false
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        sol1(s, t)
    }
}

#[cfg(test)]
mod is_subsequence_392_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let mut s = "abc".to_string();
        let mut t = "ahbgdc".to_string();
        let correct_output = true;
        let result = Solution::is_subsequence(s, t);
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_2() {
        let mut s = "axc".to_string();
        let mut t = "ahbgdc".to_string();
        let correct_output = false;
        let result = Solution::is_subsequence(s, t);
        assert_eq!(result, correct_output);
    }
}

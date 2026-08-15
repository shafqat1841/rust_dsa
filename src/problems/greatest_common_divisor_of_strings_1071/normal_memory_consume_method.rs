#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if format!("{}{}", str1, str2) != format!("{}{}", str2, str1) {
            return "".to_string();
        }
        let hcf = calculate_hcf(str1.len(), str2.len());
        str1[..hcf].to_string() // str1 is still valid here because it was never moved!
    }
}

fn calculate_hcf(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

#[cfg(test)]
mod merge_strings_alternately_1768_test_2 {
    use super::*;

    #[test]
    fn check_no_1() {
        let result = Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string());
        assert_eq!(result, "ABC");
    }

    #[test]
    fn check_no_2() {
        let result = Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string());
        assert_eq!(result, "AB");
    }

    #[test]
    fn check_no_3() {
        let result = Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string());
        assert_eq!(result, "");
    }

    #[test]
    fn check_no_4() {
        let result = Solution::gcd_of_strings("AAAAAB".to_string(), "AAA".to_string());
        assert_eq!(result, "");
    }
}

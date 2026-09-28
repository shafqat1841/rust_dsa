fn sol1(s: String) -> String {
    "".to_string()
}

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_vowels(s: String) -> String {
        sol1(s)
    }
}

#[cfg(test)]
mod reverse_vowels_345_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let s = "IceCreAm".to_string();
        let correct_result = "AceCreIm".to_string();
        let result = Solution::reverse_vowels(s);
        assert_eq!(result, correct_result);
    }

        #[test]
    fn check_no_2() {
        let s = "leetcode".to_string();
        let correct_result = "leotcede".to_string();
        let result = Solution::reverse_vowels(s);
        assert_eq!(result, correct_result);
    }

}

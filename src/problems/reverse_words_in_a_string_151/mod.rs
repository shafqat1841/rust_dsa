fn sol1(s: String) -> String {
    "".to_string()
}

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_words(s: String) -> String {
        sol1(s)
    }
}

#[cfg(test)]
mod reverse_words_in_a_string_151_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let s = "the sky is blue".to_string();
        let correct_result = "blue is sky the".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_2() {
        let s = "  hello world  ".to_string();
        let correct_result = "world hello".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_3() {
        let s = "a good   example".to_string();
        let correct_result = "example good a".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, correct_result);
    }
}

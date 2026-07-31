struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod merge_strings_alternately_1768_test {
    use super::*;

    #[test]
    fn it_works() {
       let result = Solution::merge_alternately( "".to_string(), "".to_string());
        assert_eq!(result, "");
    }
}
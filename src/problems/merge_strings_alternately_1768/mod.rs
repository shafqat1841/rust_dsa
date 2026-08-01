#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let word1_bytes = word1.into_bytes();
        let word2_bytes = word2.into_bytes();

        let word1_length = word1_bytes.len();
        let word2_length = word2_bytes.len();
        
        let word3_length = word1_length + word2_length;
        
        let mut word3 = Vec::<u8>::new();

        for i in 0..word3_length {
            let word1_char = word1_bytes.get(i);
            let word2_char = word2_bytes.get(i);
            if let Some(v) = word1_char {
                word3.push(*v);
            }
            if let Some(v) = word2_char {
                word3.push(*v);
            }
        }

        match String::from_utf8(word3) {
            Ok(string) => string,
            Err(e) => format!("Invalid UTF-8 bytes: {}", e),
        }
    }
}

#[cfg(test)]
mod merge_strings_alternately_1768_test {
    use super::*;

    #[test]
    fn check_no_1() {
        let result = Solution::merge_alternately("abc".to_string(), "def".to_string());
        assert_eq!(result, "adbecf");
    }

    #[test]
    fn check_no_2() {
        let result = Solution::merge_alternately("abc".to_string(), "pqr".to_string());
        assert_eq!(result, "apbqcr");
    }

    #[test]
    fn check_no_3() {
        let result = Solution::merge_alternately("ab".to_string(), "pqrs".to_string());
        assert_eq!(result, "apbqrs");
    }

    #[test]
    fn check_no_4() {
        let result = Solution::merge_alternately("abcd".to_string(), "pq".to_string());
        assert_eq!(result, "apbqcd");
    }
}

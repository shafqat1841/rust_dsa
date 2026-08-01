#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {

        let str1_bytes = str1.into_bytes();
        let str2_bytes = str2.into_bytes();

        let mut res = Vec::<u8>::new();

        let mut p1 = 0;
        let mut p2 = 0;

        loop {
            let char1 = str1_bytes.get(p1); 
            let char2 = str2_bytes.get(p2); 

            p1 += 1;
            p2 += 1;

            let char1 = match char1 {
                Some(v) => v,
                None => break
            };

            let char2 = match char2 {
                Some(v) => v,
                None => break
            };

            if char1 == char2 {
                res.push(*char1);
            }
        }








        match String::from_utf8(res){
            Ok(v) => v,
            Err(e) => e.to_string()
        }
    }
}

#[cfg(test)]
mod merge_strings_alternately_1768_test {
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

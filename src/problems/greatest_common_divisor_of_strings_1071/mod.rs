#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let len1 = str1.len();
        let len2 = str2.len();

        let hcf = calculate_hcf(len1, len2);

        println!("hcf: {}", hcf);

        let mut o = str2;

        if len1 < len2 {
            o = str1
        }

        let o_bytes = o.into_bytes();

        let res_bytes = &o_bytes[..hcf];

        let res = match String::from_utf8(res_bytes.to_vec()) {
            Ok(str_slice) => str_slice,
            Err(e) => "".to_string(),
        };

        res
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

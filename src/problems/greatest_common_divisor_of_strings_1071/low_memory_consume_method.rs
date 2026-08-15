#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        
        let str1_bytes = str1.as_bytes();
        let str2_bytes = str2.as_bytes();
        
        let str1_len = str1_bytes.len();
        let str2_len = str2_bytes.len();

        let hcf = calculate_hcf(str1_len, str2_len);

        let res_bytes = &str2_bytes[..hcf];
        let mut longer_length = str2_len;
        
        if str1_len > str2_len {
            longer_length = str1_len;
        }


        let mut run = 0;
        let mut all_match = true;


        loop {
            let start = run;
            let end = run + hcf;
            
            if run < str1_len {
                let str1_com = &str1_bytes[start..end];
                let com1 = *str1_com == *res_bytes;
                
                if !com1 {
                    all_match = false;
                    break;
                }
            }
            
            if run < str2_len {
                let str2_com = &str2_bytes[start..end];
                let com2 = *str2_com == *res_bytes;
                if !com2 {
                    all_match = false;
                    break;
                }
            }
            
            if  run >= longer_length {
                break;
            }

            run += hcf;

        }

        if !all_match {
            "".to_string()
        } else {
            let res = match str2.get(..hcf) {
                None =>   "",
                Some(res) => res
            };

            res.to_string()
            
        }
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

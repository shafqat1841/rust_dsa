// removing_stars_from_a_string_2390

#![allow(dead_code)]
#![allow(unused)]

// accepted
fn sol1(s: String) -> String {
    let s = s.as_bytes();

    let mut res_bytes = Vec::<u8>::new();

    for i in 0..s.len() {
        let ele = s[i];

        if ele == b'*' {
            if !res_bytes.is_empty() {
                res_bytes.pop();
            }
        } else {
            res_bytes.push(ele);
        }

    }

    String::from_utf8(res_bytes).unwrap()
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_stars(s: String) -> String {
        sol1(s)
    }
}

#[cfg(test)]
mod removing_stars_from_a_string_2390_tests {
    use super::*;

    #[test]
    fn main() {
        let input =  "sha*fq*at".to_string();
        let res = "shfat".to_string();
        let output = Solution::remove_stars(input);
        assert_eq!(output,res)
    }

}

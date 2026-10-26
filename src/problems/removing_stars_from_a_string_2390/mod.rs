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

    } // time = n , space = n

    String::from_utf8(res_bytes).unwrap()
}
// total time = n
// total space = n

// let input =  "sha*fq*at".to_string();
// let res = "shfat".to_string();   

// accepted and optimized
fn sol2(s: String) -> String {
    let mut s = s.into_bytes();
    let mut write = 0;

    for i in 0..s.len() {
        let ele = s[i];

        if ele == b'*' {
            if write > 0 {
                write -= 1;
            }
        } else {
            s[write] = ele;
            write += 1;
        }

    } // time = n , space = 1

    s.truncate(write);

    String::from_utf8(s).unwrap()
}
// total time = n
// total space = 1

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_stars(s: String) -> String {
        sol2(s)
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

// removing_stars_from_a_string_2390

#![allow(dead_code)]
#![allow(unused)]

//
fn sol1(s: String) -> String {
    "".to_string()
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
        let input =  "".to_string();
        let res = "".to_string();
        let output = Solution::remove_stars(input);
        assert_eq!(output,res)
    }

}

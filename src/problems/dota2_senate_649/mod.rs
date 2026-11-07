// dota2_senate_649

#![allow(dead_code)]
#![allow(unused)]

fn sol1(senate: String) -> String {
    "".to_string()        
}

struct Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        sol1(senate)
    }
}
#[cfg(test)]
mod dota2_senate_649_tests {
    use super::*;


    #[test]
    fn main() {
        let input = "RD".to_string();
        let res = Solution::predict_party_victory(input);
        let output = "Radiant".to_string();
        assert_eq!(output, res);

    }

}

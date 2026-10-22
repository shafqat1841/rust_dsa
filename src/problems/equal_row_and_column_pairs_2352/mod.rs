// equal_row_and_column_pairs_2352

#![allow(dead_code)]
#![allow(unused)]

//
fn sol1(grid: Vec<Vec<i32>>) -> i32 {
    0
}
// total time =
// total space =

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        sol1(grid)
    }
}

#[cfg(test)]
mod equal_row_and_column_pairs_2352_tests {
    use super::*;

    #[test]
    fn main() {
        let input =  [[3,2,1].to_vec(),[1,7,6].to_vec(),[2,7,7].to_vec()].to_vec();
        let res = 1;
        let output = Solution::equal_pairs(input);
        assert_eq!(output,res)
    }

}

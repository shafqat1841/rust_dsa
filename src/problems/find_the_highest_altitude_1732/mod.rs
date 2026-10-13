// find_the_highest_altitude_1732

#![allow(dead_code)]
#![allow(unused)]

//    [-5,1,5,0,-7]
// [0,-5,-4,1,1,-6]
//  0 - 5   = -5
// -5 + 1   = -4
// -4 + 5   = 1
//  1 + 0   = 1
//  1 - 7   = -6

fn sol1(gain: Vec<i32>) -> i32 {
   0
}
// total time =
// total space =

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        sol1(gain)
    }
}

#[cfg(test)]
mod find_the_highest_altitude_1732tests {
    use super::*;

    #[test]
    fn main() {
        let input =  [-5,1,5,0,-7].to_vec();
        let res = 1;
        let output = Solution::largest_altitude(input);
        assert_eq!(output,res)
    }

}

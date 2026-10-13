// find_the_highest_altitude_1732

#![allow(dead_code)]
#![allow(unused)]

//    [-5,1,5,0,-7]

//  i     +   (i + 1) = res
// -----------------------------
//  0     +     -5    = -5
// -5     +      1    = -4
// -4     +      5    =  1
//  1     +      0    =  1
//  1     +     -7    = -6

// [0,-5,-4,1,1,-6]

// 1 ans

// [1,2,3,4]

//  i     +   (i + 1) = res
// -----------------------------
//  0     +      1    =  1
//  1     +      2    =  3
//  3     +      3    =  6
//  6     +      4    =  10

// [0,2,4,7,11]

// 11 ans

// accepted
fn sol1(gain: Vec<i32>) -> i32 {
    let mut heighest = 0;
    
    let mut res = 0;
    
    for i in 0..gain.len() {
        let value = gain[i];
        res = value + res;

        if res > heighest {
            heighest = res;
        }
    } // time = n , space = 1

    heighest
}
// total time = n
// total space = 1

// another solution in rust idomatic way 
fn sol2(gain: Vec<i32>) -> i32 {
    gain.iter()
        .scan(0, |altitude, &g| {
            *altitude += g;
            Some(*altitude)
        })
        .max()
        .unwrap_or(0)
        .max(0) // Ensuring 0 is considered since trip starts at altitude 0
}

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        sol1(gain)
    }
}

#[cfg(test)]
mod find_the_highest_altitude_1732_tests {
    use super::*;

    #[test]
    fn main() {
        let input =  [1,2,3,4].to_vec();
        let res = 10;
        let output = Solution::largest_altitude(input);
        assert_eq!(output,res)
    }

}

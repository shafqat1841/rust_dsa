#![allow(dead_code)]
#![allow(unused)]

// move_zeroes_283

// [0, 0, 0, 1, 0, 3, 12]
// [1, 0, 0, 0, 0, 3, 12]
// [0, 1, 0, 0, 3, 0, 0, 0, 0, 12]
// [0]
// [1]
// []

// accepted
fn sol1(nums: &mut Vec<i32>) {
    let mut zi: usize = 0;
    let mut nzi: usize = 0;

    while nzi < nums.len() {
        
        if nums[nzi] != 0 {
            if nzi != zi {
                nums.swap(nzi, zi);
            }
            zi += 1;
        }

        nzi += 1;
    } // time = n , space = 1
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        sol1(nums)
    }
}

#[cfg(test)]
mod move_zeroes_283_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let mut input = [0, 1, 0, 3, 12].to_vec();
        let correct_output = [1, 3, 12, 0, 0];
        let result = Solution::move_zeroes(&mut input);
        assert_eq!(input, correct_output);
    }

    #[test]
    fn check_no_2() {
        let mut input = [0].to_vec();
        let correct_output = [0];
        let result = Solution::move_zeroes(&mut input);
        assert_eq!(input, correct_output);
    }
}

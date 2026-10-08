// container_with_most_water_11

#![allow(dead_code)]
#![allow(unused)]

use std::cmp;

// input  = [1,8,6,2,5,4,8,3,7]
// output = 49

// input  = [1,1]
// output = 1

// ia = first index
// ib = second index
// h * w
// h = min(input[ia], input[ib]) // need to be as greatest as possible
// w = ia - ib // need to be as greatest as possible

// failed
fn sol1(height: Vec<i32>) -> i32 {
    let mut output = 0;

    let mut ia = 0;
    let ib1 = height.len() - 1;
    let eleb1 = height[ib1];
    while ia < height.len() {
        let elea = height[ia];

        let height = cmp::min(elea, eleb1);

        let width = ib1 - ia;

        let res = height * (width as i32);

        if output < res {
            output = res;
        }

        ia += 1;
    }

    let mut ib = height.len();
    let ia1 = 0;
    let elea1 = height[ia1];

    while ib > 0 {
        ib -= 1;
        let eleb = height[ib];

        let height = cmp::min(elea1, eleb);

        let width = ib - ia1;

        let res = height * (width as i32);

        if output < res {
            output = res;
        }
    }

    output
}


// accepted but not optimized
fn sol2(height: Vec<i32>) -> i32 {

    let mut i_s = 0;
    let mut i_e = height.len() - 1;

    let mut max_area = 0;

    while i_s < i_e {
        let ele1 = height[i_s];
        let ele2 = height[i_e];

        let height = cmp::min(ele1, ele2);

        let width = i_e - i_s;

        let area = height * (width as i32);

        println!("area: {}",area);

        if area > max_area {
            max_area = area;
        }

        if ele1 < ele2 {
            i_s += 1;
        } else {
            i_e -= 1;
        }
    }

    max_area
}
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_area(height: Vec<i32>) -> i32 {
        sol2(height)
    }
}

#[cfg(test)]
mod container_with_most_water_11_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let mut height = [1, 8, 6, 2, 5, 4, 8, 3, 7].to_vec();
        let correct_output = 49;
        let result = Solution::max_area(height);
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_2() {
        let mut height = [1, 1].to_vec();
        let correct_output = 1;
        let result = Solution::max_area(height);
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_3() {
        let mut height = [8, 7, 2, 1].to_vec();
        let correct_output = 7;
        let result = Solution::max_area(height);
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_4() {
        let mut height = [2, 3, 4, 5, 18, 17, 6].to_vec();
        let correct_output = 17;
        let result = Solution::max_area(height);
        assert_eq!(result, correct_output);
    }
}

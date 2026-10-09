// max_number_of_k_sum_pairs_1679

#![allow(dead_code)]
#![allow(unused)]


// not accepted because of Time Limit Exceeded
fn sol1(nums: Vec<i32>, k: i32) -> f64 {
    let mut max_ave = f64::MIN;

    let range = k - 1;

    let mut sp: i32 = range ;
    
    while sp < nums.len() as i32 {

        let mut lk = sp - range;

        let mut numbers: f64 = 0.00000;

        while lk <= sp {
            let ele = nums[lk as usize];
            numbers += ele as f64;
            lk += 1;
        }

        let ave = numbers / k as f64;

        if ave > max_ave {
            max_ave = ave 
        }


        sp += 1;

    }

    // println!("max_ave: {:?}",max_ave);
    let multiplier = 10_f64.powi(5);
    let rounded = (max_ave * multiplier).round() / multiplier;
    rounded

}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        sol1(nums, k)   
    }
}

#[cfg(test)]
mod maximum_average_subarray_i_643_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let nums = [1,2,3,4,5,6].to_vec();
        // 4 + 5 + 6 / 3 => 15 / 3 => 5
        let k = 3;
        let res: f64 = 5.00000;
        let output = Solution::find_max_average(nums, k);
        assert_eq!(output,res)
    }

    #[test]
    fn check_no_2() {
        let nums = [-1,-2,-3,-4,-5,-6].to_vec();
        // 4 + 5 + 6 / 3 => 15 / 3 => 5
        let k = 3;
        let res: f64 = -2.0;
        let output = Solution::find_max_average(nums, k);
        assert_eq!(output,res)
    }

    #[test]
    fn check_no_3() {
        let nums = [2].to_vec();
        // 4 + 5 + 6 / 3 => 15 / 3 => 5
        let k = 1;
        let res: f64 = 2.00000;
        let output = Solution::find_max_average(nums, k);
        assert_eq!(output,res)
    }

       #[test]
    fn check_no_4() {
        let nums = [1,1,2,2,3,3,4,4,5,5,6,6].to_vec();
        // 4 + 5 + 5 + 6 + 6 / 5 => 26 / 5 => 5.20000
        let k = 5;
        let res: f64 = 5.20000;
        let output = Solution::find_max_average(nums, k);
        assert_eq!(output,res)
    }

    #[test]
    fn check_no_5() {
        let nums = [1,1,2,2,3,3,4,4,5,0,0,6].to_vec();
        // 4 + 4 + 5 / 3 => 13 / 3 => 4.33333
        let k = 3;
        let res: f64 = 4.33333;
        let output = Solution::find_max_average(nums, k);
        assert_eq!(output,res)
    }

    #[test]
    fn check_no_6() {
        let nums = [1,1,2].to_vec();
        // 1 + 1 + 2 / 3 => 4 / 3 => 1.33333
        let k = 3;
        let res: f64 = 1.33333;
        let output = Solution::find_max_average(nums, k);
        assert_eq!(output,res)
    }
}

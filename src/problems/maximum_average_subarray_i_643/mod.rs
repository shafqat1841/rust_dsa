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
    max_ave

}

// wrong because there may be 0 at the last parts of unsorted array
fn sol2(mut nums: Vec<i32>, k: i32) -> f64 {
    println!("nums: {:?}",nums);
    nums.sort();
    println!("nums: {:?}",nums);
    let before_last_value = nums.len() - k as usize;
    println!("before_last_value: {:?}",before_last_value);
    let last_k_nums = &nums[before_last_value..];
    println!("before_last_value: {:?}",before_last_value);
    
    let mut max_val = 0.0;
    
    for val in last_k_nums {
        max_val += *val as f64;
    }
    
    let value = max_val / k as f64;
    
    value

}


// accepted
// total time = n
// total space = 1
fn sol3(nums: Vec<i32>, k: i32) -> f64 {
    let mut max_ave = f64::MIN;

    let mut p = 0;
    
    let mut local_val = 0;
    while p < k {
        
        let ele = nums[p as usize];
        
        local_val += ele;

        p += 1;
    } // time = k , space = 1

    let new_max_ave = local_val as f64 / k as f64;
    
    if new_max_ave > max_ave {
        max_ave = new_max_ave
    }


    while p < nums.len().try_into().unwrap() {

        let first_index = p - k;

        let first_ele = nums[first_index as usize];

        local_val = local_val - first_ele;

        let p_ele = nums[p as usize];

        local_val = local_val + p_ele;

        let new_max_ave = local_val as f64 / k as f64;

         if new_max_ave > max_ave {
            max_ave = new_max_ave
        }

        p += 1;

    } // time = k + (n - k) => n , space = 1

    

    max_ave

}


// accepted and idiomatic
// total time = n
// total space = 1
fn sol4(nums: Vec<i32>, k: i32) -> f64 {
   let k = k as usize;
        
    let mut current_sum: i32 = nums[..k].iter().sum();
    let mut max_sum = current_sum;

    for i in k..nums.len() {
        current_sum = current_sum - nums[i - k] + nums[i];
        if current_sum > max_sum {
            max_sum = current_sum;
        }
    }

    max_sum as f64 / k as f64

}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        sol4(nums, k)   
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
        // 4 + 4 + 5 / 3 => 13 / 3 => 4.333333333333333
        let k = 3;
        let res: f64 = 4.333333333333333;
        let output = Solution::find_max_average(nums, k);
        assert_eq!(output,res)
    }

    #[test]
    fn check_no_6() {
        let nums = [1,1,2].to_vec();
        // 1 + 1 + 2 / 3 => 4 / 3 => 1.3333333333333333
        let k = 3;
        let res: f64 = 1.3333333333333333;
        let output = Solution::find_max_average(nums, k);
        assert_eq!(output,res)
    }
}

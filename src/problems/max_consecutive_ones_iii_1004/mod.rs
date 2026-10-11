// max_consecutive_ones_iii_1004

#![allow(dead_code)]
#![allow(unused)]

// total time = 
// total space = 
// [1,1,1,0,0,0,1,1,1,1,0] , k = 2 , res = 6
fn sol0(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    
    let mut limit = k;
    
    let mut count = 0;

    let mut i = 0;
    let len = nums.len();

    while i < len {
        let ele = nums[i];
        
        // [0,0,1,0,1,0,0,1] , k = 1 , res = 3
        // res = 2 , limit = 0 , count = 1
        // i = 4 , ele = 1
        if ele == 1 {
            count += 1;
            i += 1;
        } else if limit > 0 {
            count += 1;
            limit -= 1;
            i += 1;
        } else {
            
            if count > res {
                res = count;
            }

            limit = k;
            count = 0;
    
        }

    }

    res
}

// total time = 
// total space = 
// [1,1,1,0,0,0,1,1,1,1,0] , k = 2 , res = 6
// [0,0,0,1] , k = 4 , res = 4
fn sol1(nums: Vec<i32>, k: i32) -> i32 {
    let len = nums.len();
    
    
    let mut res = 0;
    let mut limit = k;
    let mut count = 0;
    
    let mut i = 0;
    let mut j = 0;
    
    while i < len && j < len {

        if nums[j] == 1 {
            count += 1;
            j += 1;
        } else if limit > 0 {
            count += 1;
            limit -= 1;
            j += 1;
        } else {     
            if count > res {
                res = count;
            }
            limit = k;
            count = 0;
            i += 1;
            j = i;
        }
        
    }

    if count > res {
        res = count;
    }

    res
}


struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        sol1(nums,k)
    }
}

#[cfg(test)]
mod max_consecutive_ones_iii_1004_tests {
    use super::*;

    #[test]
    fn main() {
        let input = [0,0,1,0,1,0,0,1].to_vec();
        let k = 1;
        let res = 3;
        let output = Solution::longest_ones(input, k);
        assert_eq!(output,res)
    }

}

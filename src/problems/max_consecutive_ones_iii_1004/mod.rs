// max_consecutive_ones_iii_1004

#![allow(dead_code)]
#![allow(unused)]

// total time = N ^ 3
// total space = 1
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

// total time = 2n or n
// total space = 1
// [1,1,1,0,0,0,1,1,1,1,0] , k = 2 , res = 6
// [0,0,0,1] , k = 4 , res = 4
fn sol2(nums: Vec<i32>, k: i32) -> i32 {
    let mut left = 0;
    let mut zeros_count = 0;
    let mut max_len = 0;

    for right in 0..nums.len() {
        // Expand the window by including nums[right]
        if nums[right] == 0 {
            zeros_count += 1;
        }

        // If zero count exceeds k, contract the window from the left
        while zeros_count > k {
            if nums[left] == 0 {
                zeros_count -= 1;
            }
            left += 1;
        } // worst case time = left < n < right , space = 1

        // Update the maximum length found so far
        max_len = max_len.max((right - left + 1) as i32);
    } // time = n , space = 1

    max_len
}


struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        sol2(nums,k)
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

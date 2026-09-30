#![allow(dead_code)]

fn sol1(nums: Vec<i32>) -> Vec<i32> {
    println!("nums 1: {:?}", nums);
    let mut l = 1;
    let mut r = 1;
    let len = nums.len();
    let mut i = 0;
    let mut j = len;

    let mut nums2 = nums.clone();
    
    while i < len {
        let number = if i == 0 { 1 } else { nums[i - 1] };
        let ans = l * number;
        l = ans;
        nums2[i] = ans;
        i += 1;
    }
    println!("nums2: {:?}", nums2);
    
    while j > 0 {
        j -= 1;
        let number = if j == len - 1 { 1 } else { nums[j + 1] };
        let ans = r * number;
        r = ans;
        nums2[j] = nums2[j] * ans;
    }
    println!("nums2: {:?}", nums2);

    nums2
}


fn sol2(nums: Vec<i32>) -> Vec<i32> {
    println!("nums 1: {:?}", nums);
    let mut l = 1;
    let mut r = 1;
    let len = nums.len();
    let mut i = 0;
    let mut j = len;

    let mut nums2 = nums.clone();
    
    while i < len {
        let number = if i == 0 { 1 } else { nums[i - 1] };
        let ans = l * number;
        l = ans;
        nums2[i] = ans;
        i += 1;
    }
    println!("nums2: {:?}", nums2);
    
    while j > 0 {
        j -= 1;
        let number = if j == len - 1 { 1 } else { nums[j + 1] };
        let ans = r * number;
        r = ans;
        nums2[j] = nums2[j] * ans;
    }
    println!("nums2: {:?}", nums2);

    nums2
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // sol1(nums)
        sol2(nums)
    }
}

#[cfg(test)]
mod product_of_array_except_self_238_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let input = [1, 2, 3, 4];
        let correct_output = [24, 12, 8, 6];
        let result = Solution::product_except_self(input.to_vec());
        assert_eq!(result, correct_output);
    }

        #[test]
        fn check_no_2() {
            let input = [-1, 1, 0, -3, 3];
            let correct_output = [0, 0, 9, 0, 0];
            let result = Solution::product_except_self(input.to_vec());
            assert_eq!(result, correct_output);
        }
}

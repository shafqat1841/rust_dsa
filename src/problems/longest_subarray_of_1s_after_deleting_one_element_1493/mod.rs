// longest_subarray_of_1s_after_deleting_one_element_1493

#![allow(dead_code)]
#![allow(unused)]

// [1,0,1]
// [1,1] = 2

// [1,0,1,1,1,0,1]
// [1,0,1,1,1,1] = 4

// [0,1,1]
// [1,1] = 2
 
// [1,1,0]
// [1,1] = 2

// [1,1,1]
// [1,1] = 2

// [0,0]
// [] = 0

// [0,1,1,1,0,1,1,0,1]
// [0,1,1,1,1,1,0,1] = 5

// [0,1,1,1,1,1,0]
// [1,1,1,1,1,0] = 5


fn sol1(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let limit = 1;
    
    let mut left = 0;
    let mut right = 0;
    let mut count = 0;
    let mut size = 0;

   while right < len {

    if nums[right] == 0 {
        count += 1;
    }

    while count > limit {
        if nums[left] == 0 {
            count -= 1;
        }
        left += 1;
    } // time = k and space = 1
    
    if right - left > size {
        size = right - left;
    }
    right += 1;
} // time = n and space = 1

size as i32
}
// total time = 2n or n
// total space = 1

fn sol2(nums: Vec<i32>) -> i32 {
    let mut max_len = 0;
    let mut prev_len = 0;
    let mut curr_len = 0;

    for &num in &nums {
        if num == 1 {
            curr_len += 1;
        } else {
            max_len = max_len.max(prev_len + curr_len);
            prev_len = curr_len;
            curr_len = 0;
        }
    } // time = n , space = 1

    max_len = max_len.max(prev_len + curr_len);

    if max_len == nums.len() as i32 {
        max_len - 1
    } else {
        max_len
    }
}
// total time = n
// total space = 1

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        sol2(nums)
    }
}

#[cfg(test)]
mod longest_subarray_of_1s_after_deleting_one_element_1493_tests {
    use super::*;

    #[test]
    fn main() {
        let input = [1,1,0,1].to_vec();
        let res = 3;
        let output = Solution::longest_subarray(input);
        assert_eq!(output,res)
    }

}

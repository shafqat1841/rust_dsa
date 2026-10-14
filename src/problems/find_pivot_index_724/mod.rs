// find_pivot_index_724

#![allow(dead_code)]
#![allow(unused)]

// [1,2,3,1,2] , res = 2
// [7, 5, -5] , res = 0
// [1,2,3] , res = -1
// [-1,-1,-1,-1,-1,0] = res = 2
// [-1, -2, -3] , [0, -1, -2]
fn sol1(nums: Vec<i32>) -> i32 {
    let mut res: i32 = -1;

    let mut l = 0;
    let mut r = nums.len() - 1;

    let mut l_sum = 0;
    let mut r_sum = 0;

    let mut sum_left: Vec<i32> = Vec::new();
    let mut sum_right: Vec<i32> = Vec::new();

    while l < r {

        
        
        l_sum = nums[l] + l_sum;
        r_sum = nums[r] + r_sum;
        
        
        sum_left.push(l_sum);
        sum_right.push(r_sum);
        
        l += 1;
        r -= 1;
    }

    if l_sum == r_sum {
        res = l as i32
    }
    
    res


}
// total time = 
// total space = 

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        sol1(nums)
    }
}

#[cfg(test)]
mod find_pivot_index_724_tests {
    use super::*;

    #[test]
    fn main() {
        // let input =  [1,7,3,6,5,6].to_vec();
        let input =  [1,2,3,1,2].to_vec();
        let res = 2;
        let output = Solution::pivot_index(input);
        assert_eq!(output,res)
    }

}

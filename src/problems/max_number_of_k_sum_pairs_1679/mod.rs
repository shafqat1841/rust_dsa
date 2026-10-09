// max_number_of_k_sum_pairs_1679

#![allow(dead_code)]
#![allow(unused)]

use std::collections::HashMap;

// println!("nums: {:?}",nums);
// println!("found_nums: {:?}",found_nums);
// [1, 2, 3, 4] / 5
// [3, 1, 3, 4, 3] / 6

// [2,2,2,3,1,1,4,1] / 4 = 2

//  1       2       2     2 1 2 1 1
// [2,5,4,4,1,3,4,4,1,4,4,1,2,1,2,2,3,2,4,2]
// 3
fn sol1(nums: Vec<i32>, k: i32) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    let mut found_nums = 0;

    for val in nums {
        println!("val: {:?}", val);

        if k < val || val == k  {
            continue;
        }

        let key = k - val;

        if map.contains_key(&val) {
            println!("before remove map: {:?}",map);
            found_nums += 1;
            map.remove(&val);
            println!("after remove map: {:?}",map);
            println!("found_nums: {:?}",found_nums);
        } else {
            map.insert(key, val);
            println!("insert map: {:?}",map);
        }
        println!("---------------------------------------------");
    }

    found_nums
}
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        sol1(nums, k)
    }
}

#[cfg(test)]
mod max_number_of_k_sum_pairs_1679_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let mut nums = [1, 2, 3, 4].to_vec();
        let k = 5;
        let correct_output = 2;
        let result = Solution::max_operations(nums, k);
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_2() {
        let mut nums = [3, 1, 3, 4, 3].to_vec();
        let k = 6;
        let correct_output = 1;
        let result = Solution::max_operations(nums, k);
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_3() {
        let mut nums = [2, 2, 2, 3, 1, 1, 4, 1].to_vec();
        let k = 4;
        let correct_output = 2;
        let result = Solution::max_operations(nums, k);
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_4() {
        let mut nums = [2, 5, 4, 4, 1, 3, 4, 4, 1, 4, 4, 1, 2, 1, 2, 2, 3, 2, 4, 2].to_vec();
        let k = 3;
        let correct_output = 4;
        let result = Solution::max_operations(nums, k);
        assert_eq!(result, correct_output);
    }
}

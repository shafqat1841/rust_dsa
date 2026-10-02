#![allow(dead_code)]

// not accepted by leetcode because
// Time Limit Exceeded 78 / 87 testcases passed
// and also because it a brute force and of the time complexity O(n^3) which is not optimal for this problem 

fn sol1(nums: Vec<i32>) -> bool {
    // println!("nums: {:?}", nums);
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    let mut i_index = 0;
    let mut j_index = 0;
    let mut k_index = 0;

    if nums.len() < 3 {
        return false;
    }

    while i_index < nums.len() - 2 {
        i = nums[i_index];

        j_index = i_index + 1;

        while j_index < nums.len() - 1 {
            if nums[j_index] > i {
                j = nums[j_index];

                k_index = j_index + 1;

                while k_index < nums.len() {
                    if nums[k_index] > j {
                        k = nums[k_index];
                        // println!("i: {}, j: {}, k: {}", i, j, k);

                        if i < j && j < k {
                            return true;
                        } else {
                            i = 0;
                            j = 0;
                            k = 0;
                        }
                    }

                    k_index += 1;
                }
            }

            j_index += 1;
        }

        i_index += 1;
    }

    // println!("i: {}, j: {}, k: {}", i, j, k);
    return false;
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        sol1(nums)
    }
}

#[cfg(test)]
mod increasing_triplet_subsequence_334 {
    use super::*;

    #[test]
    fn check_no_1() {
        let input = [1, 2, 3, 4, 5];
        let correct_output = true;
        let result = Solution::increasing_triplet(input.to_vec());
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_2() {
        let input = [5, 4, 3, 2, 1];
        let correct_output = false;
        let result = Solution::increasing_triplet(input.to_vec());
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_3() {
        let input = [2, 1, 5, 0, 4, 6];
        let correct_output = true;
        let result = Solution::increasing_triplet(input.to_vec());
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_4() {
        let input = [1, 5, 0, 4, 1, 3];
        let correct_output = true;
        let result = Solution::increasing_triplet(input.to_vec());
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_5() {
        let input = [1];
        let correct_output = false;
        let result = Solution::increasing_triplet(input.to_vec());
        assert_eq!(result, correct_output);
    }
}

// find_the_difference_of_two_arrays_2215

#![allow(dead_code)]
#![allow(unused)]

use std::collections::HashSet;
// [1,1,3,3,2] , [1,2,2] = [[3],[]]
// [] , [1,2] = [[],[1,2]]
// [1,2] , [] = [[1,2],[]]
// [] , [] = [[],[]]
// [1,2,3] , [4,5,6] = [[1,2,3],[4,5,6]]
// [1,2,3] , [3,3,4,5] = [[1,2],[4,5]]

// accepted
fn sol1(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::<Vec<i32>>::new();

    let mut map1: HashSet<i32> = HashSet::new();
    let mut map2: HashSet<i32> = HashSet::new();
    let mut map3: HashSet<i32> = HashSet::new();

    for val in nums1.iter() {
        map1.insert(*val);
    }

    for val in nums2.iter() {
        if map1.contains(val) {
            map2.insert(*val);
        }
    }
    
    let mut vec1 = Vec::<i32>::new();
    for val in nums1.iter() {
        if !map2.contains(val) && !map3.contains(val) {
            vec1.push(*val);
            map3.insert(*val);
        }
    }
    
    let mut vec2 = Vec::<i32>::new();
    for val in nums2.iter() {
        if !map2.contains(val) && !map3.contains(val) {
            vec2.push(*val);
            map3.insert(*val);
        }
    }

    res.push(vec1);
    res.push(vec2);

    res
}
// total time = n
// total space = 1

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        sol1(nums1,nums2)
    }
}

#[cfg(test)]
mod find_the_difference_of_two_arrays_2215_tests {
    use super::*;

    #[test]
    fn main() {
        let input =  [1,2,3].to_vec();
        let input2 =  [3,4,5].to_vec();
        let res = [[1,2],[4,5]].to_vec();
        let output = Solution::find_difference(input,input2);
        assert_eq!(output,res)
    }

}

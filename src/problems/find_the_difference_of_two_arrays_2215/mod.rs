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

    let mut map1: HashSet<i32> = HashSet::new(); // time = 1 , space = 1
    let mut map2: HashSet<i32> = HashSet::new(); // time = 1 , space = 1
    let mut map3: HashSet<i32> = HashSet::new(); // time = 1 , space = 1

    for val in nums1.iter() {
        map1.insert(*val);
    } // time = n , space = n

    
    for val in nums2.iter() {
        if map1.contains(val) {
            map2.insert(*val);
        }
    } // time = m , space = m
    
    
    
    let mut vec1 = Vec::<i32>::new();
    for val in nums1.iter() {
        if !map2.contains(val) && !map3.contains(val) {
            vec1.push(*val);
            map3.insert(*val);
        }
    } // time = n , space = 2n
    
    let mut vec2 = Vec::<i32>::new();
    for val in nums2.iter() {
        if !map2.contains(val) && !map3.contains(val) {
            vec2.push(*val);
            map3.insert(*val);
        }
    } // time = m , space = 2m
    
    res.push(vec1); // time = 1 , space = 2n
    res.push(vec2); // time = 1 , space = 2m
    
    res
}
// total time = 2n + 2m
// total space = 5n + 5m



// [1,1,3,3,2] , [1,2,2] = [[3],[]]
// [] , [1,2] = [[],[1,2]]
// [1,2] , [] = [[1,2],[]]
// [] , [] = [[],[]]
// [1,2,3] , [4,5,6] = [[1,2,3],[4,5,6]]
// [1,2,3] , [3,3,4,5] = [[1,2],[4,5]]

// [1,2,3,3] , [1,1,2,2] = [[3],[]] 

// accepted
fn sol2(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::<Vec<i32>>::new();

    let mut map1: HashSet<i32> = HashSet::new();
    let mut map2: HashSet<i32> = HashSet::new();

    let len1 = nums1.len();
    let len2 = nums2.len();

    let mut max_len = len1;

    if len2 > len1 {
        max_len = len2;
    }

    let mut i = 0;
    while i < max_len {

        if i < len1 {
            let ele1 = nums1[i];
            map1.insert(ele1);
        }


        if i < len2 {
            let ele2 = nums2[i];
            map2.insert(ele2);
        }
        

        i += 1;
        
    } // time = n , space = n + m

    let mut vec1 = Vec::<i32>::new();
    let mut vec2 = Vec::<i32>::new();

    for val in &map1 {
        if !map2.contains(val) {
            vec1.push(*val)
        }  
    } // time = n , space = n

    
    for val in &map2 {
        if !map1.contains(val) {
            vec2.push(*val)
        }  
    } // time = m , space = m
    
    
    
    res.push(vec1);
    res.push(vec2);
    
    res
}
// total time = 2n + m => n + m , total space = 2n + 2m => n + m

// accepted
fn sol3(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let map1: HashSet<i32> = nums1.into_iter().collect(); // time = n , space = n
    let map2: HashSet<i32> = nums2.into_iter().collect(); // time = m , space = m
    
    let diff1: Vec<i32> = map1.difference(&map2).cloned().collect(); // time = n , space = n
    let diff2: Vec<i32> = map2.difference(&map1).cloned().collect(); // time = m , space = m
    
    vec![diff1,diff2]
}
// total time = 2n + 2m => n + m , total space = 2n + 2m => n + m

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        sol3(nums1,nums2)
    }
}

#[cfg(test)]
mod find_the_difference_of_two_arrays_2215_tests {
    use super::*;

    #[test]
    fn main() {
        let input =  [1,2,3].to_vec();
        let input2 =  [3,4,5].to_vec();
        let res = [[2,1],[4,5]].to_vec();
        let output = Solution::find_difference(input,input2);
        assert_eq!(output,res)
    }

}

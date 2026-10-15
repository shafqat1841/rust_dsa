// unique_number_of_occurrences_1207

#![allow(dead_code)]
#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;

// [1,2,2,3,3,3] = true
// [1,2,3] = false
// accepted
fn sol1(arr: Vec<i32>) -> bool {

    let mut map = HashMap::<i32, usize>::new();

    for val in arr.iter() {
        if map.contains_key(val) {
            let v = map.get(val);
            match v {
                Some(v) => {
                    let new_val = *v + 1;
                    
                    map.insert(*val,new_val);
                },
                _ => {}
            }
        } else {
            map.insert(*val,0);
        }
    } // time = n , space = n

    let mut map2 = HashSet::<usize>::new();
    
    for val in map.values() {
        if map2.contains(val) {
            return false;
        } 
        map2.insert(*val);
    } // time = n , space = n

   true
}
// total time = 2n => n
// total space = 2n => n

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        sol1(arr)
    }
}

#[cfg(test)]
mod unique_number_of_occurrences_1207_tests {
    use super::*;

    #[test]
    fn main() {
        let input =  [1,2,2,3,3,3].to_vec();
        let res = true;
        let output = Solution::unique_occurrences(input);
        assert_eq!(output,res)
    }

}

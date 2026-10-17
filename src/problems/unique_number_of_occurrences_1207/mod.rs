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

fn sol2(arr: Vec<i32>) -> bool {
    let mut count = [0i16; 2001]; // time = 1 , space = k
    for &val in &arr {
        count[(val + 1000) as usize] += 1;
    } // time = n , space = 1

    let mut frequencies = [false; 1001]; // time = 1 , space = k
    for &freq in &count {
        if freq > 0 {
            let id = freq as usize;
            if frequencies[id] {
                return false;
            }
            frequencies[id] = true;
        }
    } // time = k , space = 1

    true
}

// total time = 1 + n + 1 + k => n + k => n
// total space = k + 1 + k + 1 => 2k => 1

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        sol2(arr)
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

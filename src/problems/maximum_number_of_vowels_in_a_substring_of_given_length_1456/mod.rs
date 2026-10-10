// maximum_number_of_vowels_in_a_substring_of_given_length_1456

#![allow(dead_code)]
#![allow(unused)]

use std::collections::HashSet;

// s = "aaeefftti"
// k = 3
// res = 3

// s = awewiwowu
// k = 4
// r = 2



fn sol1(s: String, k: i32) -> i32 {
    let k = k as usize;
    // let vowals = [b'a',b'e',b'i',b'o',b'u'];
    let mut vowals: HashSet<u8> = std::collections::HashSet::new();
    vowals.insert(b'a');
    vowals.insert(b'e');
    vowals.insert(b'i');
    vowals.insert(b'o');
    vowals.insert(b'u');
    
    let bytes = s.as_bytes();

     // s = "leetcode"
    // k = 3
    
    let mut max_vowals = 0; 
    for val in &bytes[..k] {
        if vowals.contains(val) {
            max_vowals += 1;
        }
    }
    // max_vowals = 2
    
    // s = "leetcode"
    // k = 3

    // tcode
    // lee
    
    // code
    // ee
    // e
    // c

    // ode
    // e
    // e
    // o

    // de
    // tcode
    // t
    // d

    // e
    // code
    // c
    // e

    // max_vowals = 2
    for i in k..bytes.len() {
        
        let mut lv = max_vowals; 
        // lv = 2
        
        let le = bytes[i - k];
        // le = c
        
        let ne = bytes[i];
        // ne = e
        
        if vowals.contains(&le) {
            lv -= 1;
        } 
        // lv = 2
        
        if vowals.contains(&ne) {
            lv += 1;
        } 
        // lv = 3

        if lv > max_vowals {
            max_vowals = lv;
        }
        // max_vowals = 3

    }

    max_vowals
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_vowels(s: String, k: i32) -> i32 {
        sol1(s,k)   
    }
}

#[cfg(test)]
mod maximum_number_of_vowels_in_a_substring_of_given_length_1456_tests {
    use super::*;

    #[test]
    fn main() {
        let s = "abciiidef".to_string();
        let k = 3;
        let res = 3;
        let output = Solution::max_vowels(s, k);
        assert_eq!(output,res)
    }
}

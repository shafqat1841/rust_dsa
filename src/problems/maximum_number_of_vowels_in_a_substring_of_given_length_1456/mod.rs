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


// accepted
fn sol1(s: String, k: i32) -> i32 {
    let k = k as usize;
    let mut vowals: HashSet<u8> = std::collections::HashSet::new();
    vowals.insert(b'a');
    vowals.insert(b'e');
    vowals.insert(b'i');
    vowals.insert(b'o');
    vowals.insert(b'u');

    let s = &s;
    
    let bytes = s.as_bytes();

    let mut max_vowals = 0; 
    for val in &bytes[..k] {
        if vowals.contains(val) {
            max_vowals += 1;
        }
    }

    let mut lv = max_vowals; 
    for i in k..bytes.len() {
        
        let j = i - k;
        let le = bytes[j];
        
        let ne = bytes[i];
        
        if vowals.contains(&le) {
            if lv - 1 >= 0 {
                lv -= 1;
            }
        } 
        
        if vowals.contains(&ne) {
            lv += 1;
        } 
        
        if lv > max_vowals {
            max_vowals = lv;
        }

    }

    max_vowals
}



fn is_vowel(b: u8) -> bool {
    matches!(b,b'a' | b'e' | b'i' | b'o' | b'u')
} // time = 1 , space = 5

// accepted
// total time = n
// total space = 1
fn sol2(s: String, k: i32) -> i32 {
    let k = k as usize;
    let bytes = s.as_bytes();
    
    if bytes.len() < k {
        return 0;
    }

    let mut max_vowels = 0;
    
    // Initialize the first window
    for &val in &bytes[..k] {
        if is_vowel(val) {
            max_vowels += 1;
        }
    }

    let mut lv = max_vowels;
    
    for i in k..bytes.len() {
        let le = bytes[i - k]; 
        let ne = bytes[i];

        if is_vowel(le) {
            lv -= 1;
        }
        if is_vowel(ne) {
            lv += 1;
        }
        
        if lv > max_vowels {
            max_vowels = lv;
        }
    }

    max_vowels
}


struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn max_vowels(s: String, k: i32) -> i32 {
        sol2(s,k)   
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

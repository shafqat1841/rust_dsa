// determine_if_two_strings_are_close_1657

#![allow(dead_code)]
#![allow(unused)]

use std::collections::HashMap;
use std::collections::HashSet;

// failed because we have to also consider the same ferequency of the characters 
// as well like the following case

// a = 2 , b = 2 , z = 2 , c = 1 , total = 7
// a = 1 , b = 2 , z = 3 , c = 1 , total = 7
// w1 = "abbzzca" , w2 = "babzzcz"
// res = false

fn sol1(word1: String, word2: String) -> bool {

    let mut map1 = std::collections::HashMap::<u8,u32>::new();
    let mut map2 = std::collections::HashMap::<u8,u32>::new();

    let mut less_word = word1.as_bytes();
    let mut more_word = word2.as_bytes();

    if word1.len() <= word2.len() { 
        less_word = word1.as_bytes(); 
        more_word = word2.as_bytes(); 
    } else { 
        less_word = word2.as_bytes(); 
        more_word = word1.as_bytes(); 
    }

    for i in 0..less_word.len() {
        let ele = less_word[i];
        
        if map1.contains_key(&ele) {
            let mut val = map1.get(&ele).unwrap();
            let new_val = val + 1;
            map1.insert(ele,new_val);
        } else {
            map1.insert(ele,1);
        }
    }
    
    
    for i in 0..more_word.len() {
        let ele = more_word[i];

        if map2.contains_key(&ele) {
            let mut val = map2.get(&ele).unwrap();
            let new_val = val + 1;
            map2.insert(ele,new_val);
        } else {
            map2.insert(ele,1);
        }
    }

    
    let mut less = 0;
    let mut more = 0;
    
    
    for keys in map1.keys() {

        let less_val = map1.get(keys).unwrap();
        less += less_val;
        
        if map2.contains_key(keys) {
            let more_val = map2.get(keys).unwrap();
            more += more_val;
        } else {
            return false;
        }

    }

    if less != more {
        return false;
    }

   true
}
// total time = 
// total space = 




// Operation 1: Swap any two existing characters.
// For example, abcde -> aecdb
//               |  |
// Operation 2: Transform every occurrence of one existing character into another existing character, and do the same with the other character.
// For example, aacabb -> bbcbaa (all a's turn into b's, and all b's turn into a's)
//              || |||

// w1 = "abc" , w2 = "bca" 
// bca => acb => abc
// res = true

// w1 = "abccb" , w2 = "bccaa"
// abccb => bacca => bcaca => bccaa
// res = true

// w1 = "cabbba" , w2 = "abbccc"
// cabbba => acbbbc => abcccb => abccbc => abcbcc => abbccc
// res = true

// w1 = "aaabb" , w2 = "bccaa"
// res = false

// w1 = "axccb" , w2 = "bccaa"
// res = false

// a = 2 , b = 2 , z = 2 , c = 1 , total = 7
// a = 1 , b = 2 , z = 3 , c = 1 , total = 7
// w1 = "abbzzca" , w2 = "babzzcz"
// res = false

// a = 1 , b = 3 , c = 1 , z = 1 , f = 1 , total = 7
// a = 1 , b = 2 , c = 1 , z = 3 , f = 0 , total = 7
// w1 = "abbbzcf" , w2 = "babzzcz"
// res = false

//
fn sol2(word1: String, word2: String) -> bool {

    if word1.len() != word2.len() {
        return false;
    }

    let mut map1 = std::collections::HashMap::<u8,u32>::new();
    let mut map2 = std::collections::HashMap::<u8,u32>::new();

    let mut less_word = word1.as_bytes();
    let mut more_word = word2.as_bytes();

    if word1.len() <= word2.len() { 
        less_word = word1.as_bytes(); 
        more_word = word2.as_bytes(); 
    } else { 
        less_word = word2.as_bytes(); 
        more_word = word1.as_bytes(); 
    }

    for i in 0..less_word.len() {
        let ele = less_word[i];
        
        if map1.contains_key(&ele) {
            let mut val = map1.get(&ele).unwrap();
            let new_val = val + 1;
            map1.insert(ele,new_val);
        } else {
            map1.insert(ele,1);
        }
    }
    
    
    for i in 0..more_word.len() {
        let ele = more_word[i];

        if map2.contains_key(&ele) {
            let mut val = map2.get(&ele).unwrap();
            let new_val = val + 1;
            map2.insert(ele,new_val);
        } else {
            map2.insert(ele,1);
        }
    }

    if map1.len() != map2.len() {
        return false;
    }
    
    for key in map1.keys() {
        if !map2.contains_key(key) {
            return false;
        }
    }

    let mut map3 = std::collections::HashMap::<u32,u32>::new();
    
    for val in map1.values() {
        if map3.contains_key(val) {
            let v = map3.get(val).unwrap();
            let new_v = v + 1;
            map3.insert(*val, new_v);
        } else {
            map3.insert(*val, 1);
        }
    }
    
    for val in map2.values() {
        if map3.contains_key(val) {

            let v = map3.get(val).unwrap();

            let new_v = v - 1;
            if new_v == 0 {
                map3.remove(val);
            } else {
                map3.insert(*val, new_v);
            }

        } else {
            return false;
        }
    }
    
    if map3.len() > 0 {
        return false;
    }

   true
}
// total time = 
// total space = 

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        sol2(word1, word2)
    }
}

#[cfg(test)]
mod determine_if_two_strings_are_close_1657_tests {
    use super::*;

    #[test]
    fn main() {
        let input =  "abc".to_string();
        let input2 =  "cba".to_string();
        let res = true;
        let output = Solution::close_strings(input,input2);
        assert_eq!(output,res)
    }

    #[test]
    fn main2() {
        let input =  "cabbba".to_string();
        let input2 =  "abbccc".to_string();
        let res = true;
        let output = Solution::close_strings(input,input2);
        assert_eq!(output,res)
    }

    #[test]
    fn main3() {
        let input =  "abbzzca".to_string();
        let input2 =  "babzzcz".to_string();
        let res = false;
        let output = Solution::close_strings(input,input2);
        assert_eq!(output,res)
    }
}

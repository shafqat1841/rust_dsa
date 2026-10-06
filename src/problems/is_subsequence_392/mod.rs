// is_subsequence_392

#![allow(dead_code)]
#![allow(unused)]

use core::prelude::v1;
use std::collections::HashMap;

// accepted but did not considered the FOLLOW UP question from the problem
fn sol1(s: String, t: String) -> bool {
    if s.len() == 0 {
        return true;
    }

    if t.len() == 0 {
        return false;
    }

    if s.len() > t.len() {
        return false;
    }

    let s = s.as_bytes(); // time = 1, space = 1

    let t = t.as_bytes(); // time = 1, space = 1 

    let mut sc = 0;

    let mut i_f = usize::MAX;

    for i in (0..s.len()).rev() {
        let i_ele = s[i];
        for j in (0..t.len()).rev() {
            let j_ele = t[j];

            if i_ele == j_ele {
                if j < i_f {
                    i_f = j;
                    sc += 1;
                    if sc == s.len() {
                        return true;
                    }
                    break;
                }
            }
        }
    } // time = n + m , space = 1 

    // total time = n + m => n
    // total  space = 1

    false
}

// optimized and accepted but did not considered the FOLLOW UP question from the problem
fn sol2(s: String, t: String) -> bool {
    if s.len() == 0 {
        return true;
    }

    if t.len() == 0 {
        return false;
    }

    if s.len() > t.len() {
        return false;
    }

    let s = s.as_bytes(); // time = 1, space = 1

    let t = t.as_bytes(); // time = 1, space = 1 

    let mut sc = s.len();

    let mut i_f = usize::MAX;

    for j in (0..t.len()).rev() {
        let i_ele = s[sc - 1];
        let j_ele = t[j];

        if i_ele == j_ele {
            if j < i_f {
                i_f = j;
                sc -= 1;
                if sc == 0 {
                    return true;
                }
            }
        }
    } // time = n , space = 1 

    // total time = n
    // total  space = 1

    false
}

// optimized and accepted and also considered the FOLLOW UP question from the problem
fn sol3(s: String, t: String) -> bool {
    let mut map: HashMap<u8, Vec<usize>> = HashMap::new();

    for (i, &b) in t.as_bytes().iter().enumerate() {
        map.entry(b).or_default().push(i);
    } // time = n , space = n

    let mut current_pos = 0;

    for b in s.as_bytes() {
        if let Some(indices) = map.get(b) {
            match indices.binary_search(&current_pos) {
                Ok(i) => {
                    current_pos = indices[i] + 1;
                }
                Err(i) => {
                    if i < indices.len() {
                    } else {
                    }
                    if i < indices.len() {
                        current_pos = indices[i] + 1;
                    } else {
                        return false;
                    }
                }
            } // time = log n , space = 1
        } else {
            return false;
        }
    } // time = m , space = 1

    // total time = n * m log n
    // total space = n

    true
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        sol3(s, t)
    }
}

#[cfg(test)]
mod is_subsequence_392_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let mut s = "abc".to_string();
        let mut t = "ahbgdc".to_string();
        let correct_output = true;
        let result = Solution::is_subsequence(s, t);
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_2() {
        let mut s = "axc".to_string();
        let mut t = "ahbgdc".to_string();
        let correct_output = false;
        let result = Solution::is_subsequence(s, t);
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_3() {
        let mut s = "acb".to_string();
        let mut t = "ahbgdc".to_string();
        let correct_output = false;
        let result = Solution::is_subsequence(s, t);
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_4() {
        let mut s = "aaaaaa".to_string();
        let mut t = "bbaaaa".to_string();
        let correct_output = false;
        let result = Solution::is_subsequence(s, t);
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_5() {
        let mut s = "abc".to_string();
        let mut t = "".to_string();
        let correct_output = false;
        let result = Solution::is_subsequence(s, t);
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_6() {
        let mut s = "abc".to_string();
        let mut t = "ahbgdc".to_string();
        let correct_output = true;
        let result = Solution::is_subsequence(s, t);
        assert_eq!(result, correct_output);
    }
}

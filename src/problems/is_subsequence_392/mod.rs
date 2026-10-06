// is_subsequence_392

#![allow(dead_code)]
#![allow(unused)]

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

    let s = s.into_bytes();

    let t = t.into_bytes();

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
    }

    false
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn is_subsequence(s: String, t: String) -> bool {
        sol1(s, t)
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
}

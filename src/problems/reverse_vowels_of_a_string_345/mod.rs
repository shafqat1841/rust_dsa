// a e i o u
fn sol1(mut s: String) -> String {
    let vowels = "aeiouAEIOU".as_bytes();
    let len = s.len();
    let mut i1 = 0;
    let mut i2 = len - 1;

    let mut s = s.into_bytes();

    loop {
        for i in i1..i2 {
            let char = s.get(i);
            match char {
                Some(c) => {
                    if vowels.contains(c) {
                        i1 = i + 1;
                        break;
                    }
                }
                None => {
                    panic!("Error: Could not get character at index {}", i);
                }
            }
            i1 += 1;
        }

        for i in (i1..i2).rev() {
            let char = s.get(i);
            match char {
                Some(c) => {
                    if vowels.contains(c) {
                        i2 = i - 1;
                        break;
                    }
                }
                None => {
                    panic!("Error: Could not get character at index {}", i);
                }
            }
            i2 -= 1;
        }

        if i1 >= i2 {
            break;
        }

        s.swap(i1, i2);

    }
    String::from_utf8(s).unwrap()
}

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_vowels(s: String) -> String {
        sol1(s)
    }
}

#[cfg(test)]
mod reverse_vowels_345_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let s = "IceCreAm".to_string();
        let correct_result = "AceCreIm".to_string();
        let result = Solution::reverse_vowels(s);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_2() {
        let s = "leetcode".to_string();
        let correct_result = "leotcede".to_string();
        let result = Solution::reverse_vowels(s);
        assert_eq!(result, correct_result);
    }
}

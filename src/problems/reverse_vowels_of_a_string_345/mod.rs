// a e i o u
fn sol1(s: String) -> String {
    println!("s: {}", s);

    let vowels = "aeiouAEIOU".as_bytes();
    let len = s.len();
    let mut i1 = 0;
    let mut i2 = len - 1;

    let mut s = s.into_bytes();

    while i1 < i2 {
        loop {
            let char = s[i1];
            let res = vowels.contains(&char);
            println!("i1: {}, res: {}, char: {}", i1, res, char);
            if res || i1 >= i2 {
                break;
            }
            i1 += 1;
        }

        loop {
            let char = s[i2];
            let res = vowels.contains(&char);
            println!("i2: {}, res: {}, char: {}", i2, res, char);
            if res || i2 <= i1 {
                break;
            }
            i2 -= 1;
        }
        
        s.swap(i1, i2);
        println!("s: {:?}", s);

        i1 += 1;
        i2 -= 1;
    }

    let result = match String::from_utf8(s) {
        Ok(s) => s,
        Err(e) => {
            panic!("Error: Could not convert bytes to string");
        }
    };

    println!("result: {}", result);

    result
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

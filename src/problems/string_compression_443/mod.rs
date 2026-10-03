#![allow(dead_code)]
#![allow(unused)]

// string_compression_443

// accepted
fn sol1(chars: &mut Vec<char>) -> i32 {
    let mut i1 = 0;
    let mut i2 = 0;
    // let mut count = 0;

    while i1 < chars.len() {
        let c_c = chars[i1];
        let mut c_count = 1;

        i1 += 1;

        while i1 < chars.len() && chars[i1] == c_c {
            i1 += 1;
            c_count += 1;
        }

        chars[i2] = c_c;
        i2 += 1;

        if c_count > 1 {
            let count_str = c_count.to_string();
            for c in count_str.chars() {
                chars[i2] = c;
                i2 += 1;
            }
        }
    }

    i2 as i32
}

struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        // sol1(nums)
        sol1(chars)
    }
}

#[cfg(test)]
mod string_compression_443_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let mut input = ['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let correct_output = 6;
        let result = Solution::compress(&mut input.to_vec());
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_2() {
        let mut input = ['a'];
        let correct_output = 1;
        let result = Solution::compress(&mut input.to_vec());
        assert_eq!(result, correct_output);
    }

    #[test]
    fn check_no_3() {
        let mut input = [
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        let correct_output = 4;
        let result = Solution::compress(&mut input.to_vec());
        assert_eq!(result, correct_output);
    }
}

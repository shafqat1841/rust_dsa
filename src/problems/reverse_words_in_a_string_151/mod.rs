fn sol1(s: &str) -> String {
    let mut res = Vec::<String>::new();

    let s = s.as_bytes();

    let len = s.len();

    let mut i = 0;

    while i < len {
        if s[i] != 32 {
            let start = i;
            let mut end = start;
            while end < len && s[end] != 32 {
                end += 1;
            }

            let u8_word = s[start..end].to_vec();
            let word = String::from_utf8(u8_word).unwrap();

            res.insert(0, word);

            i = end;
        }

        i += 1;
    }

    res.join(" ")
}

// optimized code
fn sol2(s: &str) -> String {
    let mut words = Vec::<&str>::new(); // time = 1 , space = n 

    let bytes = s.as_bytes();

    let len = s.len();

    let mut i = 0;

    while i < len {
        if bytes[i] == 32 {
            i += 1;
            continue;
        }

        if i >= len {
            break;
        }

        let start = i;
        while i < len && bytes[i] != 32 {
            i += 1;
        }

        let word = &s[start..i];
        words.push(word);
    } // time = n , space = 1 

    words.reverse(); // time = n , space = 1

    words.join(" ")  // time = n , space = n

    // total time = 3n = n
    // total space = 2n = n
}

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_words(s: String) -> String {
        // sol1(&s)
        sol2(&s)
    }
}

#[cfg(test)]
mod reverse_words_in_a_string_151_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let s = "the sky is blue".to_string();
        let correct_result = "blue is sky the".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_2() {
        let s = "  hello world  ".to_string();
        let correct_result = "world hello".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_3() {
        let s = "a good   example".to_string();
        let correct_result = "example good a".to_string();
        let result = Solution::reverse_words(s);
        assert_eq!(result, correct_result);
    }
}

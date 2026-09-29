fn sol1(mut s: &str) -> String {
    // println!("s: {}", s);
    // println!("s len: {}", s.len());

    if s.len() == 0 {
        return s.to_string();
    }

    if s.chars().nth(0).unwrap() == ' ' {
        let mut empty = 1;

        for i in 1..s.len() {
            if s.chars().nth(i).unwrap() == ' ' {
                empty += 1;
            } else {
                break;
            }
        }
        // println!("empty 1: {}", empty);

        s = &s[empty..];
    }

    // let mut len = s.len();

    if s.chars().nth(s.len() - 1).unwrap() == ' ' {
        let mut empty = s.len();

        for i in (s.len() - 1)..0 {
            if s.chars().nth(i).unwrap() == ' ' {
                empty -= 1;
            } else {
                break;
            }
        }
        // println!("empty 2: {}", empty);

        s = &s[..empty];
    }

    // println!("s: {}", s);
    // println!("s len: {}", s.len());

    s.to_string()
}

fn sol2(s: &str) -> String {
    // println!("s: {}", s);
    let mut res = Vec::<String>::new();

    let s = s.as_bytes();

    let mut i = s.len();

    while i > 0 {
        i -= 1;

        if s[i] != 32 {
            let start = i;
            let mut end = start;

            while end > 0 && s[end] != 32 {
                end -= 1;
            }

            let end_value = if end == 0 { 0 } else { end + 1};
            
            let data = &s[end_value..=start];

            // println!("data: {:?}",data);
            
            let aaa = String::from_utf8(data.to_vec()).unwrap();
            // println!("aaa: {:?}",aaa);
            res.push(aaa);
            if end != 0 {
                res.push(" ".to_string());
            }

            i = end;
        }
    }

    res.join("")

    // match String::from_utf8(res) {
    //     Err(e) => {
    //         panic!("Error: {}", e);
    //     },
    //     Ok(val) => val
    // }

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

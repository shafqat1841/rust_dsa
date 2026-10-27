// decode_string_394

#![allow(dead_code)]
#![allow(unused)]


// "2[e]2[a2[b2[c]]]2[d]gh"
// "eeabccbccabccbccgh"

// "2[a2[b2[c]]]"
// "abccbccabccbcc"


fn is_match_number(ele: u8) -> bool {
    let same = matches!(ele, b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9');
    same
}

// 2[c]
// 2[c2[b]]
fn decode(bytes: &[u8], mut i: usize) -> (Vec<u8>,usize) {
    let mut new_cha = Vec::<u8>::new();
    let mut sub_bytes = Vec::<u8>::new();

    let mut number = i;
    let mut start = i + 2;

    while bytes[start] != b']' {
        if is_match_number(bytes[start]) {
            let (new_sub_part,new_index) = decode(bytes,start);
            for val in new_sub_part {
                new_cha.push(val);
            }
            start = new_index;
        } else {
            new_cha.push(bytes[start]);
            start += 1;
        }
    }

    while number > 0 {
          for &val in &new_cha { 
              sub_bytes.push(val);    
          }

          number -= 1;
    }

    (sub_bytes,i)
}

// 
fn sol1(s: String) -> String {
    let s = s.as_bytes();

     let mut new_bytes = Vec::<u8>::new();

     let mut i = 0;

     while i < s.len() {
         let ele = s[i];

         if !is_match_number(ele) {
             new_bytes.push(ele);
             i += 1;
             continue;
         } 

         let (sub_bytes,new_index) = decode(s,i);

         for val in sub_bytes {
            new_bytes.push(val);
         }

         i = new_index;

     }

     String::from_utf8(new_bytes).unwrap()


}
// total time = 
// total space = 

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn decode_string(s: String) -> String {
        sol1(s)
    }
}

#[cfg(test)]
mod decode_string_394_tests {
    use super::*;


    #[test]
    fn main() {
        let input = "2[e]2[a2[b2[c]]]2[d]gh".to_string();
        let res = "eeabccbccabccbccgh".to_string();
        let output = Solution::decode_string(input);
        assert_eq!(output,res)
    }

}

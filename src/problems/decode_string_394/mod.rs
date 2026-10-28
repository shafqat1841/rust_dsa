// decode_string_394

#![allow(dead_code)]
#![allow(unused)]

mod solution3;


fn is_match_number(ele: u8) -> bool {
    let same = matches!(ele, b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9');
    same
} 
// total time = 1 
// total space = 10 => 1

fn make_new_cha(bytes: &[u8], mut i: usize) -> (Vec<u8>,usize)  {
    let mut new_cha = Vec::<u8>::new();

     let mut index = i + 1;

    while index < bytes.len() && bytes[index] != b']' {
        if !is_match_number(bytes[index]) {
            new_cha.push(bytes[index]);
            index += 1;
            continue;
        }

        let (new_sub_part,new_index) = decode(bytes, index);
        for val in new_sub_part {
            new_cha.push(val);
        }

        index = new_index;
    }

    (new_cha, index + 1)


}

fn decode(bytes: &[u8], mut i: usize) -> (Vec<u8>,usize) {
    let mut sub_bytes = Vec::<u8>::new();

    let mut num_i = i;
    let mut numbers_bytes = Vec::<u8>::new();
    
    while num_i < bytes.len() && is_match_number(bytes[num_i]) {
        let ele = bytes[num_i];
        numbers_bytes.push(ele);
        num_i += 1;
    }

    
    let number_string = String::from_utf8(numbers_bytes).unwrap();
    let mut num: u32 = number_string.parse().unwrap();

    let  (new_cha, new_index) = make_new_cha(bytes, num_i);
    
    while num > 0 {
          for &val in &new_cha { 
              sub_bytes.push(val);    
          }

          num -= 1;
    }

    (sub_bytes, new_index)
}

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

fn sol2(s: String) -> String  {
    let mut count_stack: Vec<usize> = Vec::new();
    let mut string_stack: Vec<String> = Vec::new();
    let mut current_string = String::new();
    let mut k: usize = 0;

    for c in s.chars() {
        if c.is_digit(10) {
            // Build multi-digit numbers (e.g., '3', '0', '0' -> 300)
            k = k * 10 + c.to_digit(10).unwrap() as usize;
        } else if c == '[' {
            // Save current state onto the stacks and reset for the inner block
            count_stack.push(k);
            string_stack.push(current_string);
            current_string = String::new();
            k = 0;
        } else if c == ']' {
            // Pop the previous context and repeat the current string
            let repeat_count = count_stack.pop().unwrap();
            let mut prev_string = string_stack.pop().unwrap();
            
            prev_string.push_str(&current_string.repeat(repeat_count));
            current_string = prev_string;
        } else {
            // Accumulate regular characters
            current_string.push(c);
        }
    }

    current_string
}
// total time = 
// total space = 


struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn decode_string(s: String) -> String {
        // sol2(s)
        solution3::sol3(s)
    }
}

#[cfg(test)]
mod decode_string_394_tests {
    use super::*;


    #[test]
    fn main() {
        // let input = "2[e]2[a2[b2[c]]]2[d]gh".to_string();
        let input = "100[leetcode]".to_string();
        let mut array = Vec::<String>::new();
        for _ in 0..100 {
            array.push("leetcode".to_string());
        }
        let array_joined = array.join("");
        let res = array_joined;
        let output = Solution::decode_string(input);
        assert_eq!(output,res)
    }

}

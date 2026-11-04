

fn is_match_number(ele: u8) -> bool {
    let same = matches!(ele, b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9');
    same
} 
// time = 1 , space = 1


fn decode(bytes: &[u8], mut i: usize) -> (Vec<u8>,usize) {

    let mut num_i = i;
    let mut numbers_bytes = Vec::<u8>::new();
    
    while num_i < bytes.len() && is_match_number(bytes[num_i]) {
        let ele = bytes[num_i];
        numbers_bytes.push(ele);
        num_i += 1;
    }
    // time = d , space = d
    // where d is the number of digits

    
    let number_string = String::from_utf8(numbers_bytes).unwrap();
    let mut num: u32 = number_string.parse().unwrap();

    
    let mut index = num_i + 1;
    let mut new_cha = Vec::<u8>::new();

    while index < bytes.len() && bytes[index] != b']' {

        if !is_match_number(bytes[index]) {
            new_cha.push(bytes[index]);
            index += 1;
            continue;
        } 
        // time = m , space = m
        // where m represent the non digit numbers 
        
        let (new_sub_part,new_index) = decode(bytes, index);
        // time = t , space = t
        // where t represent the recussive call

        for val in new_sub_part {
            new_cha.push(val);
        }
        // time = c , space = c
        // where c represent the non digit numbers from the recussive call

        index = new_index;
    }
    // b = m ( of decode ) + c ( of decode )  
    // time = m + b , space = b
    // We are deliberately separating the work performed by the current function from the work performed by its recursive children. 
    // Later, we will combine them to calculate the total complexity.

    let new_index = index + 1;

    let mut sub_bytes = Vec::<u8>::new();

    
    while num > 0 {
          for &val in &new_cha { 
              sub_bytes.push(val);    
          }

          num -= 1;
    }
    // r = initial value of num
    // time = r * b , space = b + (r * b)
    // since r >= 1 then 
    // b <= rb
    // so b + rb <= rb
    // so
    // time = rb , space = rb

    (sub_bytes, new_index)
}

// Variable	   Meaning
// n    	   Original encoded input length
// l    	   Final decoded output length

// d    	   Number of digits in the repetition number
// m    	   Number of iterations of the current while loop
// b    	   Length of new_cha
// r    	   Repetition count (num)
// p           max recussive depth


// TIME COMPLEXITY
// T (group) = d(group) + m(group) + B(group) + rB(group)

// since r >= 1 then 
// T (group) = d(group) + m(group) + rB(group)

// l (group) (group) = rb(group)
// T (group) = d(group) + m(group) + l(group)
// this is the time for one group

// d and m can not be greater then all encoded input characters
// so d(group) = n and m = n(group)
// so T (group) = d(group) + m(group) + l(group) => n (final) + n (final) + l(group)

// we know that the number of groups cannot exceed n 
// and The decoded length of any individual group cannot exceed l (final), the final decoded output length.
// so l (group) <= group * l (final)
// since g <= n
// l (group) = n * l (final)

// let combine all
// T (group) = n (final) + n (final) + l(group) => T (final) = n (final) + n (final) + nl (final)
// T = n + n + nl
// T = 2n + nl
// or simply
// T = nl
// where n is the number of input character and l is the final output length

// SPACE COMPLEXITY
// we know
// p <= n and d <= n and m <= l (final)
// S = n + l (final)

// total time = nl
// total space = n + l



pub fn sol1(s: String) -> String {
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
        // time = n , space  = n
            
        let (sub_bytes,new_index) = decode(s,i);
        // t (recusive) = nl (group) , space = n (group) + l (group) => l (final)
        
        for val in sub_bytes {
            new_bytes.push(val);
        }
        // time = l (final), space  = l (final)

         i = new_index;

     }
     // time = nl, space = n + l

     String::from_utf8(new_bytes).unwrap()

}
// total time = nl
// total space = n + l
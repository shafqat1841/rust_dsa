// number_of_recent_calls_933_v2

#![allow(dead_code)]
#![allow(unused)]

use std::collections::VecDeque;

pub struct RecentCounter {
    buffer: [i32; 10000],
    head: usize,
    tail: usize,
    len: usize,
}

#[allow(dead_code)]
impl RecentCounter {
    fn new() -> Self {
        Self {
            buffer: [0; 10000],
            head: 0,
            tail: 0,
            len: 0,
        }
    }
    // total time = 1
    // total space = 1
    
    fn ping(&mut self, t: i32) -> i32 {

        self.buffer[self.tail] = t;
        self.tail = (self.tail + 1) % 10000;
        self.len += 1;

        let lower_limit = t - 3000;

        while self.len > 0 {
            let oldest = self.buffer[self.head];
            if oldest < lower_limit {
                self.head = (self.head + 1) % 10000;
                self.len -= 1;
            } else {
                break;
            }
        }

        self.len as i32
    }
    // total time = n
    // total space = n
}

#[cfg(test)]
mod number_of_recent_calls_933_v2_tests {
    use super::*;


    #[test]
    fn main() {
        let mut dq = RecentCounter::new();
        
        let res_1 = dq.ping(1);
        let output_1 = 1;
        assert_eq!(output_1, res_1);
        
        let res_2 = dq.ping(100);
        let output_2 = 2;
        assert_eq!(output_2, res_2);

        let res_3 = dq.ping(3001);
        let output_3 = 3;
        assert_eq!(output_3, res_3);

        let res_4 = dq.ping(3002);
        let output_4 = 3;
        assert_eq!(output_4, res_4);

    }

}

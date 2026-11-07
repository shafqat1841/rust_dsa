// number_of_recent_calls_933_v3

#![allow(dead_code)]
#![allow(unused)]

use std::collections::VecDeque;

pub struct RecentCounter {
    buffer: [i32; 10000],
    start: usize,
    end: usize,
}

#[allow(dead_code)]
impl RecentCounter {
    fn new() -> Self {
        Self {
            buffer: [0; 10000],
            start: 0,
            end: 0,
        }
    }
    // total time = 1
    // total space = 1
    
    fn ping(&mut self, t: i32) -> i32 {

        self.buffer[self.end] = t;
        self.end += 1;

        let lower_limit = t - 3000;

        while self.start < self.end
            && self.buffer[self.start] < lower_limit
        {
            self.start += 1;
        }

        (self.end - self.start) as i32
    }
    // total time = n
    // total space = n
}

#[cfg(test)]
mod number_of_recent_calls_933_v3_tests {
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

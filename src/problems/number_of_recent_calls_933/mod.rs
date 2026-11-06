// number_of_recent_calls_933

#![allow(dead_code)]
#![allow(unused)]

use std::collections::VecDeque;

struct RecentCounter {
    deque: VecDeque<i32>,
}

#[allow(dead_code)]
impl RecentCounter {
    fn new() -> Self {
        let deque = VecDeque::with_capacity(10000);
        RecentCounter {
            deque
        }
    }
    
    fn ping(&mut self, t: i32) -> i32 {

        self.deque.push_front(t);
        
        let lower_limit = t - 3000;

        loop {

            let last = self.deque.back().unwrap();

            if *last >= lower_limit {
                break;
            } 

            self.deque.pop_back();

        }

        self.deque.len() as i32
    }
}

#[cfg(test)]
mod number_of_recent_calls_933_tests {
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

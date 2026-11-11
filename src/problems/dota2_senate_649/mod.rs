// dota2_senate_649

#![allow(dead_code)]
#![allow(unused)]

use std::collections::VecDeque;
    
    // RDDDRDRR , R = 4 , D = 4 , R_r = 0 , D_r = 0
    // DDDRDRRR , R = 4 , D = 4 , R_r = 1 , D_r = 0
    // DDRDRRR , R = 4 , D = 3 , R_r = 0 , D_r = 0
    // DRDRRRD , R = 4 , D = 3 , R_r = 0 , D_r = 1
    // RDRRRDD , R = 4 , D = 3 , R_r = 0 , D_r = 2
    // DRRRDD , R = 3 , D = 3 , R_r = 0 , D_r = 1
    // RRRDDD , R = 3 , D = 3 , R_r = 0 , D_r = 2
    // RRDDD , R = 2 , D = 3 , R_r = 0 , D_r = 1
    // RDDD , R = 1 , D = 3 , R_r = 0 , D_r = 0
    // DDDR , R = 1 , D = 3 , R_r = 1 , D_r = 0
    // DDR , R = 1 , D = 2 , R_r = 0 , D_r = 0
    // DRD , R = 1 , D = 2 , R_r = 0 , D_r = 1
    // RDD , R = 1 , D = 2 , R_r = 0 , D_r = 2
    // DD , R = 0 , D = 2 , R_r = 0 , D_r = 1

// accepted 
fn sol1(senate: String) -> String {
    let senate = senate.as_bytes();
        
    let mut r_num = 0;
    let mut d_num = 0;

    let mut r_remove = 0;
    let mut d_remove = 0;

    let mut remainning = VecDeque::<u8>::new();

    for i in 0..senate.len() {
        let ele = senate[i];
        if ele == b'R' {
            r_num += 1;
        } else {
            d_num += 1;
        }
        remainning.push_back(ele);
    } // time = n , space = n


    while r_num > 0 && d_num > 0 {

        let ele = remainning.pop_front().unwrap();

        if ele == b'R' {
            if d_remove > 0 {
                r_num -= 1;
                d_remove -= 1;
            } else {
                r_remove += 1;
                remainning.push_back(ele);
            }
        } else {
            if r_remove > 0 {
                d_num -= 1;
                r_remove -= 1;
            } else {
                d_remove += 1;
                remainning.push_back(ele);
            }
        }

    } // time = r + d => n , space = n
    // where r is the numbers of Radiant voters 
    // and where d is the numbers of Dire voters

    if r_num > d_num {
        "Radiant".to_string()        
    } else {
        "Dire".to_string()        
    } // time = 1 , space = 1
    
}
// total time = n + n => 2n => n
// total space = n + n => 2n => n

// accepted
fn sol2(senate: String) -> String {
    let n = senate.len();

    let mut radiant = VecDeque::<usize>::new();
    let mut dire = VecDeque::<usize>::new();

    for (i, &ele) in senate.as_bytes().iter().enumerate() {
        if ele == b'R' {
            radiant.push_back(i);
        } else {
            dire.push_back(i);
        }
    } // time = n , space = n

    while !radiant.is_empty() && !dire.is_empty() {
        let r = radiant.pop_front().unwrap();
        let d = dire.pop_front().unwrap();

        if r < d {
            radiant.push_back(r + n);
        } else {
            dire.push_back(d + n);
        }
    } // time = n , space = n

    if radiant.is_empty() {
        "Dire".to_string()
    } else {
        "Radiant".to_string()
    }
}
// total time = n + n => 2n => n
// total space = n + n => 2n => n



struct Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        // sol1(senate)
        sol2(senate)
    }
}
#[cfg(test)]
mod dota2_senate_649_tests {
    use super::*;


    #[test]
    fn main() {
        let input = "RDDDRDR".to_string();
        let res = Solution::predict_party_victory(input);
        let output = "Dire".to_string();
        assert_eq!(output, res);

    }

}

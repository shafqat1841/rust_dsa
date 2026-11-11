// dota2_senate_649

#![allow(dead_code)]
#![allow(unused)]

use std::collections::VecDeque;

    // RDDDRDRR , R = 4 , D = 4 , R_r = 0 , D_r = 0
    // R DDDRDRR , R = 4 , D = 4 , R_r = 1 , D_r = 0
    // R DDRDRR , R = 4 , D = 3 , R_r = 0 , D_r = 0
    // RD DRDRR , R = 4 , D = 3 , R_r = 0 , D_r = 1
    // RDD RDRR , R = 4 , D = 3 , R_r = 0 , D_r = 2
    // RDD DRR , R = 3 , D = 3 , R_r = 0 , D_r = 1
    // RDDD RR , R = 3 , D = 3 , R_r = 0 , D_r = 2
    // RDDD R , R = 2 , D = 3 , R_r = 0 , D_r = 1
    // RDDD , R = 1 , D = 3 , R_r = 0 , D_r = 0
    // R DDD , R = 1 , D = 3 , R_r = 1 , D_r = 0
    // R DD , R = 1 , D = 2 , R_r = 0 , D_r = 0
    // RD D , R = 1 , D = 2 , R_r = 0 , D_r = 1
    // RDD , R = 1 , D = 2 , R_r = 0 , D_r = 2
    // DD , R = 0 , D = 2 , R_r = 0 , D_r = 1

    
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

        match remainning.pop_front() {
            Some(ele) => {
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
            },
            None => {
                panic!("No item remains for processing")
            }
        }

    } // time = r + d , space = 1

    if r_num > d_num {
        "Radiant".to_string()        
    } else {
        "Dire".to_string()        
    } // time = 1 , space = 1
    
}

// total time = n + r + d
// total space = n

struct Solution;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        sol1(senate)
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

 // RDDDRDR
        // R|DDDRDR = RD|DRDR
        // RD|DRDR  = DD|RDR
        // DD|RDR   = DDD|R
        // DDD|R    = DDD    
        

        // RDDDRRD
        // RDDRRD
        // DDRRD
        // DDRD
        // DRD
        // DD
        
        // RDDDRD
        // RDDRD
        // DDRD
        // DDD

         // RRDDDRDDD
         // R|RDDDRDDD = RR|DDRDDD
         // RR|DDRDDD  = RRD|RDDD
         // RRD|RDDD   = RDR|DDD
         // RDR|DDD    = RD|DDR
         // RD|DDR     = DD|DR
        //  DD|DR      = DDD|
        //  DDD|      = DDD


        // RDDDRDR
        // R,DDDRDR = RD,DRDR
        // RD,DRDR  = DD,RDR
        // DD,RDR   = DDD,R
        // DDD,R    = DDD

        // DDRRR
        // D,DRRR => DRRRD => RRRDD => RRDD
        // RRDD => RDDR => DDRR => DRR
        // DRR => RRD => RD
        // RD => R

        // RDDDRDRR
        // RDDRDRR
        // DDRDRR
        // DDDRR
        // DDDR
        // DDR
        // DD

        // RDDDRDRR , R = 4 , D = 4
        // RDDRDRR , R = 4 , D = 3
        // DDRDRR , R = 3 , D = 3
        // DDDRR , R = 2 , D = 3
        // DDDR , R = 1 , D = 3
        // DDR , R = 1 , D = 2
        // DD , R = 0 , D = 2

        // RDDDRDRR , R = 0 , D = 0
        // R DDDRDRR , R = 1 , D = 0
        // R DDRDRR , R = 0 , D = 0
        // D DRDRR , R = 0 , D = 0
        // DD RDRR , R = 0 , D = 1
        // DD DRR , R = 0 , D = 0
        // DDD RR , R = 0 , D = 1
        // DDD R , R = 0 , D = 0
        // DDDR , R = 1 , D = 0
        //  DDR , R = 0 , D = 0
        // D DR , R = 0 , D = 1
        // DD R , R = 0 , D = 2
        // DD , R = 0 , D = 1

         // RDDDRDRR , R = 4 , D = 4 , R_r = 0 , D_r = 0
         // R DDDRDRR , R = 4 , D = 4 , R_r = 1 , D_r = 0
         // R DDRDRR , R = 4 , D = 3 , R_r = 0 , D_r = 0
         // RD DRDRR , R = 4 , D = 3 , R_r = 0 , D_r = 1
         // RDD RDRR , R = 4 , D = 3 , R_r = 0 , D_r = 2
         // RDD DRR , R = 3 , D = 3 , R_r = 0 , D_r = 1
         // RDDD RR , R = 3 , D = 3 , R_r = 0 , D_r = 2
         // RDDD R , R = 2 , D = 3 , R_r = 0 , D_r = 1
         // RDDD , R = 1 , D = 3 , R_r = 0 , D_r = 0
         // R DDD , R = 1 , D = 3 , R_r = 1 , D_r = 0
         // R DD , R = 1 , D = 2 , R_r = 0 , D_r = 0
         // RD D , R = 1 , D = 2 , R_r = 0 , D_r = 1
         // RDD , R = 1 , D = 2 , R_r = 0 , D_r = 2
         // DD , R = 0 , D = 2 , R_r = 0 , D_r = 1

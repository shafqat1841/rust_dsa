// dota2_senate_649

#![allow(dead_code)]
#![allow(unused)]

use std::collections::VecDeque;

fn sol1(senate: String) -> String {
    let senate = senate.as_bytes();

    let mut r_num = 0;
    let mut d_num = 0;

    let mut remainning = VecDeque::<u8>::new();

    for i in 0..senate.len() {
        let ele = senate[i];

        if ele == b'R' {

            r_num += 1;

            remainning.push_back(ele);
            
            
        } else {
            
            d_num += 1;
            remainning.push_back(ele);

        }

    }

    while r_num > 0 && d_num > 0 {

        let ele = remainning.pop_front();

        let ele = match ele {
            Some(v) => v,
            None => panic!("Error for pop front of ele in while for both"),
        };

        if ele == b'R' {
            
            remainning.push_back(ele);

            let mut not_found = true;

            while not_found {
                let ele2 = remainning.pop_front();

                let ele2 = match ele2 {
                    Some(v) => v,
                    None => panic!("Error for pop front of ele2 in while for b'R'"),
                };

                if ele2 == b'D' {
                    d_num -= 1;
                    not_found = false;
                } else {
                    remainning.push_back(ele2);  
                }
            }
            
            
        } else {
            
            remainning.push_back(ele);

            let mut not_found = true;

            while not_found {
                let ele2 = remainning.pop_front();


                let ele2 = match ele2 {
                    Some(v) => v,
                    None => panic!("Error for pop front of ele2 in while for b'D'"),
                };

                if ele2 == b'R' {
                    r_num -= 1;
                    not_found = false;
                } else {
                    remainning.push_back(ele2);  
                }
            }
            

        }
    }

    let f_ele = remainning.front();

    let f_ele = match f_ele {
        Some(v) => v,
        None => panic!("Error for pop front of f_ele"),
    };

    if *f_ele == b'R' {
        "Radiant".to_string()        
    } else {
        "Dire".to_string()        
    }


}

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

        
        let input = "RDDDRDR".to_string();
        let res = Solution::predict_party_victory(input);
        let output = "Dire".to_string();
        assert_eq!(output, res);

    }

}

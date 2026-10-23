// equal_row_and_column_pairs_2352

#![allow(dead_code)]
#![allow(unused)]

// [[3,2,1],[1,7,6],[2,7,7]] = 1
// [3,2,1]
// [1,7,6]
// [2,7,7]

// [[13,13],[13,13]] = 4
// [13,13]
// [13,13]

// [[13,13,13],[13,13,13],[13,13,13]] = 4
// [13,13,13]
// [13,13,13]
// [13,13,13]

// [[3,1,2,2],[1,4,4,5],[2,4,2,2],[2,4,2,2]]
// [3,1,2,2]
// [1,4,4,5]
// [2,4,2,2]
// [2,4,2,2]


//
fn sol1(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut col_map = HashMap::<String,i32>::new();
    let mut row_map = HashMap::<String,i32>::new();
    
    let len: usize = grid.len();
    let mut index = 0;
    
    while index < len {
        let mut index2 = 0;
        let mut col_vals: Vec<i32> = Vec::new();
        
        while index2 < len {
            let val = grid[index2][index];
            col_vals.push(val);
            index2 += 1;
        }

        let col_string = format!("{:?}",col_vals);
        let row_string = format!("{:?}", grid[index]);

        col_map.entry(col_string).and_modify(|val|{ *val += 1 }).or_insert(1); 
        row_map.entry(row_string).and_modify(|val|{ *val += 1 }).or_insert(1); 

        index += 1;
    } // time = n^2 , space = n^2

    let mut match_vals: i32 = 0;

    for (key,&val) in &col_map {

        if let Some(v) = row_map.get(key) {

            let mut value = val * v;
    
            match_vals += value;

        }
        
    } // time = n , space = 1

    match_vals
}
// total time = n^2+n
// total space = n^2

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        sol1(grid)
    }
}

#[cfg(test)]
mod equal_row_and_column_pairs_2352_tests {
    use super::*;

    #[test]
    fn main() {
        let input =  [[3,2,1].to_vec(),[1,7,6].to_vec(),[2,7,7].to_vec()].to_vec();
        let res = 1;
        let output = Solution::equal_pairs(input);
        assert_eq!(output,res)
    }

}

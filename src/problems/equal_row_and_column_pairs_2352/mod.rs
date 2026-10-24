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


// accepted
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
// total time = n^3
// total space = n^2

// [3,1,2,2]
// [1,4,4,4]
// [2,4,2,2]
// [2,5,2,2]

// [3,1,2,2] [1,4,4,4] [2,4,2,2] [2,5,2,2]
// [3,1,2,2] [1,4,4,5] [2,4,2,2] [2,4,2,2]

// matched 
// [3,1,2,2] = 1
// [2,4,2,2] = 2
// total matched = 3 


fn sol2(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    let mut col_map = HashMap::<Vec<i32>, i32>::new();
    let mut row_map = HashMap::<Vec<i32>, i32>::new();
    
    let len: usize = grid.len(); // n
    let mut index = 0;
    
    while index < len {
        let mut index2 = 0;
        let mut col_vals: Vec<i32> = Vec::with_capacity(len);
        
        while index2 < len {
            let val = grid[index2][index];
            col_vals.push(val);
            index2 += 1;
        } // time = n , space = n 

        let row_clone = grid[index].clone();  // time = n , space = n

        col_map.entry(col_vals).and_modify(|val| { *val += 1 }).or_insert(1); // time = n , space = n
        row_map.entry(row_clone).and_modify(|val| { *val += 1 }).or_insert(1); // time = n , space = n

        index += 1;
    }
    // total time = n * ( n + n + n + n ) => n * n => n^2
    // total space = n * ( n + n + n + n ) => n * n => n^2

    let mut match_vals: i32 = 0;

    for (key, &val) in &col_map {
        let row_map_res = row_map.get(key); // time = n , space = 1
        if let Some(v) = row_map_res {
            let value = val * v;
            match_vals += value;
        }
    } 
    // total time = n * n => n^2
    // total space = 1

    match_vals
}
// total time = n^2 + n^2 => 2n^2 => n^2
// total space = 1 + n^2 => n^2
// Overall Time Complexity:  O(n²) [from O(n²) + O(n²)]
// Overall Space Complexity: O(n²) [from storing row and column vectors in HashMaps]

fn sol3(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    let n = grid.len();
    let mut row_counts = HashMap::new();

    for row in &grid {
        let row_clone = row.clone();  // time = n , space = n
        *row_counts.entry(row_clone).or_insert(0) += 1; // time = n , space = n
    } 
    // total time = n * ( n + n ) => n* n => n^2
    // total space = n * ( n + n ) => n* n => n^2

    let mut total_pairs = 0;

    for c in 0..n {
        let mut col = Vec::with_capacity(n);

        for r in 0..n {
            col.push(grid[r][c]); // time = 1 , space = 1
        }  
        // total time = n * 1 => n
        // total space = n * 1 => n

        let col_res = row_counts.get(&col); // time = n , space = 1

        if let Some(&count) = col_res {
            total_pairs += count;
        }
    } 
    // total time =  n * ( n + n) =>  n * 2n => n * n => n^2 
    // total space = n * ( n + 1 ) => n * n => n^2

    total_pairs
} 
// total time = n^2 + n^2 => n^2
// total space = n^2 + n^2 => n^2

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        sol2(grid)
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

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
    
    // Outer loop runs n times.
    // Inside:
    // - Building col_vals takes O(n) time and O(n) space.
    // - Hashing and inserting col_vals into col_map takes O(n) time.
    // - Cloning grid[index] takes O(n) time and O(n) space.
    // - Hashing and inserting grid[index] into row_map takes O(n) time.
    // Total per iteration: O(n) time. Over n iterations, this block takes O(n²).
    while index < len {
        let mut index2 = 0;
        let mut col_vals: Vec<i32> = Vec::with_capacity(len);
        
        while index2 < len {
            let val = grid[index2][index];
            col_vals.push(val);
            index2 += 1;
        }

        col_map.entry(col_vals).and_modify(|val| { *val += 1 }).or_insert(1); 
        row_map.entry(grid[index].clone()).and_modify(|val| { *val += 1 }).or_insert(1); 

        index += 1;
    } 
    // Subtotal Phase 1: Time = O(n²), Space = O(n²) to store up to n vectors of length n in maps

    let mut match_vals: i32 = 0;

    // Iterates over at most n unique keys in col_map.
    // Inside:
    // - row_map.get(key) hashes an n-element vector in O(n) time.
    // Total per iteration: O(n) time. Over at most n entries, this block takes O(n²).
    for (key, &val) in &col_map {
        if let Some(v) = row_map.get(key) {
            let value = val * v;
            match_vals += value;
        }
    } 
    // Subtotal Phase 2: Time = O(n²), Space = O(1) auxiliary space

    match_vals
}
// Overall Time Complexity:  O(n²) [from O(n²) + O(n²)]
// Overall Space Complexity: O(n²) [from storing row and column vectors in HashMaps]

fn sol3(grid: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;

    let n = grid.len();
    let mut row_counts = HashMap::new();

    for row in &grid {
        *row_counts.entry(row.clone()).or_insert(0) += 1;
    } // time = n^2 , space = n^2

    let mut total_pairs = 0;

    for c in 0..n {
        let mut col = Vec::with_capacity(n);

        for r in 0..n {
            col.push(grid[r][c]);
        }  // time = n , space = n

        if let Some(&count) = row_counts.get(&col) {
            total_pairs += count;
        }
    } // time = n^2 , space = n

    total_pairs
} 
// total time = n^2 + n^2 => n^2
// total space = n^2 + n => n^2

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

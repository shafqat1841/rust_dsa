#[allow(dead_code)]
struct Solution;

fn solution_1(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut new_arr = candies.clone(); // time = n , space = n
    new_arr.sort_unstable(); // time = n log n
    let high_value = new_arr[new_arr.len() - 1]; // time 1

    let mut result: Vec<bool> = vec![]; // space = n

    for candy in candies {
        let new_candy = candy + extra_candies;

        if new_candy < high_value {
            result.push(false);
        }else {
            result.push(true);
        }
    } // time = n

    // total space = 2 n
    // total time = n logn

    result
}


fn solution_2(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let mut highest_value = 0;

     for candy in candies.iter() {
        if highest_value < *candy {
            highest_value = *candy;
        }
     } // time = n , space = 1

    let mut result: Vec<bool> = Vec::with_capacity(candies.len()); // time = 1 , space = n

    for candy in candies {
        let new_candy = candy + extra_candies;

        if new_candy < highest_value {
            result.push(false);
        }else {
            result.push(true);
        }
    } // time = n , space = 1

    // total space = n
    // total time = 2n

    result
}

impl Solution {
    #[allow(dead_code)]
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        // solution_1(candies, extra_candies)
        solution_2(candies, extra_candies)
    }

}

#[cfg(test)]
mod kids_with_the_greatest_number_of_candies_1431_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let candies = vec![2,3,5,1,3];
        let extra_candies = 3;
        let correct_result = [true,true,true,false,true];
        let result = Solution::kids_with_candies(candies,extra_candies);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_2() {
      let candies = vec![4,2,1,1,2];
        let extra_candies = 1;
        let correct_result = [true,false,false,false,false];
        let result = Solution::kids_with_candies(candies,extra_candies);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_3() {
     let candies = vec![12,1,12];
        let extra_candies = 10;
        let correct_result = [true,false,true];
        let result = Solution::kids_with_candies(candies,extra_candies);
        assert_eq!(result, correct_result);
    }

}

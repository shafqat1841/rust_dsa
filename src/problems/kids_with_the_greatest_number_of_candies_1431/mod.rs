#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        vec![true]
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

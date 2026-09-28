#![allow(dead_code)]

fn sol1(flowerbed: Vec<i32>, n: i32) -> bool {
    if flowerbed.len() == 1 {
        if flowerbed[0] == 0 && n == 1 {
            return true;
        }

        if flowerbed[0] == 0 && n == 0 {
            return true;
        }

        if flowerbed[0] == 1 && n == 0 {
            return true;
        }

        return false;
    }

    let mut slot = 0;

    let mut zeros = 0;

    let mut i1 = 0;

    loop {
        let value = flowerbed[i1];

        if value == 0 {
            zeros += 1;
        }

        if value == 1 || i1 == flowerbed.len() - 1 {
            if zeros == 0 {
                i1 += 1;
                break;
            }

            let mut end = zeros - 1;
            if i1 == flowerbed.len() - 1 && value == 0 {
                end = zeros;
            }

            for i in 1..=end {
                if i == 1 || i % 2 != 0 {
                    slot += 1;
                }
            }

            i1 += 1;

            break;
        }

        i1 += 1;

        if i1 == flowerbed.len() {
            break;
        }
    }

    zeros = 0;

    let mut i3 = i1;

    loop {
        if i3 >= flowerbed.len() {
            break;
        }

        let value = flowerbed[i3];

        if value == 0 {
            zeros += 1;
        }

        if value == 1 || i3 == flowerbed.len() - 1 {
            let mut end = zeros - 1;
            if i3 == flowerbed.len() - 1 && value == 0 {
                end = zeros;
            }
            for i in 1..=end {
                if i % 2 == 0 {
                    slot += 1;
                }
            }

            zeros = 0;
        }

        i3 += 1;
    }

    if slot >= n {
        return true;
    }

    false
}

fn sol2(mut flowerbed: Vec<i32>, n: i32) -> bool {
    let mut count = 0;

    let mut i = 0;

    while i < flowerbed.len() {
        if flowerbed[i] == 0 {
            let mut left = false;
            let mut right = false;

            if i == 0 || flowerbed[i - 1] == 0 {
                left = true;
            }

            if i == flowerbed.len() - 1 || flowerbed[i + 1] == 0 {
                right = true;
            }

            if left & right {
                count += 1;

                flowerbed[i] = 1;

                if count >= n {
                    return true;
                }
            }
        }

        i += 1;
    }

    count >= n
}

fn sol3(mut flowerbed: Vec<i32>, n: i32) -> bool {
    let mut count = 0;
    let len = flowerbed.len();

    for i in 0..len {
        if flowerbed[i] == 0 {
            let is_left_empty = i == 0 || flowerbed[i - 1] == 0;
            let is_right_empty = i == len - 1 || flowerbed[i + 1] == 0;

            if is_left_empty && is_right_empty {
                flowerbed[i] = 1;
                count += 1;

                if count >= n {
                    return true;
                }
            }
        }
    }

    count >= n
}

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        // sol1(flowerbed, n)
        // sol2(flowerbed, n)
        sol3(flowerbed, n)
    }
}

#[cfg(test)]
mod can_place_flowers_605_tests {
    use super::*;

    #[test]
    fn check_no_1() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 1;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_2() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        let n = 2;
        let correct_result = false;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_3() {
        let flowerbed = vec![1, 0, 0, 0, 1, 0, 1];
        let n = 1;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_4() {
        let flowerbed = vec![0, 0, 1, 0, 1];
        let n = 1;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_5() {
        let flowerbed = vec![1, 0, 0, 0, 1, 0, 0];
        let n = 2;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_6() {
        let flowerbed = vec![1, 0, 0, 0, 0, 0, 1];
        let n = 2;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_7() {
        let flowerbed = vec![0, 1, 0];
        let n = 1;
        let correct_result = false;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_8() {
        let flowerbed = vec![0, 0, 1, 0, 0];
        let n = 2;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_9() {
        let flowerbed = vec![0];
        let n = 1;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_10() {
        let flowerbed = vec![1];
        let n = 1;
        let correct_result = false;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_11() {
        let flowerbed = vec![1];
        let n = 0;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_12() {
        let flowerbed = vec![0, 0, 0, 0, 1];
        let n = 2;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_13() {
        let flowerbed = vec![1, 0, 0, 0, 0, 1];
        let n = 2;
        let correct_result = false;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_14() {
        let flowerbed = vec![1, 0, 0];
        let n = 1;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_15() {
        let flowerbed = vec![0, 0];
        let n = 1;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_16() {
        let flowerbed = vec![0, 0, 0];
        let n = 2;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_17() {
        let flowerbed = vec![0, 0, 0, 0, 0];
        let n = 3;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }

    #[test]
    fn check_no_18() {
        let flowerbed = vec![0];
        let n = 0;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }
}

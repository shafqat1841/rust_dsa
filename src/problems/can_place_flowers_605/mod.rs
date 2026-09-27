#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let flowerbed_len = flowerbed.len() as i32;

        // let space1 = (n * 2) + 1;
        // let space2 = n + 1;

        let space1 = 3;
        let space2 = 2;

        let mut zeros = 0;
        let mut start = true;

        let mut found = 0;

        let mut index = 0;

        for i in flowerbed {
            if i == 0 {
                zeros += 1;
            }

            if i == 1 {
                if start && zeros >= space2 {
                    found += 1;
                    if found == n {
                        return true;
                    }
                }

                start = false;

                if !start && zeros >= space1 {
                    found += 1;
                    if found == n {
                        return true;
                    }
                }

                zeros = 0;
            }

            if i == 0 && index == flowerbed_len - 1 && zeros >= space2 {
                found += 1;
                if found == n {
                    return true;
                }
            }

            index += 1;
        }

        false
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
        let flowerbed = vec![1,0,0,0,0,0,1];
        let n = 2;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }
}

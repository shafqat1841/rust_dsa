use std::collections::HashSet;

fn sol1(flowerbed: Vec<i32>, n: i32) -> bool {
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

fn sol2(flowerbed: Vec<i32>, n: i32) -> bool {
    let length = flowerbed.len();

    let mut zeros = 0;

    let mut start = true;

    let mut slot = 0;

    for flower in flowerbed.iter().enumerate() {
        println!("index: {:?}, i: {:?}, zeros: {}", flower.0, flower.1, zeros);
        let index = flower.0;
        let i = *flower.1;

        if i == 1 {
            if index == 0 {
                start = false;
            } else {
                if start {
                    start = false;
                    for num in zeros..index - 1 {
                        if num == 0 || num % 3 == 0 {
                            slot += 1;
                        }
                    }
                } else {
                    for num in zeros..index - 1 {
                        if num % 2 == 0 {
                            slot += 1;
                        }
                    }
                }
            }

            zeros = index + 1;
        }
    }

    for num in zeros..length {
        if num % 2 == 0 {
            slot += 1;
        }
    }

    if slot == n {
        return true;
    }

    false
}

fn sol3(flowerbed: Vec<i32>, n: i32) -> bool {

    if flowerbed.len() == 1{
        if flowerbed[0] == 0 && n == 1 {
            return true;
        }

        if flowerbed[0] == 1 && n == 0 {
            return true;
        }
        return false;
    }

    let mut gaps_data: Vec<[usize; 2]> = Vec::new();

    let mut gap_point = 0;

    for i in 0..flowerbed.len() {
        if flowerbed[i] == 1 {
            if i != 0 {
                let gap = [gap_point, i];
                gaps_data.push(gap);
            }
            gap_point = i + 1;
        }

        if i == flowerbed.len() - 1 && flowerbed[i] == 0 {
            let gap = [gap_point, i + 1];
            gaps_data.push(gap);
        }
    }

    let mut slot = 0;

    for i in 0..gaps_data.len() {
        let start = gaps_data[i][0];
        let end = gaps_data[i][1];

        if start == 0 {
            for index in start..end - 1 {
                if index == 0 || index % 3 == 0 {
                    println!("start");
                    slot += 1;
                }
            }
        } else if end == flowerbed.len() {
              for index in start+1..end {
                if index % 2 == 0 {
                    println!("end");
                    slot += 1;
                }
            }

        }   else {
            for index in start+1..end - 1 {
                if index == 0 || index % 2 == 0 {
                    println!("else");
                    slot += 1;
                }
            }
        }
    }

    println!("=================");
    println!("flowerbed: {:?}", flowerbed);
    println!("gaps_data: {:?}", gaps_data);
    println!("slot: {}", slot);
    println!("=================");

    if slot >= n {
        return true;
    }

    false
}

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        //    sol1(flowerbed, n)
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
        let flowerbed = vec![0,0,1,0,0];
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
        let flowerbed = vec![0,0,0,0,1];
        let n = 2;
        let correct_result = true;
        let result = Solution::can_place_flowers(flowerbed, n);
        assert_eq!(result, correct_result);
    }
}

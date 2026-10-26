// asteroid_collision_735

#![allow(dead_code)]
#![allow(unused)]


// accepted
fn sol1(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    
    for &ast in &asteroids {
        let mut alive = true;

        while alive && !stack.is_empty() && ast < 0 && *stack.last().unwrap() > 0 {
            let top = *stack.last().unwrap();
            
            if top < -ast {
                stack.pop();
            } else if top == -ast {
                stack.pop();
                alive = false;
            } else {
                alive = false;
            }
        }
        
        if alive {
            stack.push(ast);
        }
    }
    
    stack
}
// total time = n
// total space = n

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        sol1(asteroids)
    }
}

#[cfg(test)]
mod asteroid_collision_735_tests {
    use super::*;


    #[test]
    fn main() {
        let input =  [-2,4,-5,10,-2].to_vec();
        let res = [-2,-5,10].to_vec();
        let output = Solution::asteroid_collision(input);
        assert_eq!(output,res)
    }

}

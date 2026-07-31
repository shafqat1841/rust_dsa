pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod merge_strings_alternately_1768_test {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
pub fn add_five(num: u32) -> u32 {
    num + 5
}
pub fn sub_ten(num: u32) -> u32 {
    num - 10
}
// cargo test
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_add_five() {
        assert_eq!(add_five(5), 10);
    }
    #[test]
    fn test_sub_ten() {
        assert_eq!(sub_ten(30), 20);
    }
}

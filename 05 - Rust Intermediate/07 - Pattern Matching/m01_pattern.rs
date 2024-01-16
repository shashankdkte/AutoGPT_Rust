#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_literals() {
        let number: i32 = 20;
        match number {
            1 => println!("one"),
            2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => println!("prime"),
            _ => println!("composite"),
        };
    }
    #[test]
    fn test_match_literals_string() {
        let number: i32 = 20;
        let result: &str = match number {
            1 => "one",
            2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => "prime",
            _ => "composite",
        };
        println!("{}", result);
    }

    #[test]
    fn test_match_option() {
        let some_num: Option<i32> = Some(20);
        let prob_none: Option<i32> = None;

        let res = match some_num {
            Some(x) => x,
            None => {
                panic!("There was some problem")
            }
        };
        println!("{}", res);
        println!("{:?}", some_num);
    }
    #[test]
    fn test_match_option_1() {
        let some_num: Option<i32> = Some(20);
        let my_int: i32 = if let Some(x) = some_num {
            x
        } else {
            panic!("There was a problem")
        };
        println!("{}", my_int);
    }

    #[test]
    fn test_match_result() {
        let some_result: Result<i32, &str> = Ok(20);
        let some_err: Result<i32, &str> = Err("There was a problem");
        let res = match some_result {
            Ok(x) => x,
            Err(e) => panic!("{}", e),
        };
        print!("{}", res);
    }
}

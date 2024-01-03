/**
Enums group related values together
similar to unions
cargo test test_enums -- --nocapture
*/
#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err(String::from("Number is greater than 5"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enums() {

        let result: GivenResult<u8, String> = check_under_five(3);
        dbg!(result);
    }
}

/**
Enums group related values together
similar to unions

*/
#[derive(Debug)]
enum CarColor {
    Red,
    Green,
    Blue,
    Silver,
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}
#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T),
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(remainder)
    } else {
        GivenOption::None
    }
}
fn remainder_zero_built_in(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }
}

fn check_under_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err(String::from("Number is greater than 5"))
    }
}

fn check_under_five_built_in(num_check: u8) -> Result<u8, String> {
    if num_check < 5 {
        Ok(num_check)
    } else {
        Err(String::from("Number is greater than 5"))
    }
}
fn create_car_color_blue() -> CarColor {
    let my_car_color: CarColor = CarColor::Blue;
    my_car_color
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enums() {
        let car_color: CarColor = create_car_color_blue();
        dbg!(car_color);

        let result: GivenResult<u8, String> = check_under_five(3);
        dbg!(result);

        let result: Result<u8, String> = check_under_five_built_in(3);
        dbg!(result);

        let remainder: Option<f32> = remainder_zero_built_in(12.2);
        // dbg!(remainder)
    }
}

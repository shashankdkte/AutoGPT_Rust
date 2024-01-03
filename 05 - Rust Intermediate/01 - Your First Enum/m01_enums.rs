/**
Enums group related values together
similar to unions
cargo test test_enums -- --nocapture
*/
#[derive(Debug)]
enum CarColor {
    Red,
    Green,
    Blue,
    Silver,
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
    }
}

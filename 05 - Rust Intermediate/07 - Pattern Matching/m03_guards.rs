#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_match_guard() {
        let pair: (i32, i32) = (2, -2);

        match pair {
            (x, y) if x > 0 && y > 0 => println!("both positive"),
            (x, y) if x < 0 && y < 0 => println!("both negative"),
            (x, y) if x < 0 && y > 0 => println!("first negative"),
            (x, y) if x > 0 && y < 0 => println!("second negative"),
            (x, y) => println!("neither negative"),
        }
    }
    #[test]
    fn tests_match_struct() {
        struct Location {
            x: i32,
            y: i32,
        }

        let location: Location = Location { x: 0, y: 20 };
        match location {
            Location { x: 1, y: 2 } => println!("one"),
            _ => println!("other"),
        }
    }
}

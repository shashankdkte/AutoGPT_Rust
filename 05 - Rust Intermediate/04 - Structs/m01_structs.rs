#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structs() {
        let user_1: User = User {
            username: String::from("someusername123"),
            email: String::from("mynbi@example.com"),
            sign_in_count: 1,
            active: true,
        };
        dbg!(user_1);
    }
}

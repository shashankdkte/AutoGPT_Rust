#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
impl User {
    fn incremant_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }
    fn change_email(&mut self, new_email: &str) {
        self.email = String::from(new_email);
    }
}
fn change_username(user: &mut User, new_username: &str) {
    user.username = String::from(new_username);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structs() {
        let mut user_1: User = User {
            username: String::from("someusername123"),
            email: String::from("mynbi@example.com"),
            sign_in_count: 1,
            active: true,
        };

        change_username(&mut user_1, "newusername");

        dbg!(user_1);

        let mut user_2: User = User {
            username: String::from("naruto@konoha"),
            email: String::from("naruto@konoha.com"),
            sign_in_count: 0,
            active: true,
        };

        user_2.change_email("hokage@konoha.com");
        user_2.incremant_sign_in_count();

        dbg!(user_2);
    }
}

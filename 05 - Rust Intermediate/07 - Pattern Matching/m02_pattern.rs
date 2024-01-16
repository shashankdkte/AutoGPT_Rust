#[derive(Debug)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("Quitting")
        }
        Message::ChangeColor(r, g, b) => {
            println!("Changing color to r: {}, g: {}, b: {}", r, g, b)
        }
        Message::Move { x, y } => {
            println!("Moving to x: {}, y: {}", x, y)
        }
        Message::Write(text) => {
            println!("Writing: {}", text)
        }
    };
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_large_enum() {
        let my_option: Message = Message::Quit;
        process_message(my_option);
    }
}

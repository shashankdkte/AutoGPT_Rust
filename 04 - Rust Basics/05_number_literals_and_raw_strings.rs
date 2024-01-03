fn main() {
    println!("Big number is {}", 98_222_000);
    println!("Hex is {}", 0xff);
    println!("Octal is {}", 0o77);
    println!("Binary is {}", 0b1010);
    println!("Byte a is {}", b'a');

    //Raw String Literals
    let text: &str = r#"This is  "great""#;
    println!("{}", text);
}

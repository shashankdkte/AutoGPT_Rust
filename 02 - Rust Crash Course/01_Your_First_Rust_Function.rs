fn main() {
    println!("Hello, world!");
    let mut num1: u32 = 50;
    println!("Initial value of {}", num1);
    num1 = add_five(num1);
    println!("Final Value of {}", num1);
}

fn add_five(num: u32) -> u32 {
    num + 5
}

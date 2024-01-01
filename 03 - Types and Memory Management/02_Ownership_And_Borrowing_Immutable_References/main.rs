fn main() {
    // Works
    let number1: i32 = 50;
    let number2: i32 = 80;
    println!("Number1 is {}", number1);
    println!("Number2 is {}", number2);

    // Will Not Work
    // Ownership of  "hello" no longer with greet
    //Shallow Copy
    /*
    let greet: String = String::from("Hello");
    let greet2: String = greet;
    println!("{}", greet);
    */
    //Will Work
    //Deep Copy
    let greet: String = String::from("Hello");
    let greet2: String = greet.clone();
    println!("This is greet {}", greet);
    println!("This is greet2 {}", greet2);

    //Works
    //Borrowing
    let greet: String = String::from("Hello");
    let greet2: &String = &greet;
    let greet3: &String = &greet;
    println!("This is greet {}", greet);
    println!("This is greet2 {}", greet2);
    println!("This is greet3 {}", greet3);
    //Does not Work
    // let r: &String = make_string_dangle();
    let s: String = no_string_dangle();
}
/*
fn make_string_dangle() -> &String {
    let s = String::from("Hello");
    let r: &String = &s;
    r
}
*/
fn no_string_dangle() -> String {
    let s = String::from("Hello");
    s
}

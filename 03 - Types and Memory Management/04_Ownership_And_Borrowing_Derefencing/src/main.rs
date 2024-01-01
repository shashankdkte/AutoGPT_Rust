fn main() {
    let name: String = String::from("John");
    let fullname: &String = &name;
    //Value of Pointer
    println!("Hello, {:p}!", fullname);
    //Derefernce Coercion  *fullname
    println!("Hello, {}!", fullname);

    let mut new_name = String::from("John");
    let new_name_ref: &mut String = &mut new_name;

    *new_name_ref = String::from("Bob");
    println!("Hello, {}!", new_name);

    let mut x: i32 = 50;
    x = 70;
    dbg!(x);

    let y: &mut i32 = &mut x;
    *y += 1;
    dbg!(y);
}

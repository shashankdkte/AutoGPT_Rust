fn main() {
    let mut name: String = String::from("John");
    let fullname: &mut String = &mut name;
    println!("The name is {}", fullname);
    generate_fullname(fullname);
    println!("The name is {}", fullname);
}

fn generate_fullname(name: &mut String) {
    name.push_str(" Smith");
}

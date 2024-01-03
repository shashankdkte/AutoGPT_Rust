fn main() {
    let name: &str = "John";
    println!("name is {:?}", name);

    let dynamic_name: String = String::from("John Doe");
    println!("dynamic_name is {:?}", dynamic_name);
    println!("dynamic_name is stored in memory at {:p}", &dynamic_name);

    let pen_name = name.to_string();
    println!("pen_name is {:?}", pen_name);

    let str_slice: &str = &dynamic_name[0..4];
    println!("str_slice is {:?}", str_slice);

    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.push('l');
    chars.push('o');
    chars.push('!');

    println!("chars is {:?}", chars);

    let removed_char: char = chars.pop().unwrap();
    println!("removed character is {:?}", removed_char);
    println!("chars is {:?}", chars);
}

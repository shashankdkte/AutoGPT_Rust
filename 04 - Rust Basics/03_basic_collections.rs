fn main() {
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

    chars.iter().for_each(|c: &char| print!("{}", c));

    let chars_again: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    println!("chars_again is {:?}", chars_again);

    let collected: String = chars_again.iter().collect();
    println!("{:?}", collected);
}

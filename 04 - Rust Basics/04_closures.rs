fn main() {
    let num: i32 = 42;
    let add_num = |x: i32| x + num;
    println!("{}", add_num(2));
}

const OUR_COURSE: &str = "Rust";
fn main() {
    println!("Welcome to Programming {}", OUR_COURSE);

    //Stack
    let x: i32;
    x = 10;

    println!("X is {}", x);
    let y: i32 = 4;
    println!("Y is {}", y);

    //For Loop
    for i in 0..y {
        println!("i is {}", i);
    }

    //Mutation
    let mut z: i32 = 5;
    println!("Z was {}", z);
    z = 7;
    println!("but now it is {}", z);

    let freezing_temp: f64 = -2.4;
    println!("freezing temp is {}", freezing_temp);

    let is_zero_remainder: bool = 10 % 4 != 0;
    println!("is zero remainder {}", is_zero_remainder);

    let my_char: char = 'a';
    println!("my char is {}", my_char);

    let emoji_char: char = '\u{1F600}';
    println!("emoiji char is {}", emoji_char);

    let my_floats: [f32; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    println!("my ints are {:?}", my_floats);

    let your_floats: [f32; 5] = my_floats.map(|x| x + 2.0);
    println!("your floats are {:?}", your_floats);
}

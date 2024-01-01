const MY_INTEGER: u8 = 10;
fn main() {
    //Stack
    let num1: u8 = 50;
    println!("num1 is {}", num1);

    //Heap
    let mut arr: Vec<u8> = vec![1, 2, 3, 4, 5];
    println!("arr is {:?}", arr);
    arr.push(6);
    println!("After pushing arr is {:?}", arr);

    // Reference on stack pointing to Heap
    let arr_2 = &arr[0..3];
    println!("arr_2 is {:?}", arr_2);

    //Heap
    let mut name: String = String::from("John");
    println!("name is {}", name);
    name.push_str(" Doe");
    println!("name is {}", name);

    // Reference on stack pointing to Heap
    let first = &name[0..4];
    println!("first is {}", first);

    println!("MY INTEGER IS {:?}", MY_INTEGER);
}

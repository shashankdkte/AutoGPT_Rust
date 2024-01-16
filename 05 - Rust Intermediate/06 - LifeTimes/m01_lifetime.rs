pub fn example_1() {
    let r: &i32;

    let x: i32 = 5;
    r = &x;

    println!("{}", r);
}
pub fn example_2() {
    //Allocate space in memory
    let highest_age: i32;

    //Initialize vars
    let alice_age: i32 = 20;
    let bob_age: i32 = 30;

    // Call function
    highest_age = largest(&alice_age, &bob_age);
    //Print output
    println!("Highest age {}", highest_age);
    fn largest(compare1: &i32, compare2: &i32) -> i32 {
        if (compare1 > compare2) {
            *compare1
        } else {
            *compare2
        }
    }
}

pub fn example_3() {
    //Allocate space in memory
    let highest_age: &i32;

    //Initialize vars
    let alice_age: i32 = 20;
    let bob_age: i32 = 30;

    // Call function
    highest_age = largest(&alice_age, &bob_age);
    //Print output
    println!("Highest age {}", highest_age);
    fn largest<'a>(compare1: &'a i32, compare2: &'a i32) -> &'a i32 {
        if (compare1 > compare2) {
            compare1
        } else {
            compare2
        }
    }
}

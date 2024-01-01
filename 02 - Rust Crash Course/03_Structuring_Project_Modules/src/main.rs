mod math_functions;
mod my_funcs;

use crate::math_functions::add_value::{add_ten, add_twenty};
use crate::math_functions::mul_value::{mul_ten, mul_twenty};

use crate::my_funcs::{add_five, sub_ten};
fn main() {
    println!("Hello, world!");
    let mut num1: u32 = 50;
    println!("Initial value of {}", num1);
    num1 = add_five(num1);
    println!("After Adding Value of {}", num1);
    num1 = sub_ten(num1);
    println!("After Subtract Value of {}", num1);
    num1 = add_ten(num1);
    println!("After Add Value of {}", num1);
    num1 = add_twenty(num1);
    println!("After Add Value of {}", num1);
    num1 = mul_ten(num1);
    println!("After Mul Value of {}", num1);
    num1 = mul_twenty(num1);
    println!("After Mul Value of {}", num1);
}

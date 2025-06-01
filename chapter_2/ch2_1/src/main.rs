#![allow(unused_mut)]

fn main() {
    // By default: immutable
    // Auto deduction of type
    // By default: i32
    let a = 100;

    // Explicitly specify the data type
    let b: i32 = 10;

    // Explicitly specify the variable is mutable
    // which is different from the default immutable
    let mut c = 30i32;

    // Append the data type in the value
    let d = 30_i32;

    // Function return as parameters
    let e = add(add(a, b), add(c, d));

    println!(" ({} + {}) + ({} + {}) = {}", a, b, c, d, e);
}

fn add(a: i32, b: i32) -> i32 {
    // Expression rather than a statement
    // a + b; => this is a statement
    a + b
}

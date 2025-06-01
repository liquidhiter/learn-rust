#![allow(unused_variables)]  // Disable warning for the entire crate

use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");

    // Variable binding:
    // let a = "hello, world!"
    // "hello, world!" is a string literal
    // which is stored in the program's read-only
    // data segment
    // variable 'a' here is a reference to this read-only
    // data, NOTE: an immutable reference

    // String literal (&str) - stored in read-only memory
    let a = "hello, world!";
    
    // String type - stored on the heap, owned by the variable
    let mut b = String::from("hello, world!");
    b.push_str(" more text"); // Can modify because it's mutable and owned
    
    println!("String literal: {}", a);
    println!("Owned String: {}", b);

    fun();

    // Very similar to Python's syntax of unpacking
    // here return_tuple returns a (i32, i32)
    // let is used to unpack this tuple
    // I would expect that it can probably unpack
    // other data struct
    let (a, b): (i32, i32) = return_tupple();
    println!("{}, {}", a, b);

    // Unpack the assignment
    unpack_assignment();

    // Variable shadowing
    shadow();

    comparison_examples();

    // Absolute value examples
    let int_num: i32 = -42;
    let float_num: f64 = -3.14;
    
    println!("Absolute value of {} is {}", int_num, int_num.abs());
    println!("Absolute value of {} is {}", float_num, float_num.abs());
    
    // For unsigned types, abs() is not needed as they're always positive
    let unsigned_num: u32 = 42;
    println!("Unsigned number: {}", unsigned_num);
    
    // For floating point, you can also use abs() from std::f64
    println!("Alternative way: {}", f64::abs(float_num));
}

#[allow(unused_variables)]  // Disable warning for this function only
fn fun() {
    let x = 1;  // No warning because of the attribute above
    let y = 20;
}

fn return_tupple() -> (i32, i32) {
    (10, 20)
}

fn unpack_assignment() {
    let (a, b, c, d, e, f, g): (i32, i32, i32, i32, i32, i32, i32);

    struct Struct {
        e: i32
    }

    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5, 6];

    Struct { e, .. } = Struct { e: 5 };
    
    (f, g) = return_tupple();
    
    println!("a = {}, b = {}, c = {}, d = {}, e = {}, f = {}, g = {}", a, b, c, d, e, f, g);
}

fn shadow() {
    let x = 5;
    println!("x = {}, address: {:p}", x, &x);
    
    // New variable - different memory space
    let x = x + 1;
    println!("x = {}, address: {:p}", x, &x);

    {
        // Shadow
        let x = x << 1;
        println!("x = {}, address: {:p}", x, &x);
    }

    println!("x = {}, address: {:p}", x, &x);
}

fn comparison_examples() {
    // PartialEq examples
    let a = 5;
    let b = 10;
    println!("a == b: {}", a == b);  // Uses PartialEq
    println!("a != b: {}", a != b);  // Uses PartialEq

    // NaN example (demonstrates why floating point uses PartialEq)
    let nan = f64::NAN;
    println!("NaN == NaN: {}", nan == nan);  // false!

    // cmp examples
    let result = std::cmp::cmp(&a, &b);
    match result {
        Ordering::Less => println!("{} is less than {}", a, b),
        Ordering::Equal => println!("{} equals {}", a, b),
        Ordering::Greater => println!("{} is greater than {}", a, b),
    }

    // Custom type with PartialEq
    #[derive(PartialEq)]
    struct Person {
        name: String,
        age: u32,
    }

    let p1 = Person { name: String::from("Alice"), age: 30 };
    let p2 = Person { name: String::from("Bob"), age: 30 };
    println!("p1 == p2: {}", p1 == p2);  // false, because names are different
}
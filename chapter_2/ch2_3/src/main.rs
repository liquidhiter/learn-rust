use num::complex::Complex;

fn main() {
    // println!("Hello, world!");

    // saturating_add: result is within the MAX / MIN
    assert_eq!(100u8.saturating_add(1), 101);

    assert_eq!(u8::MAX.saturating_add(127), u8::MAX);

    let a : u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);

    let x = 3.01;
    let y : f32 = 5.0;
    println!("x = {}, y = {}", x, y);

    let a: f32 = 0.3;
    let b: f32 = 0.1 + 0.2;
    if a == b {
        println!("0.1 + 0.2 = 0.3");
    } else {
        println!("0.1 + 0.2 != 0.3");
    }

    // f32 / f64: std::cmp::PartialEq
    // instead of std::cmp
    println!("0.1 + 0.2 - 0.3 = {}", 0.1 + 0.2 - 0.3);

    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    let x = (-24.0_f32).sqrt();
    if x.is_nan() {
        println!("Not a number: {}", x);
    }

    if x == x {
        println!("x == x");
    } else {
        println!("x can't be compared with x");
    }

    // forty_twos[0]: 42.0, f32
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];
    println!("{:.2}", forty_twos[0]);

    // let bits : u8 = 255;
    // let b = bits << 8;

    for i in 1..5 {
        println!("{}", i);
    }

    for i in 1..=5 {
        println!("{}", i);
    }

    for i in 'a'..='z' {
        println!("{}", i);
    }

    let clx_a = Complex { re: 2.1, im: -1.2 };
    let clx_b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}

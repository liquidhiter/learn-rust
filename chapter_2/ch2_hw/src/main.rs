use std::ops::{Range, RangeInclusive};

// Disable warnings after pulling in other crates
// to avoid being overwritten
#[allow(unused_variables)]
#[allow(unused_assignments)]
// #[allow(arithmetic_overflow)]

fn main() {
    {
        // Variable must be firstly initialized before
        // any usage
        let x: i32 = 20;
        let y: i32;
        println!("x is equal to {}", x); 
    }

    {
        let mut x = 1;
        x += 2; 
        println!("x = {}", x); 
    }

    {
        let x: i32 = 10;
        let y: i32 = -5;
        {
            let y: i32 = 5;
            println!("x 的值是 {}, y 的值是 {}", x, y);
        }
        println!("x 的值是 {}, y 的值是 {}", x, y); 
    }

    {
        let x = define_x();
        println!("{}, world", x); 
    }

    {
        let x: i32 = 5;
        {
            let x = 12;
            // assert_eq!(x, 5);
        }
        // assert_eq!(x, 12);
        let x = 42;
        println!("{}", x); // 输出 "42".
    }

    {
        let mut x: i32 = 1;
        x = 7;
        // 遮蔽且再次绑定
        let mut x = x; 
        x += 3;
    
    
        let y = 4;
        // 遮蔽
        let y = "I can also be bound to text!"; 
    }

    {
        let _x = 1; 
    }

    {
        let (mut x, y) = (1, 2);
        x += 2;
    
        assert_eq!(x, 3);
        assert_eq!(y, 2);
    }

    {
        let (x, y);
        (x,..) = (3, 4);
        [.., y] = [1, 2];
        // 填空，让代码工作
        assert_eq!([x,y], [3, 2]);
    }

    {
        let x: i32 = 5;
        let mut y: u32 = 5;
        y = x as u32;
        // i32 by default
        let z = 10; // 这里 z 的类型是? 
    }

    {
        let v: u16 = 38_u8 as u16;
    }

    {
        let x = 5;
        assert_eq!("i32".to_string(), type_of(&x));
        println!("{}", type_of(&x));
    }

    {
        assert_eq!(i8::MAX, 127_i8); 
        assert_eq!(u8::MAX, 255_u8); 
    }

    {
        // 1024_i32 = 1024
        // 0xff_i32 = 255
        // 0o77_i32 = 7 * 8 + 7 = 63
        // 0b1111_1111 = 255
        // v = 1024 + 255 * 2 + 63 = 1597
        let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
        assert!(v == 1597);
    }

    {
        let x = 1_000.000_1; // f64
        let y: f32 = 0.12; // f32
        let z = 0.01_f64; // f64
        println!("type of x = {}", type_of(&x));
        println!("type of y = {}", type_of(&y));
    }

    {
        assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);
        let sum = 0.1 + 0.2;
        assert!(f64::abs(sum - 0.3) < 0.0001);
    }

    {
        let mut sum = 0;
        for i in -3..2 {
            sum += i
        }
        // -3 + -2 + -1 + 0 + 1 = -5
        assert!(sum == -5);
    
        for c in 97..=122 {
            println!("{}",c);
        }
    }

    {
        assert_eq!((1..5), Range{ start: 1, end: 5 });
        assert_eq!((1..=5), RangeInclusive::new(1, 5));
    }

    {
        // 整数加法
        assert!(1u32 + 2 == 3_u32);

        // 整数减法
        assert!(1i32 - 2 == -1i32);
        // assert!(1u8 - 2 == 255u8);
        assert!(1u8.wrapping_sub(2) == 255u8);
        
        assert!(3 * 50 == 150);

        assert!(9.6f32 / 3.2f32 == 3.0f32); // error ! 修改它让代码工作

        assert!(24 % 5 == 4);
        
        // 逻辑与或非操作
        assert!(true && false == false);
        assert!(true || false == true);
        assert!(!true == false);

        // 位操作
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
    }

}

fn define_x() -> String {
    let x: String = String::from("hello");
    x
}

/* Demo
 * How to get the type of a variable
 * Example usage:
 * ```
 * let x = 20_i32;
 * let typeOfX = type_of(&x);
 * println!("type of variable x is {}", typeOfX);
 * ```
 */
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
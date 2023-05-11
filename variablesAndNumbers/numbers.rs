//use std::num::complex::Complex;

fn main(){
    // integers
    let _x:i32 = -1000;

    let _y:u32 = 100;

    let _large_number: u64 = 123456789;

    // floats
    let mut _xf:f32 = -1.2345;
    _xf = -1.2345;

    // complex numbers
    //let complex_integer = Complex::new(10, 20);
    //let complex_float = Complex::new(10.0, 20.1);

    // binary literals
    let x:u8 = 0b10101010;
    let y:u8 = 0b01010101;
    dbg!(x | y);

    // hex literals
    let x:u8 = 0xAF;
    let largex = 0xABCDEF01u32;
    println!("{x}, {largex}");

    // boolean
    let _yes = true;
    let _no = false;

}
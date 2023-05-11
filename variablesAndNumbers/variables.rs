fn main(){
    let x = 1;
    println!("{x}");
    let x = 'i';
    println!("{x}");

    let something;
    let x = 5;
    //cannot use unintialized variable in rust
    //println!("x, something {x}, {something}");
    something = x * 5;
    println!("x, something {x}, {something}");

    let mut y = 0;
    y = y * 2 + x;
    println!("{y}");

    const BLAH:i32 = 42;
    y *= BLAH;

    println!("const and vriable value {BLAH},{y}")
}
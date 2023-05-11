fn main(){
    let z = multiply(5,6) ;
    println!("result : {z}");
    let txt = format!("result: {z}", z=z);
    println!("{}", txt);
}

fn multiply(x : i32,y:i32) -> i32{
    x * y
}
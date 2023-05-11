//creating global variable
static GLOBAL_VARIABLE : i32 = 1;



fn f(){
    //let local_variable = 2;
    
    subfn();
    //cant reassign immutable global variable in rust
    //global_variable = 9;
    let local_variable = 1;
    println!("global {GLOBAL_VARIABLE}");
    println!("local {local_variable}");
    //const BLAH : i32 = int("42");
    //println!("{BLAH}");
}
fn subfn(){
    let local_variable = 2;
    println!("local {local_variable}");
    println!("global {GLOBAL_VARIABLE}");
    let enclosed_variable = 3;
    println!("enclosed {enclosed_variable}");
}

fn main(){
    f();
}
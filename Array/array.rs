//array demo in rust

fn main(){
    let xs = [1,2,3,4,5];
    let ys = [0;500];

    println!("1st elemnet {}", xs[0]);
    println!("2nd element {}", xs[1]);

    println!("xs size {}", xs.len());
    println!("ys size {}", ys.len());

    println!("borrow a section of array as a slice {:?}", &xs[1..3]);
   // println!("{}",xs[5]);
}
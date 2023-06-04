fn main(){
    let many_types = (1u8, 2u32, 'a', true);

    println!("first value: {}",many_types.0);
    println!("second value: {}",many_types.1);

    let tuple_of_tuples = ((1u8, 2u32, 2u64), (4u8, -1i8, -2i32));
    println!("tuple of tuples: {:?}",tuple_of_tuples);

    let pair = (1u8, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    println!("one element tuple: {:?}", (5, ));
    println!("just an integer: {:?}", (5));

    let _tuple = (1u32, "hello", 4.5f32, true);
    let (a, b, c, d) = _tuple;
    println!("{},{},{},{}",a, b, c, d);

    #[derive(Debug)]
    struct Matrix(f32,f32,f32,f32);
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}",matrix)

}

fn reverse(pair: (u8,bool)) -> (bool,u8){
    let (integer, boolean) = pair;
    (boolean, integer)
}
fn main(){
    let mut xs = vec![1,2,3];
    println!("Initial list: {:?}", xs);

    println!("Push 4 into the list");
    xs.push(4);
    println!("list:{:?}", xs);

    println!("lists length:{}", xs.len());

    println!("Second element:{}", xs[1]);

    println!("Pop last element:{:?}", xs.pop());

    //println!("Fourth element:{}", xs[3]);

    println!("Contents of xs:");

    //xs.into_iter() moves ownership to x for each iteration. it works fine if we only want to print it,
    // but here we jst want to read its contents,we dont want to modify it or be the real owner of the elements
    //.iter() borrows reference immutable
    for x in xs.iter(){
        println!("> {}", x);
    }

    // enumerate every idex of the list
    
    //we have passed there iterator to an enumerator which returns a tuple of index,value of the iterator
    for (i, x) in xs.iter().enumerate(){
        println!("In position {} we have value {}",i,x);
    }

    //.iter_mut()  --> borrowed mutable reference
    //we directly modify the element inside the vector, we use * to dereference 
    for x in xs.iter_mut(){
        *x *= 3; //xs[i] = x * 3;
    }
    // more idiomatic
    //map func takes a closure and collect it into a vec
    let xs:Vec<i32> = xs.iter().map(|x| x*3).collect(); // [x * 3 for x in xs];
    println!("Updated list:{:?}", xs);

    //creating vector from ranges
    let collected_iterator:Vec<i32> = (0..10).collect(); // [x for x in range(0, 10)];
    println!("Collected (0..10) into:{:?}", collected_iterator);
    
}
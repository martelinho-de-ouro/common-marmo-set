// Vectors
// Strings
// HashMaps

// 
fn main() {
    let mut v1: Vec<i8> = Vec::new();
    println!("{:?}", v1);

    // using vec! macro and default i32, rust inferred that.
    let v2 = vec![1,1,2];
    println!("{:?}", v2);

    v1.push(1);
    v1.push(1);
    v1.push(2);
    println!("{:?}", v1);

    let second= &v2[1];
    println!("second from v2: {second}");

    // The .get returns an Option that we can use with `match`
    // We have these 2 ways to reference/get an element.
    // When using [] and trying to access an element that does not exist,
    // it will cause a panic.
    // The other way using .get, we can handle with the None.
    let second: Option<&i32> = v2.get(1);
    match second {
        Some(second) => println!("second from v2, again: {second}"),
        None => println!("N/A"),
    }

    let hold_my_beer = &v1[0];
    v1.push(9);
    // We can't do that at the same time.
    // "cannot borrow `v1` as mutable because it is also borrowed as immutable"
    // ------
    // vectors put the values next to each other in memory, 
    // adding a new element onto the end of the vector might require allocating 
    // new memory and copying the old elements to the new space, 
    // if there isn’t enough room to put all the elements next to each other 
    // where the vector is currently stored. 
    // In that case, the reference to the first element would be pointing 
    // to deallocated memory. The borrowing rules prevent programs 
    // from ending up in that situation
    // ------
    // println!("{:?}", hold_my_beer);

    for i in &v1 {
        println!("{i}");
    }

    for j in &mut v1 {
        // * dereference operator
        *j += 1;
        println!("{j}");
        
        // not possible to add or delete values inside for loop
        // v1.push(0);
        // v1.remove(0);
    }

    // We can use enum + vectors to store different types
    enum Car {
        Name(String),
        Year(i16),
    }
    
    let cars  = vec![Car::Name(String::from("foo")), Car::Year(1983)];

    {
        let v3 = vec![1,1,2];
    } // after this the memory is cleaned up.

}
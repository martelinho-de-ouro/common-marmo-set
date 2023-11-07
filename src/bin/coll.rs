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
    // println!("{:?}", hold_my_beer);
    
}
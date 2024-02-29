fn main() {
    // string literal ( static by default )
    let abc = "abc";
    println!("{}", abc);

    // explicit static string literal
    let def: &'static str = "def";
    println!("{}", def);

    // string object
    let ghi = String::from("ghi");
    println!("{}", ghi.len());

    // pushes string to string object
    let mut greeting = String::from("hello");
    greeting.push_str(" world");
    println!("{}", greeting);

    // converts literal into object
    let not_possible_to_use_foo_name: String =
        "change this literal to object".to_string();
    println!("{}", not_possible_to_use_foo_name);

    // concat string
    let hello = String::from("hello");
    let world = " world";
    let hello_world = hello + world;
    println!("{}", hello_world);

    // slice a string literal
    let hello = "world";
    let hello_slice = &hello[0..1];
    // prints "world"
    println!("{}", hello);
    // prints 'w'
    println!("{}", hello_slice);

    // replace a range
    let mut some_string_object =
        String::from("this is a string literal that will be a string object");
    // changes "this" to "some_string_object"
    some_string_object.replace_range(0..4, "some_string_object");
    println!("{}", some_string_object);

    // loop the chars with for
    for c in some_string_object.chars() {
        println!("{}", c);
    }
    println!("===");
    // loop the chars with iterator
    some_string_object.chars().for_each(|c| {
        println!("{}", c);
    });

    let call = "ola";
    println!("{:?}", call.as_bytes());

    let dash = "-";
    println!("{:?}", dash.as_bytes());
}

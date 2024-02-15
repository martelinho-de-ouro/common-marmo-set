fn main() {
    // string literal ( static by default )
    let abc = "abc"; 
    println!("{}", abc);

    // static string literal 
    let def:&'static str = "def";
    println!("{}", def);

    // string object
    let ghi = String::from("ghi");
    println!("{}", ghi.len());

    // push 
    let mut greeting = String::from("hello");
    greeting.push_str(" world");
    println!("{}", greeting);

    // Convert literal into object
    let not_possible_to_use_foo_name: String = "change this literal to object".to_string();
    println!("{}", not_possible_to_use_foo_name);

}
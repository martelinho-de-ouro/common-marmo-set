fn main() {
    println!("a");
    let a: &str = "a";
    println!("{}", a);
    let b: &str = "b";
    println!("{},{}", a, b);
    println!("{c},{d}", c="c", d="d");
    eprintln!("Error: the error");
    
    print!("1");
    eprint!("2");
    print!("3");
    eprint!("4");
    print!("5");

    
}
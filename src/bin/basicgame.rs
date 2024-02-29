//! This is almost the same code found in the book:
//! `The Rust Programming Language, 2nd Edition`
//!
//! I'm writing this documentation because I found this on
//! <https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html>
//!
//! The code will ask for a number like this:
//! - 'Type the number'
//!
//! And then it will printout something like this:
//! - 'you typed: 1'
//!
use rand::Rng;
use std::cmp::Ordering;
use std::io;

/*
Rust has a set of items defined in the standard lib.
This `set` is called `prelude`. <https://doc.rust-lang.org/std/prelude/index.html>
*/

/* [**] (instance, function, method, type, enum, variant...etc) */

// `fn` to declare a **function
fn main() {
    // `rand::thread_rng is a **function that give a random number from the
    // current thread execution seeded by the OS.
    // The Rng is a **trait that contains a **method `gen_range` that takes
    // a range expression and generates a random number based on
    // the range argument.
    let generated_number = rand::thread_rng().gen_range(1..=2000);

    // println!("the generated number is: {generated_number}");

    loop {
        println!("Type the number");

        /* Variables are immutable by default. We are using `mut` to make
        it mutable. */

        // `String::new` this is a **function that returns a new String **instance.
        let mut the_number = String::new();

        // The `stdin` **function is called from the `io` library/module.
        // The `stdin` **function returns an **instance of the **type `std::io::Stdin`.
        io::stdin()
            // The `read_line` is a **method from the **type `Stdin`
            // (`Stdin` also called a `handle` to the standard input for the terminal).
            // `&` the argument is a reference (copy vs reference thing)
            // `read_line` returns a `Result` value, which is an enum that can be
            // in one of multiple possible **states and each possible state
            // is called `variant`.
            // The purpose of the `Result` is to encode error-handling information.
            // The variants are `Ok` and `Err`. The `Err` contains the info about
            // how or why the thing failed.
            .read_line(&mut the_number)
            // The `Result` type has an `expect` **method that can be called
            .expect("unable to read line");

        // `parse` returns a `Result` type which is an **enum that contains
        // `Ok` and `Err` **variants.
        let the_number: u32 = match the_number.trim().parse() {
            Ok(n) => n,
            // `_` is a catch-all value ( to match all Err values )
            Err(_) => continue,
        };

        println!("you typed: {the_number}");

        // The `cmp` is a **method that can be called on anything that
        // can be compared, and it returns a **variant of the `Ordering` **enum.
        // The `match` is an **expression to decide what to do next based
        // on the **variant returned.
        // The `match` **expression contains **"arms", an **arm consist of a
        // pattern to match against.
        match the_number.cmp(&generated_number) {
            // `Ordering` is an **enum with the `Less`, `Greater` and `Equal` **variants.
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("equal");
                break;
            }
        }
    }
}

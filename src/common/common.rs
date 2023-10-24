//! Common things.

// I already know the `mut` part.
// The diffs between variable and constant
//   * not allowed to use `mut`
//   * it is declared with `const`
//   * the type must be annotated.
//   * const FOO_BAR: u32 = 12345; // this name convention
// Shadowing is diff from `mut` because we get a compile-time error in case try to reassign the variable without the `let` keyword.
// a new variable is created when using `let` and shadowing, the type can be changed but we can keep/reuse the variable name.
// When parsing the types we need to always add the type to the var.
// **Scalar types**
//   a single value: integers, floats, booleans and chars.
//   **Integers**
//     i8    - u8
//     i16   - u16
//     i32   - u32
//     i64   - u64
//     i128  - u128
//     isize - usize [ arch dependant 64bits or 32bits]

//     Note: When compiling in release mode `--release` Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping.
//       -- values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold.
//       -- In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on.

//     To handle the possibility of overflow we can use the methods provided by the std lib:
//       * wrap all mode with wrapping_* methods.
//       * return the value `None` if there is overflow with the `checked_*` methods.
//       * return the value and a boolean indicating whether there was overflow with the `overflowing_*` methods.
//       * saturate the min and max values with `saturating_*` methods.

//   **floats**
//     * f32 and f64 signed.
//   **boolean**
//     * 1 byte size
//     * `bool` keyword
//   **char**
//     * 4 bytes size
//     * `char` keyword

// **Compound types**
//   **tuple**
//     * fixed length
//     * created with a comma-separated list of the values inside parentheses.
//     * each position has a type
//     * don't need to be the same time
//     * destructuring thing like js
//     * tuples without values they call `unit`.
//   **array**
//     * every element have the same type.
//     * fixed length.
//     * we can get runtime errors when trying to access index out of bounds elements.
//     * In other low-level languages, this kind of check is not done, and we provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing

// **Functions**
//   * snake_case
//   * fn keyword
//   * must declare parameter types
//   * we can return early with `return` but most functions returns the last expression implicitly.

// **expressions**
//   * calling a function is an expression.
//   * calling a macro is an expression.
//   * a new scope block created is an expression
//   * Expressions do not include ending semicolons -- if added it will be 1 statement and will nor return a value.

// **loops**
//   * loop returns values
//   * it contains loop-labels
fn main() {
    let mut x: u32 = 5;
    println!("the value of x: {x}");
    x = 9;
    println!("the new value {x}");

    const FOO_BAR: u16 = 65535;
    println!("the const {FOO_BAR}");

    {
        let x = x + 1;
        println!("the value of X inside the scope {x}");
    }

    println!("the value of X outside the scope {x}");

    let foo = "foo";
    let foo = foo.len();
    println!("the foo {foo}");

    // ---------------------------------------
    // compiler error bellow:
    // let mut foo = "foo";
    // foo = foo.len();
    // ---------------------------------------

    // Literals
    let z = 44u8;
    let y: u8 = 255;
    let dec = 12_345;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_0000;
    let byt = b'a';
    println!("literals {z} {y} {dec} {hex} {oct} {bin} {byt}");

    let a = 2.345;
    let b: f32 = 3.1;
    println!("floats {a} {b}");

    let serious: bool = false;
    println!("the bool {serious} ");

    let char: char = 'z';
    println!("the char {char}");

    let tap: (i16, f32, u8) = (255, 1.1, 2);
    let (a, b, c) = tap;
    println!("the tup {a} {b} {c}");

    let zero = tap.0;
    let one = tap.1;
    let two = tap.2;
    println!("single values {zero}");
    println!("single values {one}");
    println!("single values {two}");

    let ray = [4, 3, 7, 9];
    let ray2: [u8; 2] = [1, 2];
    let r1 = ray[0];
    let r2 = ray2[1];
    println!("array {r1} {r2}");

    foo_bar();

    let nine = {
        let k = 8;
        k + 1
    };

    println!("the nine {nine}");

    let bar_foo = bar_foo();
    println!("the bar_foo value {bar_foo}");

    let aaa2 = aaa(1);
    println!("the aaa2 {aaa2}");

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let mut counter = 0;
    let the_result_of_the_loop = loop {
        counter += 1;
        if counter == 300 {
            break counter;
        }
    };
    println!("the result {the_result_of_the_loop}");

    let mut count2 = 0;
    'counting_up: loop {
        println!("count {count2}");
        let mut remaining = 5;

        loop {
            println!("remaining {remaining}");
            if remaining == 4 {
                break;
            }
            if count2 == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count2 += 1;
    }
    println!("end count2 {count2} ");

    let mut other_number = 3;
    while other_number != 0 {
        println!("{other_number}");
        other_number -= 1;
    }

    let things = [10, 20, 30, 40];
    for t in things {
        println!("{t}");
    }
}

fn foo_bar() {
    println!("aaaaa");
}

fn bar_foo() -> i8 {
    9
}

fn aaa(y: i16) -> i16 {
    y + 3
}

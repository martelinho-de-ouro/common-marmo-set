//! ownership
//! A set of rules that governs how a Rust program manages memory.

// Some languages uses GC others we need to free and allocate the memory and Rust uses another approach:
//   * the memory is managed through a system of ownership with a set of rules that the compiler checks.
//   * if the rules are violated the program will not compile.
//   * ownership features will not slow down the program at runtime.
//   * good to think about the stack and heap when programming in Rust.
//   * data with known size are stored in the stack and unknown size at compile time or changeable size must be stored on the heap.
//   * when adding data on the heap a certain amount of space is requested, the mem allocator finds 1 empty space that is big enough, mark it as being in use and returns a pointer.
//   * pushing to the stack is faster because the allocator don't need to search for a place to store the data [the location is the top of the stack].
//   * accessing data on the heap is slower than doing it on stack.
//   * when the code calls a function, the values passed into the function and the function's local vars get pushed on the stack, when the function is over, the values are popped of the stack.
//   * the rules:
//     * each value in Rust has 1 owner
//     * there can only be 1 owner at a time
//     * when the owner goes out of scope, the value will be dropped
//   * In Rust the mem is automatically returned once the variable that owns it goes out of scope.
//     * When a variable goes out of scope, Rust calls a special function called `drop` ( called automatically at the closing curly bracket )
//   * Rust will never create `deep` copies of the data.
//     * In case needed we can use the `clone` method.
//   * If a type implements the `Copy` trait, vars that use it don't move, but rather are copied.
//     * Rust won't let us annotate a type with `Copy` if the type or any of it parts, has implemented the `Drop` trait.
//   * The ownership of a variable follows the same pattern -> assigning a value to another variable moves it.
//     * when a variable that includes data on the heap goes out of scope, the value will be cleaned up by `drop` unless ownership of the data has been moved to another var.
//   * `& reference` and `* dereference` operators
//   * We can change a borrowed value with `&mut` when the variable is also `mut`
//     * restriction: if we have a mutable reference to a value, we can not have it to other references.
//     * `cannot borrow `str` as mutable more than once at a time`
//     * This is to prevent `data races` at compile time.
//     * We can create a new scope with { } allowing multiple-mutable-references
//     * not possible to have a mutable refs + 1 immutable ref
//   * The Rust compiler guarantees that refs will never be dangling refs.
//   * A slice is a kind of ref that does not have ownership.
//     * `string slice` is a ref to part of a `String`
fn main() {
    let _foo: &str = "aaa";
    let mut bar: String = String::from("aaa");
    bar.push_str(",bar_foo");
    println!("{bar}");

    // integers have a known size at compile time
    // so they are stored entirely on the stack then
    // copies of the values are quick.
    // we don't need .clone() here.
    let x: i32 = 1;
    let y: i32 = x;
    println!("x = {x}, y = {y}");

    // stack-data-only
    // Ok, I got the part of the s1 invalidation.
    // let s1 = String::from("111");
    // let s2 = s1;
    // println!("{s1}");
    // -- the clone -- also copy the heap-data not only the stack.
    // ( .clone() may be expensive )
    let s1: String = String::from("111");
    let s2: String = s1.clone();
    println!("{s1} and {s2}");

    let v = String::from("qwerty");
    take_owner(v);
    // the owner of v is now take_owner function.
    // println!({v});

    let w = 3;
    make_copy(w);

    // use the value but not take ownership
    let foo1 = String::from("asdf");
    let (foo2, len) = the_length(foo1);
    println!("{foo2} -> {len}");
    // using reference
    let foo3 = String::from("asdf");
    let len2 = the_length_without_tuple(&foo3);
    println!("{foo3} -> {len2}");

    // the scopes of immutable refs aab and aac end after the println!.
    let mut aa = String::from("zxcvb");
    let aab = &aa;
    let aac = &aa;
    println!("{aab} -> {aac}");
    let aad = &mut aa;
    println!("{aad}");

    let foobarstr = String::from("foo bar");
    let foostr = &foobarstr[0..3];
    let barstr = &foobarstr[4..7];
    let first = &foobarstr[0..1];
    let last = &foobarstr[6..];
    println!("{foostr} {barstr} {first} {last}");
}

fn take_owner(str: String) {
    println!("{str}");
}

fn make_copy(integer: i32) {
    println!("{integer}");
}

fn the_length(str: String) -> (String, usize) {
    let length = str.len();
    (str, length)
}

fn the_length_without_tuple(str: &String) -> usize {
    str.len()
}

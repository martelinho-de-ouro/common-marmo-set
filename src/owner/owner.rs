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
fn main() {
  let _foo = "aaa";
  let mut bar = String::from("aaa");
  bar.push_str(",bar_foo");
  println!("{bar}");

  // stack-data-only
  // Ok, I got the part of the s1 invalidation.
  // let s1 = String::from("111");
  // let s2 = s1;
  // println!("{s1}");
  // -- the clone -- also heap-data
  let s1 = String::from("111");
  let s2 = s1.clone();
  println!("{s1} and {s2}");

}
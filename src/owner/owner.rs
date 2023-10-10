//! ownership
//! A set of rules that govern how a Rust program manages memory.

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
//   * var scope
//    
fn main() {
  let foo = "aaa";
  
}
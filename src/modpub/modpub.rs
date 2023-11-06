// * A package is a bundle of one or more crates (binary or library or both).
// * A package contains a `Cargo.toml`.
// * `src/main.rs` is the crate root for binary crate, by convention.
// *  `src/lib.rs` is the crate root for library crate, by convention. 
// * A package can have multi binary crates by placing files in the src/bin dir.
//   * Each file will be a separate binary crate.
fn main() {
    println!("aaaaa");
}
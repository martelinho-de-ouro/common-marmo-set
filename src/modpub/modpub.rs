// * A package is a bundle of one or more crates (binary or library or both).
// * A package contains a `Cargo.toml`.
// * `src/main.rs` is the crate root for binary crate, by convention.
// *  `src/lib.rs` is the crate root for library crate, by convention. 
// * A package can have multi binary crates by placing files in the src/bin dir.
//   * Each file will be a separate binary crate.
// * The compiler tries to find a module in these places:
//   * Inline, within curly brackets that replace the semicolon following `mod foo`
//   * In the file src/foo.rs
//   * In the file src/foo/mod.rs
//     * In the case of a submodule called `bar`
//       * Inline, directly following mod bar, within curly brackets instead of the semicolon
//       * In the file src/foo/bar.rs
//       * In the file src/foo/bar/mod.rs
// * A BarFoo type inside foo-bar module would be found at
//   * crate::foo::bar::BarFoo
// * `pub` makes the thing (module and items) public, without `pub` the thing is private.

use crate::foo::bar::FooBar;

pub mod foo;

fn main() {
    let foobar = FooBar {};
    println!("result: {:?}", foobar);
}

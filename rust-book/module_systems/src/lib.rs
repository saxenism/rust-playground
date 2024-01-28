// If lib.rs is defined in the root of your source directory,
// then rust will automatically create a library crate with the same
// name as your package and this lib.rs would be the crate root.

// A package must have atleast 1 crate (default (binary) crate is main.rs in the root folder)
// A package can have either 0 or 1 library crates.
// A package can have any number of binary crates.
// If we want more binary crates (apart from main.rs), create a sub-directory called `bin`.
//  And each file in this folder represents a separate binary crate.
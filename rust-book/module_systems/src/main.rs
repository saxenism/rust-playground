// This chapter teaches managing your own projects
// using: Packages, crates and modules

// A Package can consist of crates.
// A crate can be a binary crate, that can be executed => something.exe
// Or it can be a library create, which is code that can be used by other programs => OpenZeppelin libraries

// Crates contain modules.
// Modules allow you to organise chunks of code & maintain privacy rules

// Example: 
// 1. A website package in Rust
// 2. Package has a login library crate
// 3. Crate has an authentication module
// 4. You can now make the code inside of your authentication module private but expose 1 login function publically

// There is also a concept of workspaces, which allows you to store inter-related packages inside the workspace.

fn main() {
    println!("Hello, world!");
}

//! Hello world example for Rust
//! This is a crate description, needed or else missing_docs warning will occur

/// Prints "Hello, world!"
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of a person or entity
///
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    hello("Arrow Contributor");
}

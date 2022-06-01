pub mod bump;
pub mod config;
pub mod core;
pub mod file;
pub mod next;
pub mod validate;

#[cfg(test)]
pub mod tests;

pub fn hello() {
    println!("Hello from semver!");
}

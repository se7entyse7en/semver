pub mod cmd;
pub mod config;
pub mod core;
pub mod file;

#[cfg(test)]
pub mod tests;

pub fn hello() {
    println!("Hello from semver!");
}

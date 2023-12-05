use std::env;

fn main() {
    // Iterate over all environment variables
    for (key, value) in env::vars() {
        // Use 'cargo:warning' to print messages during the build process
        println!("cargo:warning={}={}", key, value);
    }
}

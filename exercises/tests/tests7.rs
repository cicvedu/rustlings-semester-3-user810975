// tests7.rs
//
// When building packages, some dependencies can neither be imported in
// `Cargo.toml` nor be directly linked; some preprocesses varies from code
// generation to set-up package-specific configurations.
//
// Cargo does not aim to replace other build tools, but it does integrate
// with them with custom build scripts called `build.rs`. This file is
// usually placed in the root of the project, while in this case the same
// directory of this exercise.
//
// It can be used to:
//
// - Building a bundled C library.
// - Finding a C library on the host system.
// - Generating a Rust module from a specification.
// - Performing any platform-specific configuration needed for the crate.
//
// When setting up configurations, we can `println!` in the build script
// to tell Cargo to follow some instructions. The generic format is:
//
//     println!("cargo:{}", your_command_in_string);
//
// Please see the official Cargo book about build scripts for more
// information:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// In this exercise, we look for an environment variable and expect it to
// fall in a range. You can look into the testcase to find out the details.
//
// You should NOT modify this file. Modify `build.rs` in the same directory
// to pass this exercise.
//
// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.

// tests7.rs 
fn main() {}

#[cfg(test)]  
mod tests {  
    use super::*;  
    use std::fs::File;  
    use std::io::Read;  
  
    #[test]  
    fn test_success() {  
        // Read the timestamp from the file set by build.rs  
        let timestamp_file = std::env::var("TEST_FOO_FILE").unwrap();  
        let mut file = File::open(&timestamp_file).unwrap();  
        let mut contents = String::new();  
        file.read_to_string(&mut contents).unwrap();  
        let e: u64 = contents.trim().parse().unwrap();  
  
        let timestamp = std::time::SystemTime::now()  
            .duration_since(std::time::UNIX_EPOCH)  
            .unwrap()  
            .as_secs();  
  
        assert!(timestamp >= e && timestamp < e + 10);  
    }  
}
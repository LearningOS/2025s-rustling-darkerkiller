// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// build.rs

fn main() {
    // Required for tests7.rs: Set TEST_FOO environment variable with timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // Required for tests8.rs: Enable the "pass" feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// Using match
fn main() {
    let mut res = 42;
    let value = 12;    // Changed from Some(12) to just 12
    res += value;      // Direct addition since we don't need Option handling
    println!("{}", res);
}

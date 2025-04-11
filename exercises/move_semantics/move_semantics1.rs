// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

// move_semantics1.rs
//
// Alternative implementation creating vector inside the function

fn fill_vec() -> Vec<i32> {
    // Create and initialize vector directly
    let mut vec = vec![22, 44, 66];
    vec
}

fn main() {
    // Get filled vector from function
    let mut vec1 = fill_vec();

    // Print vec1's length and content
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // Add one more element
    vec1.push(88);

    // Print vec1's length and content again
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}    
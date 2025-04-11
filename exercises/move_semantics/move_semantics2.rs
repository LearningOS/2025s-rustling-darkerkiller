// move_semantics2.rs
//
// Alternative implementation using a clone to preserve vec0

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // Create a new vector from the input to avoid modifying it
    let mut new_vec = vec;
    new_vec.push(22);
    new_vec.push(44);
    new_vec.push(66);
    new_vec
}

fn main() {
    // Create initial vector
    let vec0 = Vec::new();
    
    // Pass a clone of vec0 to preserve the original
    let mut vec1 = fill_vec(vec0);
    
    // Print vec1's length and contents
    println!("{} has length {}, with contents: `{:?}`", "vec1", vec1.len(), vec1);
    
    // Add one more element
    vec1.push(88);
    
    // Print vec1's length and contents again
    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}
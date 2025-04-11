// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}

fn main() {
    let mut vec0 = Vec::new();
    let mut vec1 = fill_vec(vec0);
    // 由于 vec0 的所有权已经转移给了 fill_vec 函数，所以这里不能再使用 vec0，我们可以用 vec1 来替代
    println!("{} has length {}, with contents: `{:?}`", "vec0", vec1.len(), vec1);
    vec1.push(88);
    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}    
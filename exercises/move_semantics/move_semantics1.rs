// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

// 此函数用于向传入的向量中添加元素并返回该向量
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    // 向向量中添加元素
    vec.push(22);
    vec.push(44);
    vec.push(66);

    // 返回填充后的向量
    vec
}

fn main() {
    // 创建一个空的整数向量
    let vec0 = Vec::new();

    // 调用 fill_vec 函数填充向量，并将结果赋值给 vec1
    let mut vec1 = fill_vec(vec0);

    // 打印 vec1 的长度和内容
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    // 向 vec1 中再添加一个元素
    vec1.push(88);

    // 再次打印 vec1 的长度和内容
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}    
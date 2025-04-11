// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


// primitive_types3_optimized.rs
// 此程序创建一个至少包含 100 个元素的数组，并根据数组长度输出不同的信息。
// 代码优化了变量命名，增加了更多的长度判断逻辑。

fn main() {
    // 创建一个包含 1000 个元素的数组，每个元素的值为 0
    let large_array = [0; 1000];

    // 获取数组的长度
    let array_length = large_array.len();

    // 根据数组长度输出不同的信息
    if array_length < 100 {
        println!("这个数组有点小，我轻松搞定。");
    } else if array_length < 500 {
        println!("这个数组有一定规模，但对我来说不算难。");
    } else if array_length < 1000 {
        println!("哇，这个数组挺大的，处理起来有点挑战。");
    } else {
        println!("天呐，这是一个超级大的数组，我要全力以赴了！");
    }
}    

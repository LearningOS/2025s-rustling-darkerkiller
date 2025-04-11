// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.


// 程序入口函数
fn main() {
    // 调用 call_me 函数，传入参数 3
    call_me(3);
}

// 定义 call_me 函数，接收一个 u32 类型的参数 num
// 该函数会循环 num 次，每次打印一条提示信息
fn call_me(num: u32) {
    // 使用 for 循环从 0 到 num - 1 进行迭代
    for i in 0..num {
        // 打印提示信息，显示当前是第 i + 1 次响铃
        println!("Ring! Call number {}", i + 1);
    }
}    

// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.


// 定义 main 函数，作为程序的入口点
fn main() {
    // 调用 call_me 函数，并传入参数 3
    call_me(3);
}

// 定义 call_me 函数，接收一个 i32 类型的参数 num
fn call_me(num: i32) {
    // 使用 for 循环，循环次数为 num 次
    for i in 0..num {
        // 打印提示信息，模拟电话响铃，并显示当前是第几次响铃
        println!("Ring! Call number {}", i + 1);
    }
}    

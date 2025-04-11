// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.

#[rustfmt::skip]
macro_rules! my_macro {
    // 无参数的匹配规则
    () => {
        println!("Check out my macro!");
    };
    // 带一个表达式参数的匹配规则
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    // 调用无参数的宏规则
    my_macro!();
    // 调用带参数的宏规则
    my_macro!(7777);
}    
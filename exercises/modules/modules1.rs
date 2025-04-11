// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE
mod sausage_factory {
    // 明确声明为私有函数，外部模块无法访问
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 声明为公共函数，外部模块可以访问
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}    
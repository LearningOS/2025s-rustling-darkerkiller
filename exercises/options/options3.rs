// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    if let Some(p) = y {
        println!("Co-ordinates are {},{} ", p.x, p.y);
    } else {
        println!("No valid point provided.");
    }
    // 若想保留 y 变量后续使用，可在这里添加相应逻辑
    let _ = y; 
}
    
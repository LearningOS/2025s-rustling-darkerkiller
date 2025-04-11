// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let mut x = 100;              // Line 1
    {                             // Line 2
        let y = &mut x;           // Line 3
        *y += 100;                // Line 4
    }                             // Line 5
    assert_eq!(x, 200);           // Line 6
    let z = &mut x;               // Line 7
    *z += 1000;                   // Line 8
    assert_eq!(x, 1200);          // Line 9
}                
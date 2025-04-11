// functions4.rs
//
// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off. (Don't worry
// about the function bodies themselves, we're only interested in the signatures
// for now. If anything, this is a good way to peek ahead to future exercises!)
//
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(!is_even(3));
    }

    #[test]
    fn test_sale_price_even() {
        let original_price = 20;
        let expected_price = original_price - 10;
        assert_eq!(sale_price(original_price), expected_price);
    }

    #[test]
    fn test_sale_price_odd() {
        let original_price = 21;
        let expected_price = original_price - 3;
        assert_eq!(sale_price(original_price), expected_price);
    }
}

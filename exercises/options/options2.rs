// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        match optional_target {
            Some(word) => assert_eq!(word, target),
            None => unreachable!(),
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = (1..=range).map(Some).collect();
        optional_integers.insert(0, None);

        let mut cursor = range;
        loop {
            match optional_integers.pop() {
                Some(Some(integer)) => {
                    assert_eq!(integer, cursor);
                    cursor -= 1;
                }
                Some(None) => break,
                None => unreachable!(),
            }
        }

        assert_eq!(cursor, 0);
    }
}    
    
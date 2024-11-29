#[macro_export]
macro_rules! string {
    ($lit:literal) => {
        String::from($lit)
    };
}

mod tests {
    #[test]
    fn test_string() {
        assert_eq!(string!("hello"), "hello".to_owned());
    }
}

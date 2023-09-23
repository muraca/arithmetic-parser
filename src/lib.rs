pub fn parse(_string: String) -> i32 {
    0
}

#[test]
fn given_tests() {
    assert_eq!(parse("3a2c4".to_string()), 20);
    assert_eq!(parse("32a2d2".to_string()), 17);
    assert_eq!(parse("500a10b66c32".to_string()), 14208);
    assert_eq!(parse("3ae4c66fb32".to_string()), 235);
    assert_eq!(parse("3c4d2aee2a4c41fc4f".to_string()), 990);
}

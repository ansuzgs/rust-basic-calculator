use calc_basic::calculator::calculate;

#[test]
fn test_simple_addition() {
    assert_eq!(calculate("3 + 5").unwrap(), 8);
    assert_eq!(calculate("10 + 20").unwrap(), 30);
}

#[test]
fn test_invalid_number() {
    assert!(calculate("abc + 5").is_err());
    assert!(calculate("3 + xyz").is_err());
}

#[test]
fn test_invalid_operator() {
    assert!(calculate("3 & 5").is_err());
    assert!(calculate("4 * 2").is_err());
}

#[test]
fn test_incorrect_token_count() {
    assert!(calculate("3 +").is_err());
    assert!(calculate("3 + 5 + 2").is_err());
    assert!(calculate("+ 3 5").is_err());
}

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
    assert!(calculate("4 % 2").is_err());
}

#[test]
fn test_incorrect_token_count() {
    assert!(calculate("3 +").is_err());
    assert!(calculate("3 + 5 + 2").is_err());
    assert!(calculate("+ 3 5").is_err());
}

#[test]
fn test_simple_subtraction() {
    assert_eq!(calculate("10 - 3").unwrap(), 7);
    assert_eq!(calculate("5 - 2").unwrap(), 3);
}

#[test]
fn test_simple_multiplication() {
    assert_eq!(calculate("3 * 4").unwrap(), 12);
    assert_eq!(calculate("7 * 6").unwrap(), 42);
}

#[test]
fn test_simple_division() {
    assert_eq!(calculate("8 / 2").unwrap(), 4);
    assert_eq!(calculate("20 / 5").unwrap(), 4);
}

#[test]
fn test_division_by_zero() {
    assert!(calculate("10 / 0").is_err());
}

#[test]
fn test_no_spaces_addition() {
    assert_eq!(calculate("3+5").unwrap(), 8);
    assert_eq!(calculate("10+20").unwrap(), 30);
}

#[test]
fn test_no_spaces_subtraction() {
    assert_eq!(calculate("10-3").unwrap(), 7);
    assert_eq!(calculate("5-2").unwrap(), 3);
}

#[test]
fn test_no_spaces_multiplication() {
    assert_eq!(calculate("3*4").unwrap(), 12);
    assert_eq!(calculate("7*6").unwrap(), 42);
}

#[test]
fn test_no_spaces_division() {
    assert_eq!(calculate("8/2").unwrap(), 4);
    assert_eq!(calculate("20/5").unwrap(), 4);
}

#[test]
fn test_mixed_spaces_and_no_spaces() {
    assert_eq!(calculate("3+ 5").unwrap(), 8);
    assert_eq!(calculate("10 -3").unwrap(), 7);
    assert_eq!(calculate("4* 5").unwrap(), 20);
    assert_eq!(calculate("8 /2").unwrap(), 4);
}

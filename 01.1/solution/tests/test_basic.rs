extern crate solution;
use solution::*;
use std::str::FromStr;

#[test]
fn test_basic() {
    assert_eq!(CodeStyle::from_str(" some_var ").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str("some_var").unwrap(), CodeStyle::Underscored);
}

#[test]
fn test_basic_2() {
    assert_eq!("some_var".parse::<CodeStyle>().unwrap(), CodeStyle::Underscored);
}

#[test]
fn test_validity() {
    assert_eq!(CodeStyle::from_str("").unwrap_err(), CodeStyleError::EmptyString);
    assert_eq!(CodeStyle::from_str("  ").unwrap_err(), CodeStyleError::EmptyString);
    assert_eq!(CodeStyle::from_str("some-var").unwrap_err(), CodeStyleError::InvalidChar('-'));
    assert_eq!(CodeStyle::from_str("some var").unwrap_err(), CodeStyleError::InvalidChar(' '));
    assert_eq!(CodeStyle::from_str("+_+").unwrap_err(), CodeStyleError::InvalidFirstChar('+'));
    assert_eq!(CodeStyle::from_str("❤️_❤️").unwrap_err(), CodeStyleError::InvalidFirstChar("❤️".chars().next().unwrap()));
    assert_eq!(CodeStyle::from_str("123abc").unwrap_err(), CodeStyleError::InvalidFirstChar('1'));
    assert_eq!(CodeStyle::from_str("a+b=c").unwrap_err(), CodeStyleError::InvalidChar('+'));

    assert_eq!(CodeStyle::from_str("abc123").unwrap(), CodeStyle::Unknown);
    assert_eq!(CodeStyle::from_str("some_var").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str(" some_var ").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str("someVar").unwrap(), CodeStyle::Camelcased);
    assert_eq!(CodeStyle::from_str("SomeVar").unwrap(), CodeStyle::Camelcased);
    assert_eq!(CodeStyle::from_str("Some_Var_").unwrap(), CodeStyle::Mixed);
    assert_eq!(CodeStyle::from_str("Some_Var").unwrap(), CodeStyle::Mixed);
    assert_eq!(CodeStyle::from_str("someVar_").unwrap(), CodeStyle::Mixed);
}

#[test]
fn test_screaming_snakecase_basic() {
    assert_eq!(CodeStyle::from_str("SOME_VAR").unwrap(), CodeStyle::ScreamingSnakecased);
    assert_eq!(CodeStyle::from_str("SOME__VAR____").unwrap(), CodeStyle::ScreamingSnakecased);
}

#[test]
fn test_cyrillic1() {
    assert_eq!(CodeStyle::from_str("що_стана").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str("що___стана").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str("ЩО_СТАНА").unwrap(), CodeStyle::ScreamingSnakecased);
    assert_eq!(CodeStyle::from_str("ЩО___СТАНА").unwrap(), CodeStyle::ScreamingSnakecased);
    assert_eq!(CodeStyle::from_str("щоСтана").unwrap(), CodeStyle::Camelcased);
    assert_eq!(CodeStyle::from_str("ЩоСтана").unwrap(), CodeStyle::Camelcased);
}

#[test]
fn test_multibyte_uppercase() {
    assert_eq!(CodeStyle::from_str("someSSpecialCase").unwrap(), CodeStyle::Camelcased);
    assert_eq!(CodeStyle::from_str("SOME_SSPECIAL_CASE").unwrap(), CodeStyle::ScreamingSnakecased);
    assert_eq!(CodeStyle::from_str("SOME___SSPECIAL___CASE").unwrap(), CodeStyle::ScreamingSnakecased);
}

#[test]
fn test_whitespace() {
    assert_eq!(CodeStyle::from_str("  some_var\n").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str("\tone_two_three  ").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str("  some___var\n").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str("\tone__two_three__  ").unwrap(), CodeStyle::Underscored);
}

#[test]
fn test_digits1() {
    assert_eq!(CodeStyle::from_str("someVar123").unwrap(), CodeStyle::Camelcased);
    assert_eq!(CodeStyle::from_str("some_var_123").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str("some123Var").unwrap(), CodeStyle::Camelcased);
    assert_eq!(CodeStyle::from_str("some_123_var").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str("some__var__123_").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str("some_123__var__").unwrap(), CodeStyle::Underscored);
}

#[test]
fn test_digits2() {
    assert_eq!(CodeStyle::from_str("some12Var").unwrap(), CodeStyle::Camelcased);
    assert_eq!(CodeStyle::from_str("some_1_2_var").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str("because789").unwrap(), CodeStyle::Camelcased);
    assert_eq!(CodeStyle::from_str("because_7_8_9").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str("because_7_8_9____").unwrap(), CodeStyle::Underscored);
}

#[test]
fn test_digits3() {
    assert_eq!(CodeStyle::from_str("some٣Var").unwrap(), CodeStyle::Camelcased);
    assert_eq!(CodeStyle::from_str("some_٣_var").unwrap(), CodeStyle::Underscored);
    assert_eq!(CodeStyle::from_str("some_٣__var__").unwrap(), CodeStyle::Underscored);
}

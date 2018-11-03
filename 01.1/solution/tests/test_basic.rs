// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::*;

#[test]
fn test_basic() {
    let code_style: CodeStyle = CodeStyle::from_str(" some_var ");
    assert_eq!(code_style, CodeStyle::Underscored);

    let code_style: CodeStyle = CodeStyle::from_str("some_var");
    assert_eq!(code_style, CodeStyle::Underscored);
}

#[test]
fn test_basic_2() {
    let code_style: CodeStyle = "some_var".parse::<CodeStyle>();
    assert_eq!(code_style, CodeStyle::Underscored);
}
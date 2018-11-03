// Бележка: името на проекта трябва да се казва "solution". Ако не се казва така, променете го на
// тези два реда:
extern crate solution;
use solution::*;

#[test]
fn test_basic() {
    let code_identifier = CodeIdentifier::new("some_var").unwrap();
    assert_eq!(code_identifier.camelcase(), "someVar");
}

#[test]
fn test_basic_2() {
    let code_identifier = CodeIdentifier::new("some_var").unwrap();
    assert_eq!(code_identifier.titlecase(), "SomeVar");
}

#[test]
fn test_basic_3() {
    let code_identifier = CodeIdentifier::new("some_var").unwrap();
    assert_eq!(code_identifier.underscore(), "some_var");
}

#[test]
fn test_basic_4() {
    let code_identifier = CodeIdentifier::new("some_var").unwrap();
    assert_eq!(code_identifier.kebabcase(), "some-var");
}

#[test]
fn test_basic_5() {
    let code_identifier = CodeIdentifier::new("some_var").unwrap();
    assert_eq!(code_identifier.screaming_snakecase(), "SOME_VAR");
}

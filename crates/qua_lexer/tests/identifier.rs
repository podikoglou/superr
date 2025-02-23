#[macro_use]
mod common;

use qua_lexer::token::Token;

#[test]
fn test_identifier_alphabetic() {
    assert_identifier_correct_lexing_eq!("qua");
    assert_identifier_correct_lexing_eq!("Qua");
    assert_identifier_correct_lexing_eq!("x");
}

#[test]
fn test_identifier_alphanumeric() {
    assert_identifier_correct_lexing_eq!("qua0");
    assert_identifier_correct_lexing_eq!("qua3");
    assert_identifier_correct_lexing_eq!("qua999999999999");
}

#[test]
fn test_identifier_underscore() {
    assert_identifier_correct_lexing_eq!("x_");
    assert_identifier_correct_lexing_eq!("_x");
    assert_identifier_correct_lexing_eq!("____________________Qua_");
}

#[test]
fn test_identifier_invalid_start_digit() {
    assert_identifier_correct_lexing_ne!("0qua");
    assert_identifier_correct_lexing_ne!("3qua");
    assert_identifier_correct_lexing_ne!("999999999999qua");
}

#[test]
fn test_identifier_invalid_chars() {
    assert_identifier_correct_lexing_ne!("____________________Qua!_");
    assert_identifier_correct_lexing_ne!("!Qua__");
    assert_identifier_correct_lexing_ne!("*qua");
}

#[test]
fn test_identifier_valid_names() {
    assert_identifier_correct_lexing_eq!("print");
    assert_identifier_correct_lexing_eq!("range");
}

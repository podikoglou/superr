#[macro_use]
mod common;

use qua_lexer::token::Token;

#[test]
fn test_unicode_char_literal_ascii_uppercase_a() {
    assert_tokens_eq!("'\\u{0041}'", vec![Token::CharLiteral('A'), Token::EOF]);
}

#[test]
fn test_unicode_char_literal_ascii_uppercase_b() {
    assert_tokens_eq!("'\\u{0042}'", vec![Token::CharLiteral('B'), Token::EOF]);
}

#[test]
fn test_unicode_char_literal_ascii_uppercase_c() {
    assert_tokens_eq!("'\\u{0043}'", vec![Token::CharLiteral('C'), Token::EOF]);
}

#[test]
fn test_unicode_char_literal_ascii_uppercase_d() {
    assert_tokens_eq!("'\\u{0044}'", vec![Token::CharLiteral('D'), Token::EOF]);
}

#[test]
fn test_unicode_char_literal_ascii_uppercase_e() {
    assert_tokens_eq!("'\\u{0045}'", vec![Token::CharLiteral('E'), Token::EOF]);
}

#[test]
fn test_unicode_char_literal_ascii_uppercase_j() {
    assert_tokens_eq!("'\\u{004a}'", vec![Token::CharLiteral('J'), Token::EOF]);
}

#[test]
fn test_unicode_char_literal_cyrillic_capital_ka() {
    assert_tokens_eq!("'\\u{041a}'", vec![Token::CharLiteral('К'), Token::EOF]);
}

#[test]
fn test_unicode_char_literal_hryvnia_sign() {
    assert_tokens_eq!("'\\u{20B4}'", vec![Token::CharLiteral('₴'), Token::EOF]);
}

#[test]
fn test_unicode_char_literal_grinning_face() {
    assert_tokens_eq!("'\\u{1F600}'", vec![Token::CharLiteral('😀'), Token::EOF]);
}

#[test]
fn test_unicode_char_literal_max_unicode() {
    assert_unicode_char_literal_properly_decoded!("'\\u{10FFFF}'", 0x10FFFF);
}

#[test]
fn test_unicode_char_literal_zero() {
    assert_unicode_char_literal_properly_decoded!("'\\u{0}'", 0x0);
}

#[test]
fn test_unicode_char_literal_hex_f() {
    assert_unicode_char_literal_properly_decoded!("'\\u{F}'", 0xF);
}

#[test]
fn test_unicode_char_literal_hex_ff() {
    assert_unicode_char_literal_properly_decoded!("'\\u{FF}'", 0xFF);
}

#[test]
fn test_unicode_char_literal_hex_fff() {
    assert_unicode_char_literal_properly_decoded!("'\\u{FFF}'", 0xFFF);
}

#[test]
fn test_unicode_char_literal_hex_ffff() {
    assert_unicode_char_literal_properly_decoded!("'\\u{FFFF}'", 0xFFFF);
}

#[test]
fn test_unicode_char_literal_hex_fffff() {
    assert_unicode_char_literal_properly_decoded!("'\\u{FFFFF}'", 0xFFFFF);
}

#[test]
fn test_unicode_char_literal_hex_10ffff() {
    assert_unicode_char_literal_properly_decoded!("'\\u{10FFFF}'", 0x10FFFF);
}

#[test]
fn test_unicode_char_literal_hex_000000() {
    assert_unicode_char_literal_properly_decoded!("'\\u{000000}'", 0x000000);
}

#[test]
fn test_unicode_char_literal_hex_00000f() {
    assert_unicode_char_literal_properly_decoded!("'\\u{00000F}'", 0x00000F);
}

#[test]
fn test_unicode_char_literal_hex_0000ff() {
    assert_unicode_char_literal_properly_decoded!("'\\u{0000FF}'", 0x0000FF);
}

#[test]
fn test_unicode_char_literal_hex_000fff() {
    assert_unicode_char_literal_properly_decoded!("'\\u{000FFF}'", 0x000FFF);
}

#[test]
fn test_unicode_char_literal_hex_00ffff() {
    assert_unicode_char_literal_properly_decoded!("'\\u{00FFFF}'", 0x00FFFF);
}

#[test]
fn test_unicode_char_literal_hex_0fffff() {
    assert_unicode_char_literal_properly_decoded!("'\\u{0FFFFF}'", 0x0FFFFF);
}

#[macro_use]
mod common;

#[test]
fn string_literals() {
    assert_eq!(lit!(string "").to_string(), "\"\"");
    assert_eq!(lit!(string "hello").to_string(), "\"hello\"");
    assert_eq!(lit!(string "hello world").to_string(), "\"hello world\"");
    assert_eq!(lit!(string "Hello\nWorld").to_string(), "\"Hello\nWorld\"");
    assert_eq!(lit!(string "Quote: \"").to_string(), "\"Quote: \"\"");
    assert_eq!(
        lit!(string "Backslash: \\").to_string(),
        "\"Backslash: \\\""
    );
    assert_eq!(lit!(string "🦀").to_string(), "\"🦀\"");
}

#[test]
fn char_literals() {
    assert_eq!(lit!(char 'c').to_string(), "'c'");
    assert_eq!(lit!(char '\n').to_string(), "'\n'");
    assert_eq!(lit!(char '\t').to_string(), "'\t'");
    assert_eq!(lit!(char '\'').to_string(), "'\''");
    assert_eq!(lit!(char '"').to_string(), "'\"'");
    assert_eq!(lit!(char '\\').to_string(), "'\\'");
    assert_eq!(lit!(char '🦀').to_string(), "'🦀'");
}

#[test]
fn numeric_literals() {
    assert_eq!(lit!(float 3.14).to_string(), "3.14");
    assert_eq!(lit!(int 0).to_string(), "0");
    assert_eq!(lit!(int 42).to_string(), "42");
}

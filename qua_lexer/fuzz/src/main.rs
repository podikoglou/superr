use afl::fuzz;
use qua_lexer::lex;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let _ = lex(s.to_string());
        }
    });
}

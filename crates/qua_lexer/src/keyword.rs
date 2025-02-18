use phf::phf_map;

#[derive(PartialEq, Debug, Clone)]
pub enum Keyword {
    If,
    Else,
    For,
    While,
    Return,
    Break,
    Continue,
    In,
}

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "if" => Keyword::If,
    "else" => Keyword::Else,
    "for" => Keyword::For,
    "while" => Keyword::While,
    "return" => Keyword::Return,
    "break" => Keyword::Break,
    "continue" => Keyword::Continue,
    "in" => Keyword::In,
};

pub fn lookup(ident: &str) -> Option<Keyword> {
    KEYWORDS.get(ident).cloned()
}

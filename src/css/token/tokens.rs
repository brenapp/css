// Possible CSS tokens
#[derive(Debug, Clone)]
pub enum CSSToken {
    Comment,
    Ident(String),
    Function(String),
    AtKeyword(String),
    Hash(String, HashFlag),
    String(String),
    BadString,
    URL(String),
    BadURL,
    Delim(char),
    Number(f64, NumericFlag),
    Percentage(f64),
    Dimension(f64, NumericFlag, String),
    Whitespace,
    CDO,
    CDC,
    Colon,
    Semicolon,
    Comma,
    LeftBracket,
    RightBracket,
    LeftParentheses,
    RightParentheses,
    LeftBrace,
    RightBrace,
    EOF,
}

// Token Flags
#[derive(Debug, Clone)]
pub enum HashFlag {
    Id,
    Unrestricted,
}

#[derive(Debug, Clone)]
pub enum NumericFlag {
    Integer,
    Number,
}

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
    Number(i32, NumericFlag),
    Percentage(i32),
    Dimension(i32, NumericFlag, String),
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

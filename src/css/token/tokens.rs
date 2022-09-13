use std::fmt::Display;

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

fn format_number(number: &f64, flag: &NumericFlag) -> String {
    match flag {
        NumericFlag::Integer => format!("{}", *number as usize),
        NumericFlag::Number => format!("{}", number),
    }
}

impl Display for CSSToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CSSToken::Comment => write!(f, ""),
            CSSToken::Ident(name) => write!(f, "{}", name),
            CSSToken::Function(name) => write!(f, "{}(", name),
            CSSToken::AtKeyword(keyword) => write!(f, "@{}", keyword),
            CSSToken::Hash(ident, _flag) => write!(f, "#{}", ident),
            CSSToken::String(name) => write!(f, "{}", name),
            CSSToken::BadString => write!(f, ""),
            CSSToken::URL(url) => write!(f, "url({})", url),
            CSSToken::BadURL => write!(f, ""),
            CSSToken::Delim(ch) => write!(f, "{}", ch),
            CSSToken::Number(value, flag) => write!(f, "{}", format_number(value, flag)),
            CSSToken::Percentage(value) => write!(f, "{}%", value),
            CSSToken::Dimension(amount, flag, unit) => {
                write!(f, "{}{}", format_number(amount, flag), unit)
            }
            CSSToken::Whitespace => write!(f, " "),
            CSSToken::CDO => write!(f, ""),
            CSSToken::CDC => write!(f, ""),
            CSSToken::Colon => write!(f, ":"),
            CSSToken::Semicolon => write!(f, ";"),
            CSSToken::Comma => write!(f, ","),
            CSSToken::LeftBracket => write!(f, "["),
            CSSToken::RightBracket => write!(f, "]"),
            CSSToken::LeftParentheses => write!(f, "("),
            CSSToken::RightParentheses => write!(f, ")"),
            CSSToken::LeftBrace => write!(f, "{{"),
            CSSToken::RightBrace => write!(f, "}}"),
            CSSToken::EOF => write!(f, ""),
        }
    }
}

pub mod token;
use std::string::String;

pub fn minimize(contents: String) -> Result<String, token::error::ParseError> {
    let tokens = match token::tokenize(contents) {
        Ok(tokens) => tokens,
        Err(e) => return Err(e),
    };

    let string = tokens
        .iter()
        .map(|token| token.to_string())
        .collect::<Vec<String>>()
        .join("");

    Ok(string)
}
